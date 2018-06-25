//!

use std::collections::HashMap;
use serde_json;
use serde_json::Error;

use logic::parsers::common::ResponseOperations;
use utils::json_conversion::JsonDeserialize;

/**
    #description
    Structure for parsing GET_FEES request

    #parameters
    op - the operation type received
    protocol_version - the protocol version of the format of the transaction
    result - the payload containing data relevant to the GET_FEES transaction
*/

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParseGetTxnFeesResponse {
    pub op : ResponseOperations,
    pub result : ParseGetTxnFeesResult
}

/*
    #description
    Structure of the result value within the GET_FEES request

    #parameters
    indentifier - The DID this request was submitted from
    req_id - Unique ID number of the request with transaction
    txn_type - the type of transaction that was submitted
    fees - A key:value map with the transaction type as the key and the cost as the value
    state proof - a merkle tree proof used to verify the transaction has been added to the ledger
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParseGetTxnFeesResult {
    pub identifier : String,
    pub req_id : i32,
    //This is to change the json key to adhear to the functionality on ledger
    #[serde(rename = "type")]
    pub txn_type : String,
    pub fees : HashMap<String, i32>,
    //This is being renamed back to the snake case because that is what the JSON object key expects
    #[serde(rename = "state_proof")]
    pub state_proof : StateProof
}

/*
    #description
    Structure of the state proof value within the Result structure

    #parameters
    root_hash - the Merkle root hash of the state trie at the time of response by the ledger
    proof_nodes - the list of hashes necessary to verify the root_hash
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct StateProof {
    pub multi_signature : String,
    pub root_hash : String,
    pub proof_nodes : String
}

pub fn parse_fees_from_get_txn_fees_response(response : String) -> Result<String, Error> {
    let fees_response : ParseGetTxnFeesResponse =
            ParseGetTxnFeesResponse::from_json(&response)?;
    return serde_json::to_string(&fees_response.result.fees);
}

#[cfg(test)]
mod parse_fees_responses_test {
    use super::{parse_fees_from_get_txn_fees_response};
    use serde_json::{Value, Error};
    use serde_json;

    #[test]
    fn success_parse_fees_from_reply_response() {
        let get_fees_response =
            r#"{
                "op": "REPLY",
                "result": {
                    "identifier": "6ouriXMZkLeHsuXrN1X1fd",
                    "reqId": 47660,
                    "type": "20001",
                    "fees": {"1":1,"100":1,"101":3,"102":50,"10000":5,"10001":10},
                    "state_proof": {
                            "multi_signature": "9wdz3msFKrSdoPmTTneabpb5s5hPDfrjWCQTP8tJkWdp",
                            "root_hash": "5BU5Rc3sRtTJB6tVprGiTSqiRaa9o6ei11MjH4Vu16ms",
                            "proof_nodes": "29qFIGZlZXOT0pF7IjEiOjQsIjEwMDAxIjo4fQ=="
                    }
                }
            }"#;

        //setup output of method's data
        let fees_json : String = parse_fees_from_get_txn_fees_response(
            get_fees_response.to_string()).unwrap();
        let parsed_fees_json : Value = serde_json::from_str(&fees_json).unwrap();

        //define and setup expected output from the function
        let expected_json : Value = serde_json::from_str(
            r#"{"1":1,"100":1,"101":3,"102":50,"10000":5,"10001":10}"#).unwrap();

        println!("{:?}", expected_json);
        println!("{:?}", parsed_fees_json);

        //comparison
        assert_eq!(parsed_fees_json, expected_json, "The json objects don't match");
    }

    #[test]
    fn failure_parse_fees_from_reply_response() {
        let invalid_json_response =
            r#"{
                "op": "REPLY",
                "result": {
                    "identifier": "6ouriXMZkLeHsuXrN1X1fd",
                    "reqId": 47660,
                    "type": "20001",
                    "fees": INVALID_JSON,
                    "state_proof": {
                            "multi_signature": "9wdz3msFKrSdoPmTTneabpb5s5hPDfrjWCQTP8tJkWdp",
                            "root_hash": "5BU5Rc3sRtTJB6tVprGiTSqiRaa9o6ei11MjH4Vu16ms",
                            "proof_nodes": "29qFIGZlZXOT0pF7IjEiOjQsIjEwMDAxIjo4fQ=="
                    }
                }
            }"#;


        //convert to Error
        let invalid_fees_json : Result<String, Error> = parse_fees_from_get_txn_fees_response(
            invalid_json_response.to_string());

        let json_error_bool: bool = invalid_fees_json.is_err();
        println!("{:?}", json_error_bool);
        assert!(json_error_bool);
    }
}