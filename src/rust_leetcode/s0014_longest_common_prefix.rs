/// https://leetcode.com/problems/longest-common-prefix/
/// Easy - [string, trie]
/// Write a function to find the longest common prefix string amongst an array of strings.
/// If there is no common prefix, return an empty string "".
pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        // The current prefix.
        let mut prefix = strs
            .first()
            .expect("strs is empty")
            .chars()
            .collect::<Vec<char>>();
        for s in strs.iter() {
            // The prefix is no longer matching with the latest string.
            if prefix.is_empty() {
                break;
            }
            // Constructing a new prefix.
            let mut new_prefix = <Vec<char>>::new();
            for (c_p, c_s) in prefix.iter().zip(s.chars()) {
                // The prefix only matches until here with the current word.
                if *c_p != c_s {
                    break;
                }
                // Add a new matching letter.
                new_prefix.push(c_s);
            }
            prefix = new_prefix;
        }
        prefix.iter().collect()
    }

    pub fn longest_common_prefix_ideomatic(strs: Vec<String>) -> String {
        strs.into_iter()
            // Recursively apply the function. The output (intially the 1st element) is stored in acc (accumulated) and
            // every element is processed one after the other. The ouput is then stored again in acc.
            .reduce(|acc, s| {
                acc.chars()
                    .zip(s.chars())
                    .take_while(|(c_p, c_s)| c_p == c_s)
                    .map(|(c, _)| c)
                    .collect()
            })
            .unwrap()
    }
}

fn main() {}

#[cfg(test)]

mod tests {
    use super::Solution;

    #[test]
    fn test_0014() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
        assert_eq!(
            Solution::longest_common_prefix(vec!["ab".to_string(), "a".to_string(),]),
            "a"
        );
    }
}
