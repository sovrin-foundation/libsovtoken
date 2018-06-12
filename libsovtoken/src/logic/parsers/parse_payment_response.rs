//! types used for parse_payment_response_handler
#![allow(unused_variables)]
#![allow(unused_imports)]

use logic::address::verkey_to_address;
use logic::parsers::common::{ResponseOperations, UTXO, TXO};

/**
    for parse_payment_response_handler input resp_json
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParsePaymentResponse {
    pub op : ResponseOperations,
    pub result : ParsePaymentResponseResult,
}

/**
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParsePaymentResponseResult {
    pub identifier: String,
    #[serde(rename = "type")]
    pub txn_type: String,
    pub seq_no: i32,
    pub txn_time: i32,
    pub signature: Option<String>,
    pub signatures: Option<String>,
    pub extra: Option<String>,
    pub req_id: i64,
    pub inputs: Vec<(String, i32, String)>,
    pub outputs: Vec<(String, u32)>,
    pub root_hash: String,
    pub audit_path: Vec<String>
}

/**
    for parse_payment_response_handler output utxo_json
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ParsePaymentReply {
    pub ver : i32,
    pub utxo_json : Vec<UTXO>,
}

impl ParsePaymentReply {
    /**
        Converts ParsePaymentReply (which should be input via indy-sdk) to ParsePaymentReply
        please note:  use of this function moves ParsePaymentResponse and it cannot be used again
        after this call
    */
    pub fn from_response(base : ParsePaymentResponse) -> ParsePaymentReply {
        let mut utxos: Vec<UTXO> = vec![];

        for unspent_output in base.result.outputs {

            let (verkey, amount) = unspent_output;

            let address: String = verkey_to_address(&verkey);

            let txo: TXO = TXO { address: address.to_string(), seq_no: 1 };
            let utxo: UTXO = UTXO { payment_address: address, txo, amount, extra: "".to_string() };

            utxos.push(utxo);
        }

        let reply: ParsePaymentReply = ParsePaymentReply { ver : 1, utxo_json : utxos};
        return reply;
    }
}


#[cfg(test)]
mod parse_payment_response_tests {
    #[allow(unused_imports)]

    use logic::parsers::common::{ResponseOperations, UTXO, TXO};
    use utils::json_conversion::{JsonDeserialize, JsonSerialize};
    use utils::random::{rand_req_id, rand_string};
    use super::*;

    static PARSE_PAYMENT_RESPONSE_JSON: &'static str = r#"{
                "op": "REPLY",
                "result": {
                    "identifier": "QEb3MVVWv1McB8YpgXAvj8SbZDLRRHaPpWt9jFMgfRss3CYBH",
                    "type": "10001",
                    "seqNo": 4,
                    "txnTime": 1527714130,
                    "signature": null,
                    "signatures": null,
                    "extra": null,
                    "reqId": 1527714086374556,
                    "inputs": [
                        ["QEb3MVVWv1McB8YpgXAvj8SbZDLRRHaPpWt9jFMgfRss3CYBH", 3, "3TMn17XTUd7Qr93hiuBWJFyihZ7aQSDbZTwqJEepUFQ5NRoCYYA2ARih2eQLNUZcB2wDSeQaxRFXhrcW2a5RyXrx"],
                        ["t3gQdtHYZaEHTL92j81QEpv5aUHmHKPGQwjEud6mbyhuwvTjV", 3, "4hPYHU1gBnC3ViQEyWf4zz3UPSrT364BfgP5YupBFv6HiuTh7JNLKKDLiiuwxHDHRd4o8AQwGVTT7nJHNTVq8NZy"],
                        ["2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy", 3, "2VvANwBDYNcHyyheGSHx2og7Pc31hw5Box74xZ1EYrm6HijeKqAnKGX6dHF8gL6x78vWUgTpHRA5V41YB7EJMcKq"]
                    ],
                    "outputs": [
                        ["2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC", 11],
                        ["2k7K2zwNTF7pouG3yHqnK2LvVWVj1FdVEUSTkdwtoWYxeULu8h", 19],
                        ["2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy", 9]
                    ],
                    "rootHash": "FRkqRd5jyNRK3SGSGNoR6xMmYQvLVnotGLGWYxR1dCN4",
                    "auditPath": [
                        "6QFFFVbio2q8viWBbuVfvQsv3Qgd3Ub64Qv41i5wH8Bo", "8vDzQmeYb8ecQ7Nyv5i6V8nUwT3fsebqTHMXqgzYi1NU"
                    ]
                }
            }"#;

    // the PARSE_PAYMENT_RESPONSE_JSON is valid per the documentation.   If serde correctly serializes it
    // into ParsePaymentResponse then we know the ParsePaymentResponse structure matches
    #[test]
    fn success_parse_payment_response_from_json() {

        let response: ParsePaymentResponse = ParsePaymentResponse::from_json(PARSE_PAYMENT_RESPONSE_JSON).unwrap();

        assert_eq!(response.op, ResponseOperations::REPLY);
    }

    // this test passes when the valid JSON defined in PARSE_PAYMENT_RESPONSE_JSON is correctly serialized into
    // ParsePaymentResponse which is then successfully converted to ParsePaymentReply and then into json
    #[test]
    fn success_response_json_to_reply_json() {
        let response: ParsePaymentResponse = ParsePaymentResponse::from_json(PARSE_PAYMENT_RESPONSE_JSON).unwrap();
        let reply: ParsePaymentReply = ParsePaymentReply::from_response(response);
    }


}