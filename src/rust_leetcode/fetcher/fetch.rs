use super::question::{Question, RawQuestion, QUESTION_QUERY_STRING};

use reqwest::Client;
use serde_json::json;

/// The URL where `POST` requests can be made.
const LEETCODE_GRAPHQL: &str = "https://leetcode.com/graphql";
/// The operation to retrieve parameters of a LeetCode question.
const QUESTION_QUERRY_OPERATION: &str = "questionData";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Query {
    #[serde(rename = "operationName")]
    operation_name: String,
    variables: serde_json::Value,
    query: String,
}

impl Query {
    pub fn question_querry(title_slug: &str) -> Self {
        Query {
            operation_name: QUESTION_QUERRY_OPERATION.to_owned(),
            variables: json!({"titleSlug": title_slug}),
            query: QUESTION_QUERY_STRING.to_owned(),
        }
    }
}

pub async fn fetch_question_async(name: &str) -> Result<Question, reqwest::Error> {
    let client = Client::new();
    let question = client
        .post(LEETCODE_GRAPHQL)
        .json(&Query::question_querry(name))
        .send()
        .await?
        .json::<RawQuestion>()
        .await?
        .take_question();
    Ok(question)
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::super::question::Question;
    use super::fetch_question_async;

    const Q_NAME: &str = "jump-game";
    const Q_FILE: &str = "src/rust_leetcode/fetcher/test_resources/s0055_question.json";

    #[tokio::test]
    #[cfg_attr(not(feature = "fetcher"), ignore)]
    async fn test_fetch_async() {
        let question = fetch_question_async(Q_NAME)
            .await
            .expect("cannot fetch Question");
        let from_json: Question =
            serde_json::from_reader(fs::File::open(Q_FILE).expect("cannot open file"))
                .expect("cannot convert into Question");
        assert_eq!(question, from_json);
    }
}
