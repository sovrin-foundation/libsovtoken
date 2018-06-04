//! types used for parse_payment_response_handler
#![allow(unused_variables)]
#![allow(unused_imports)]

/**
    for parse_response_with_fees_handler input resp_json
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParseResponseWithFeesRequest {
    pub fees : (Vec<(String, i32, String)>, Vec<(String, i32)>, i32),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Input {
    pub payment_address: String,
    pub sequence_number: u32,
    pub signature: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Output {
    pub payment_address: String,
    pub amount: u32
}

#[cfg(test)]
mod parse_response_with_fees_handler_tests {
    #[allow(unused_imports)]

    use utils::json_conversion::{JsonDeserialize, JsonSerialize};
    use utils::random::{rand_req_id, rand_string};
    use super::*;

    // "fees" : [ [], [], int ]

    static PARSE_RESPONSE_WITH_FEES_JSON: &'static str = r#"{
                "fees": [
                    [
                        ["QEb3MVVWv1McB8YpgXAvj8SbZDLRRHaPpWt9jFMgfRss3CYBH", 2, "5Z7ktpfVQAhj2gMFR8L6JnG7fQQJzqWwqrDgXQP1CYf2vrjKPe2a27borFVuAcQh2AttoejgAoTzJ36wfyKxu5ox"]
                    ],
                    [
                        ["2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC", 11]
                    ],
                    3
                ]
            }"#;

    #[test]
    fn success_json_to_parse_response_with_fees_response() {
        let reply: ParseResponseWithFeesRequest = ParseResponseWithFeesRequest::from_json(PARSE_RESPONSE_WITH_FEES_JSON).unwrap();
    }
}