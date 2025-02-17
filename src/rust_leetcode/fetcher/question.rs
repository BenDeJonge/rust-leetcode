use std::{collections::HashMap, fmt::Display, ops::Deref};

use serde::{Deserialize, Serialize};

use super::difficulty::Difficulty;

/// The attributes of the question that will be retrieved.
pub const QUESTION_QUERY_STRING: &str = r#"
query questionData($titleSlug: String!) {
    question(titleSlug: $titleSlug) {
        title
        content
        codeDefinition
        sampleTestCase
        topicTags {
            slug
        }
        questionId
        difficulty
        isPaidOnly
        }
    }"#;

const TOPIC_TAG_KEY: &str = "slug";
const LANGUAGE_KEY: &str = "value";
const WANTED_LANGUAGE: &str = "rust";
const CODE_DEFINITION_KEY: &str = "defaultCode";

/// A Question object from the LeetCode API.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct QuestionFromLeetcode {
    pub title: String,
    pub content: String,
    #[serde(rename = "codeDefinition")]
    pub code_definition: CodeDefinitionFromLeetcode,
    #[serde(rename = "sampleTestCase")]
    pub sample_test_case: String,
    #[serde(rename = "topicTags")]
    pub topic_tags: TopicTagsFromLeetcode,
    #[serde(rename = "questionId", deserialize_with = "parse")]
    pub id: usize,
    #[serde(deserialize_with = "parse")]
    pub difficulty: Difficulty,
    #[serde(rename = "isPaidOnly")]
    pub is_paid_only: bool,
}

/// A Question object from a JSON file.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Question {
    pub title: String,
    pub content: String,
    #[serde(rename = "codeDefinition")]
    pub code_definition: CodeDefinition,
    #[serde(rename = "sampleTestCase")]
    pub sample_test_case: String,
    #[serde(rename = "topicTags")]
    pub topic_tags: TopicTags,
    #[serde(rename = "questionId")]
    pub id: usize,
    pub difficulty: Difficulty,
    #[serde(rename = "isPaidOnly")]
    pub is_paid_only: bool,
}

impl From<QuestionFromLeetcode> for Question {
    fn from(value: QuestionFromLeetcode) -> Self {
        Question {
            title: value.title,
            content: value.content,
            code_definition: CodeDefinition(value.code_definition.0),
            sample_test_case: value.sample_test_case,
            topic_tags: TopicTags(value.topic_tags.0),
            id: value.id,
            difficulty: value.difficulty,
            is_paid_only: value.is_paid_only,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    question: QuestionFromLeetcode,
}

/// The response from the `QUESTION_QUERRY_STRING` is nested in a `"data"` attribute.
///
/// ```json
/// {
///     "data": {
///         "question": {
///             "title": <str>,
///             "content": <str>,
///             "codeDefinition": <str>,
///             "sampleTestCase": <str>,
///             "topicTags": [
///                 {
///                     "slug": <str>
///                 },
///             ],
///             "questionId": <str>,
///             "difficulty": <str>,
///             "isPaidOnly": <bool>,
///         }
///     }
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct RawQuestion {
    data: Data,
}

impl RawQuestion {
    pub fn take_question(self) -> Question {
        self.data.question.into()
    }
}

// ----------------------------------------------------------------------------
// D E S E R I A L I Z E R S
// ----------------------------------------------------------------------------

/// A `serde`-compliant wrapper around `std::String.parse()`.
fn parse<'de, T, D>(from_str: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    String::deserialize(from_str)?
        .parse()
        .map_err(serde::de::Error::custom)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "Vec<HashMap<String, String>>")]
pub struct TopicTagsFromLeetcode(pub Vec<String>);

impl From<Vec<HashMap<String, String>>> for TopicTagsFromLeetcode {
    fn from(value: Vec<HashMap<String, String>>) -> Self {
        Self(
            value
                .into_iter()
                .filter_map(|mut map| map.remove(TOPIC_TAG_KEY))
                .collect::<Vec<String>>(),
        )
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "Vec<String>")]
pub struct TopicTags(pub Vec<String>);

impl From<Vec<String>> for TopicTags {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

impl Display for TopicTags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "String")]
pub struct CodeDefinitionFromLeetcode(pub Option<String>);

impl Deref for CodeDefinitionFromLeetcode {
    type Target = Option<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for CodeDefinitionFromLeetcode {
    fn from(value: String) -> Self {
        // [
        //  {
        //      "value": "python3",
        //      "text": "Python3",
        //      "defaultCode": "class Solution:\ndef canJump(self, nums: List[int]) -> bool:\n"
        //  },
        // ...
        // ]
        let mut code_definition = None;
        let available_map: Result<Vec<HashMap<String, String>>, serde_json::Error> =
            serde_json::from_str(&value);
        if available_map.is_ok() {
            for mut definition in available_map.unwrap() {
                if let Some(language) = definition.get(LANGUAGE_KEY) {
                    if language == WANTED_LANGUAGE {
                        code_definition = definition.remove(CODE_DEFINITION_KEY);
                    }
                }
            }
        }
        Self(code_definition)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "String")]
pub struct CodeDefinition(pub Option<String>);

impl Deref for CodeDefinition {
    type Target = Option<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for CodeDefinition {
    fn from(value: String) -> Self {
        if value.is_empty() {
            Self(None)
        } else {
            Self(Some(value))
        }
    }
}

#[cfg(test)]
mod test {
    use super::{
        super::difficulty::Difficulty, super::question::Question, CodeDefinition, TopicTags,
    };

    const Q_FILE: &str = "src/rust_leetcode/fetcher/test_resources/s0055_question.json";

    #[test]
    #[cfg_attr(not(feature = "fetcher"), ignore)]
    fn can_read_write_question() {
        let question = Question {
            title: "Jump Game".to_string(),
            content: "<p>You are given an integer array <code>nums</code>. You are initially positioned at the array&#39;s <strong>first index</strong>, and each element in the array represents your maximum jump length at that position.</p>\n\n<p>Return <code>true</code><em> if you can reach the last index, or </em><code>false</code><em> otherwise</em>.</p>\n\n<p>&nbsp;</p>\n<p><strong class=\"example\">Example 1:</strong></p>\n\n<pre>\n<strong>Input:</strong> nums = [2,3,1,1,4]\n<strong>Output:</strong> true\n<strong>Explanation:</strong> Jump 1 step from index 0 to 1, then 3 steps to the last index.\n</pre>\n\n<p><strong class=\"example\">Example 2:</strong></p>\n\n<pre>\n<strong>Input:</strong> nums = [3,2,1,0,4]\n<strong>Output:</strong> false\n<strong>Explanation:</strong> You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.\n</pre>\n\n<p>&nbsp;</p>\n<p><strong>Constraints:</strong></p>\n\n<ul>\n\t<li><code>1 &lt;= nums.length &lt;= 10<sup>4</sup></code></li>\n\t<li><code>0 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>\n</ul>\n".to_string(),
            code_definition: CodeDefinition(Some("impl Solution {\n    pub fn can_jump(nums: Vec<i32>) -> bool {\n        \n    }\n}".to_string())),
            sample_test_case: "[2,3,1,1,4]".to_string(),
            topic_tags: TopicTags(vec![
                "array".to_string(),
                "dynamic-programming".to_string(),
                "greedy".to_string()
            ]),
            id: 55,
            difficulty: Difficulty::Medium,
            is_paid_only: false
        };
        assert!(serde_json::to_string_pretty(&question).is_ok());
        assert_eq!(
            question,
            serde_json::from_reader(std::fs::File::open(Q_FILE).expect("cannot open file"))
                .expect("cannot convert into Question")
        );
    }
}
