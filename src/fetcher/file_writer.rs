use super::difficulty::Difficulty;
use super::question::Question;

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
const LEETCODE_QUESTION_URL: &str = "https://leetcode.com/problems/";
const LOCAL_FOLDER: &str = "src/rust_leetcode";
const FOLDER_EASY: &str = "src/rust_leetcode/easy";
const FOLDER_MEDIUM: &str = "src/rust_leetcode/medium";
const FOLDER_HARD: &str = "src/rust_leetcode/hard";

const STRUCT_SOLUTION: &str = "pub struct Solution {}";
const FUNCTION_DEF: &str = "pub fn ";

const README: &str = "README.md";
const README_CHAPTER_HEADER: &str = "###";
const README_COL_SEP: char = '|';
const README_OVERVIEW: &str = "## Overview";

const MOD_EASY: &str = "src/rust_leetcode/easy/mod.rs";
const MOD_MEDIUM: &str = "src/rust_leetcode/medium/mod.rs";
const MOD_HARD: &str = "src/rust_leetcode/hard/mod.rs";

pub fn create_code_file(question: &Question) {
    let path = get_path(question);
    if path.exists() {
        println!(
            "Question {} already exists at {}",
            question.title,
            path.to_str().expect("cannot convert to &str")
        );
        return;
    }
    fs::write(get_path(question), get_contents(question)).expect("cannot write to file");
    update_mod_rs(question);
    update_readme_md(question);
}

fn format_readme_line(question: &Question) -> String {
    format!(
        "| {:0>4} | [{}]({}) | {} |",
        question.id,
        question.title,
        format_url(question),
        question.topic_tags.0.join(", ")
    )
}

fn insert_at_line(buffer: &str, idx: Option<usize>, line: &str) -> String {
    let mut new_buffer = String::with_capacity(buffer.len() + line.len() + 1);
    if let Some(i) = idx {
        for top in buffer.lines().take(i) {
            new_buffer.push_str(top);
            new_buffer.push('\n');
        }
        new_buffer.push_str(line);
        new_buffer.push('\n');
        for bottom in buffer.lines().skip(i) {
            new_buffer.push_str(bottom);
            new_buffer.push('\n');
        }
        new_buffer
    } else {
        new_buffer.push_str(buffer);
        new_buffer.push_str(line);
        new_buffer.push('\n');
        new_buffer
    }
}

fn update_readme_md(question: &Question) {
    let contents = fs::read_to_string(README).expect("cannot read from README");
    let with_counter = replace_readme_counter(&contents);
    let idx = get_readme_line(question, &with_counter);
    let with_table = insert_at_line(&with_counter, idx, &format_readme_line(question));
    fs::write(README, with_table).expect("cannot write to README");
}

fn replace_readme_counter(contents: &str) -> String {
    let mut new_buffer = String::with_capacity(contents.len() + 1);
    let mut idx = 0;
    for (i, line) in contents.lines().enumerate() {
        if line.trim().starts_with(README_OVERVIEW) {
            idx = i;
            break;
        }
        new_buffer.push_str(line);
        new_buffer.push('\n');
    }
    let new_counter = format_readme_overview();
    new_buffer.push_str(&new_counter);
    new_buffer.push('\n');
    for line in contents.lines().skip(idx + new_counter.lines().count()) {
        new_buffer.push_str(line);
        new_buffer.push('\n');
    }
    new_buffer
}

fn count_files_in_folder(path: &Path, ext: Option<&str>) -> usize {
    let folder = fs::read_dir(path).expect("cannot read folder");
    if let Some(e) = ext {
        folder
            .filter(|p| {
                p.as_ref()
                    .expect("cannot read path")
                    .path()
                    .extension()
                    .map(|ext| ext == e)
                    .unwrap_or(false)
            })
            .count()
    } else {
        folder.count()
    }
}

fn format_readme_overview() -> String {
    let ext = "rs";
    // Exclude mod.rs.
    let n_easy = count_files_in_folder(Path::new(FOLDER_EASY), Some(ext)) - 1;
    let n_medium = count_files_in_folder(Path::new(FOLDER_MEDIUM), Some(ext)) - 1;
    let n_hard = count_files_in_folder(Path::new(FOLDER_HARD), Some(ext)) - 1;
    format!(
        "## Overview

|     | Difficulty | Solved |
| --- | ---------- | ------ |
| 🟢  | Easy       | {}     |
| 🟡  | Medium     | {}     |
| 🔴  | Hard       | {}      |
|     | **Total**  | **{}** |",
        n_easy,
        n_medium,
        n_hard,
        n_easy + n_medium + n_hard
    )
}

fn get_readme_line(question: &Question, contents: &str) -> Option<usize> {
    let line_idx = None;
    let mut header_found = false;
    let mut data_block_started = false;
    for (i, line) in contents.lines().enumerate() {
        if !header_found
            && line.trim().starts_with(README_CHAPTER_HEADER)
            && line
                .to_uppercase()
                .contains(&question.difficulty.to_string().to_uppercase())
        {
            header_found = true;
            continue;
        }
        // No data block yet.
        if !header_found {
            continue;
        }
        // Iterate through the data block.
        if let Some(idx_block) = line.split(README_COL_SEP).nth(1) {
            if let Ok(id) = idx_block.trim().parse::<usize>() {
                data_block_started = true;
                if id > question.id {
                    return Some(i);
                }
            }
        }
        // A wrongly formatted line means the end of the data block.
        // We did not find any index larger in the data block,
        // meaning the question belonged after the previous one.
        // This is done so we can still return a None if
        // the data block ends on the last line of the file.
        else if data_block_started {
            return Some(i);
        }
    }
    line_idx
}

fn update_mod_rs(question: &Question) {
    let path = get_path(question);
    let fname = path
        .file_stem()
        .expect("cannot get filename")
        .to_str()
        .expect("cannot convert to &str");
    let modrs = match question.difficulty {
        Difficulty::Easy => MOD_EASY,
        Difficulty::Medium => MOD_MEDIUM,
        Difficulty::Hard => MOD_HARD,
    };
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(modrs)
        .expect("cannot open mod.rs");
    writeln!(file, "mod {fname};").expect("cannot write");
}

fn get_path(question: &Question) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(LOCAL_FOLDER);
    path.push(format!("{}", question.difficulty).to_lowercase());
    path.push(format!(
        "s{:0>4}_{}.rs",
        question.id,
        question.title.to_lowercase().replace(' ', "_"),
    ));
    path
}

fn get_contents(question: &Question) -> String {
    let mut buffer = String::new();
    buffer.push_str(&get_docs(question));
    buffer.push_str(&get_code_definition(question));
    buffer.push_str(&get_unit_tests(question));
    buffer
}

/// Documentation at top of file:
/// <url>
/// <difficulty> - <tags>
/// <description>
fn get_docs(question: &Question) -> String {
    format!(
        "//! {}\n//! {} - {}\n//!\n//! {}\n\n",
        format_url(question),
        question.difficulty,
        question.topic_tags,
        format_content(&question.content)
    )
}

fn format_content(content: &str) -> String {
    voca_rs::strip::strip_tags(content)
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>()
        .join("\n//! ")
        .replace("&nbsp;", "")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("quot;", "\"")
        .replace("&#39;", "'")
        .replace("\n//! \n", "\n//!\n")
}

fn format_url(question: &Question) -> String {
    format!(
        "{}{}/",
        LEETCODE_QUESTION_URL,
        question.title.to_lowercase().replace(' ', "-")
    )
}

fn get_code_definition(question: &Question) -> String {
    format!(
        "{}\n\n{}\n\n",
        STRUCT_SOLUTION,
        question
            .code_definition
            .0
            .clone()
            .unwrap_or_default()
            .replace(
                "{
        
    }",
                "{\n        todo!()\n    }"
            )
    )
}

fn unit_test_formatter(id: usize, fn_name: &str) -> String {
    format!(
        "#[cfg(test)]
mod tests {{
    use super::Solution;

    #[test]
    fn test_{id:>04}() {{
        assert_eq!(Solution::{fn_name}(todo!(\"input\")), todo!(\"output\"));
    }}
}}
",
    )
}

fn get_fn_name(question: &Question) -> String {
    let mut buffer = String::new();
    if let Some(def) = &question.code_definition.0 {
        if let Some(i) = def.find(FUNCTION_DEF) {
            for ch in def.get((i + FUNCTION_DEF.len())..).unwrap().chars() {
                if ch == '(' {
                    break;
                }
                buffer.push(ch);
            }
        }
    }
    buffer
}

fn get_unit_tests(question: &Question) -> String {
    unit_test_formatter(question.id, &get_fn_name(question))
}

#[cfg(test)]
mod tests {

    use std::path::Path;

    use super::{
        count_files_in_folder, get_contents, get_fn_name, get_readme_line, insert_at_line,
    };

    use super::super::question::Question;

    const TEST_RESOURCES_FOLDER: &str = "src/fetcher/test_resources";
    const QUESTION_JSON: &str = "src/fetcher/test_resources/s0055_question.json";
    const RUST_FILE: &str = "src/fetcher/test_resources/s0055_jump_game.rs";

    #[test]
    #[cfg_attr(not(feature = "fetcher"), ignore)]
    fn test_count_files() {
        assert_eq!(
            count_files_in_folder(Path::new(TEST_RESOURCES_FOLDER), Some("rs")),
            1
        )
    }

    #[test]
    #[cfg_attr(not(feature = "fetcher"), ignore)]
    fn test_get_file_buffer() {
        let question: Question = serde_json::from_reader(
            std::fs::File::open(QUESTION_JSON).expect("cannot open question file"),
        )
        .expect("cannot convert into Question");
        let rust_code = std::fs::read_to_string(RUST_FILE).expect("cannot open rust file");
        assert_eq!(get_contents(&question), rust_code);
    }

    #[test]
    #[cfg_attr(not(feature = "fetcher"), ignore)]
    fn test_insert_at_line() {
        let buffer = "line0\nline1\nline2\nline3\n";
        let line = "NEWLINE";

        insert_at_line_helper(
            buffer,
            Some(0),
            line,
            "NEWLINE\nline0\nline1\nline2\nline3\n",
        );
        insert_at_line_helper(
            buffer,
            Some(1),
            line,
            "line0\nNEWLINE\nline1\nline2\nline3\n",
        );
        insert_at_line_helper(
            buffer,
            Some(2),
            line,
            "line0\nline1\nNEWLINE\nline2\nline3\n",
        );
        insert_at_line_helper(
            buffer,
            Some(3),
            line,
            "line0\nline1\nline2\nNEWLINE\nline3\n",
        );
        insert_at_line_helper(
            buffer,
            Some(4),
            line,
            "line0\nline1\nline2\nline3\nNEWLINE\n",
        );
        insert_at_line_helper(buffer, None, line, "line0\nline1\nline2\nline3\nNEWLINE\n");
    }

    fn insert_at_line_helper(buffer: &str, idx: Option<usize>, line: &str, expected: &str) {
        assert_eq!(insert_at_line(buffer, idx, line), expected)
    }

    #[test]
    #[cfg_attr(not(feature = "fetcher"), ignore)]
    fn test_get_function_name() {
        let question: Question = serde_json::from_reader(
            std::fs::File::open(QUESTION_JSON).expect("cannot open question file"),
        )
        .expect("cannot convert into Question");
        assert_eq!(get_fn_name(&question), "can_jump")
    }

    #[test]
    #[cfg_attr(not(feature = "fetcher"), ignore)]
    fn test_get_line_idx() {
        // Idx 55.
        let question: Question = serde_json::from_reader(
            std::fs::File::open(QUESTION_JSON).expect("cannot open question file"),
        )
        .expect("cannot convert into Question");
        let contents = "
        Random stuff before the interesting part.

        ### Easy

        | Index | Name                                                                                                                                   | Tags                                                        |
        | ----- | -------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------- |
        | 0001  | [Two sum](https://leetcode.com/problems/two-sum/)                                                                                      | array, hash table                                           |

        More random stuff in between.

        ### Medium

        | Index | Name                                                                                                                                   | Tags                                                        |
        | ----- | -------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------- |
        | 0028  | [Find the index of the first occurence in a string](https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/) | two pointers, string, string matching                       |
        | 0035  | [Search insert position](https://leetcode.com/problems/search-insert-position/)                                                        | array, binary search                                        |
        | 0070  | [Climbing stairs](https://leetcode.com/problems/climbing-stairs/)                                                                      | math,
        ";
        assert_eq!(get_readme_line(&question, contents), Some(17));

        let contents = "
        Random stuff before the interesting part.

        ### Easy

        | Index | Name                                                                                                                                   | Tags                                                        |
        | ----- | -------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------- |
        | 0001  | [Two sum](https://leetcode.com/problems/two-sum/)                                                                                      | array, hash table                                           |

        More random stuff in between.

        ### Medium

        | Index | Name                                                                                                                                   | Tags                                                        |
        | ----- | -------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------- |
        | 0028  | [Find the index of the first occurence in a string](https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/) | two pointers, string, string matching                       |
        | 0035  | [Search insert position](https://leetcode.com/problems/search-insert-position/)                                                        | array, binary search                                        |
        
        ### 🔴 Hard

        | Index | Name                                                                                      | Tags                                                               |
        | ----- | ----------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
        | 0004  | [Median of two sorted arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/) | array, binary search, divide and conquer                           |
        | 0023  | [Merge k sorted lists](https://leetcode.com/problems/merge-k-sorted-lists/)               | linked list, divide and conquer, heap (priority queue), merge sort |
        ";
        assert_eq!(get_readme_line(&question, contents), Some(17));

        let contents = "
        Random stuff before the interesting part.

        ### Medium

        | Index | Name                                                                                                                                   | Tags                                                        |
        | ----- | -------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------- |
        | 0028  | [Find the index of the first occurence in a string](https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/) | two pointers, string, string matching                       |
        | 0035  | [Search insert position](https://leetcode.com/problems/search-insert-position/)                                                        | array, binary search                                        |
        ";
        // There is a trailing whiteline behind the final table.
        assert_eq!(get_readme_line(&question, contents), Some(9));

        let contents = "
        Random stuff before the interesting part.

        ### Medium

        | Index | Name                                                                                                                                   | Tags                                                        |
        | ----- | -------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------- |
        | 0070  | [Climbing stairs](https://leetcode.com/problems/climbing-stairs/)                                                                      | math,
        ";
        assert_eq!(get_readme_line(&question, contents), Some(7));

        let contents = "
        Random stuff before the interesting part.

        ### Medium

        | Index | Name                                                                                                                                   | Tags                                                        |
        | ----- | -------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------- |
        ";
        assert_eq!(get_readme_line(&question, contents), None);
    }
}
