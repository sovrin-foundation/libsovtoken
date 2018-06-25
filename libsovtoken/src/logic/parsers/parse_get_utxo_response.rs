//! types used for parse_get_utxo_response_handler
#![allow(unused_variables)]
#![allow(unused_imports)]

use logic::parsers::common::{ResponseOperations, UTXO, TXO};
use utils::json_conversion::JsonSerialize;
use indy::ErrorCode;

/**
    for parse_get_utxo_response_handler input parameter resp_json
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParseGetUtxoResponse {
    pub op : ResponseOperations,
    pub protocol_version: i32,
    pub result : ParseGetUtxoResponseResult,
}

/**
    ParseGetUtxoResponseResult is the structure for the result
    member of ParseGetUtxoResponse
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParseGetUtxoResponseResult {
    #[serde(rename = "type")]
    pub txn_type : String,
    pub address : String,
    pub identifier: String,
    pub req_id: i64,
    pub outputs : Vec<(String, i32, u32)>,
}

/**
   for parse_get_utxo_response_handler output parameter utxo_json
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ParseGetUtxoReply {
    pub ver : i32,
    pub utxo_json : Vec<UTXO>,
}



impl ParseGetUtxoReply {
    /**
        Converts ParseGetUtxoResponse (which should be input via indy-sdk) to ParseGetUtxoReply
        please note:  use of this function moves ParseGetUtxoResponse and it cannot be used again
        after this call
    */
    pub fn from_response(base : ParseGetUtxoResponse) -> Result<ParseGetUtxoReply, ErrorCode> {
        let mut utxos: Vec<UTXO> = vec![];

        for unspent_output in base.result.outputs {

            let (address, seq_no, amount) = unspent_output;

            let txo = match (TXO { address, seq_no }).to_json() {
                Ok(s) => s,
                Err(err) => {
                    error!("JSON serialization error: {:?}", err);
                    return Err(ErrorCode::CommonInvalidState);
                }
            };
            let utxo: UTXO = UTXO { payment_address: base.result.address.to_string(), txo, amount, extra: "".to_string() };

            utxos.push(utxo);
        }

        let reply: ParseGetUtxoReply = ParseGetUtxoReply { ver : 1, utxo_json : utxos};
        return Ok(reply);
    }
}

#[cfg(test)]
mod parse_get_uto_responses_tests {
    #[allow(unused_imports)]

    use logic::parsers::common::{ResponseOperations, UTXO, TXO};
    use utils::json_conversion::{JsonDeserialize, JsonSerialize};
    use utils::random::{rand_req_id, rand_string};
    use super::*;

    static PARSE_GET_UTXO_RESPONSE_JSON: &'static str = r#"{
                        "op": "REPLY",
                        "protocolVersion": 1,
                        "result": {
                            "type": "10002",
                            "address": "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                            "identifier": "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                            "reqId": 23887,
                            "outputs": [
                                ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 2, 10],
                                ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 3, 3]
                            ]
                        }
                    }"#;


    #[test]
    fn success_parse_get_utxo_reply_from_response() {

        let address: String = rand_string(32);
        let identifier: String = rand_req_id().to_string();
        let mut outputs: Vec<(String, i32, u32)> = Vec::new();

        outputs.push((rand_string(32), 1, 10));
        outputs.push((rand_string(32), 2, 20));

        let outputs_len: usize = outputs.len();

        let result: ParseGetUtxoResponseResult = ParseGetUtxoResponseResult {
            txn_type : "1002".to_string(),
            address,
            identifier,
            req_id: 123457890,
            outputs
        };

        let response: ParseGetUtxoResponse = ParseGetUtxoResponse {
            op : ResponseOperations::REPLY,
            protocol_version: 1,
            result
        };

        let reply: ParseGetUtxoReply = ParseGetUtxoReply::from_response(response).unwrap();

        assert_eq!(outputs_len, reply.utxo_json.len());

    }

    #[test]
    fn success_parse_get_utxo_reply_from_response_with_empty_outputs() {
        let address: String = rand_string(32);
        let identifier: String = rand_req_id().to_string();
        let outputs: Vec<(String, i32, u32)> = Vec::new();

        let outputs_len: usize = outputs.len();

        let result: ParseGetUtxoResponseResult = ParseGetUtxoResponseResult {
            txn_type : "1002".to_string(),
            address,
            identifier,
            req_id: 123457890,
            outputs
        };

        let response: ParseGetUtxoResponse = ParseGetUtxoResponse {
            op : ResponseOperations::REPLY,
            protocol_version: 1,
            result
        };

        let reply: ParseGetUtxoReply = ParseGetUtxoReply::from_response(response).unwrap();

        assert_eq!(outputs_len, reply.utxo_json.len());
    }

    // the PARSE_GET_UTXO_RESPONSE_JSON is valid per the documentation.   If serde correctly serializes it
    // into ParseGetUtxoResponse then we know the ParseGetUtxoResponse structure matches
    #[test]
    fn success_parse_get_utxo_response_from_json() {

        let response: ParseGetUtxoResponse = ParseGetUtxoResponse::from_json(PARSE_GET_UTXO_RESPONSE_JSON).unwrap();

        assert_eq!(response.op, ResponseOperations::REPLY);
    }

    // this test passes when the valid JSON defined in PARSE_GET_UTXO_RESPONSE_JSON is correctly serialized into
    // ParseGetUtxoResponse which is then successfully converted to ParseGetUtxoReply and then into json
    #[test]
    fn success_response_json_to_reply_json() {

        let response: ParseGetUtxoResponse = ParseGetUtxoResponse::from_json(PARSE_GET_UTXO_RESPONSE_JSON).unwrap();
        let reply: ParseGetUtxoReply = ParseGetUtxoReply::from_response(response).unwrap();
        let reply_json : String = reply.to_json().unwrap();
    }
}