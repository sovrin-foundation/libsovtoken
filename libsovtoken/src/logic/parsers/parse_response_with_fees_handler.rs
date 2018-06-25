//! types used for parse_response_with_fees_handler
#![allow(unused_variables)]
#![allow(unused_imports)]

use logic::address::add_qualifer_to_address;
use logic::parsers::common::{ResponseOperations,
                             UTXO,
                             TXO,
                             TransactionMetaData,
                             RequireSignature,
                             SignatureValues};
use logic::input::Inputs;
use logic::output::{Outputs, Output};
use utils::json_conversion::JsonSerialize;
use indy::ErrorCode;

/**
    for parse_response_with_fees_handler input resp_json

    used in ['parse_response_with_fees_handler']
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParseResponseWithFees {
    pub op : ResponseOperations,
    pub protocol_version: i32,
    pub request : ParseResponseWithFeesRequest,
}

/**
    the nested "request" type in ParseResponseWithFees
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParseResponseWithFeesRequest {
    pub txn : Transaction,
    pub ver: i32,
    #[serde(rename = "txnMetadata")]
    pub tnx_meta_data: TransactionMetaData,
    pub req_signature: RequireSignature,
    pub root_hash: String,
    pub audit_path: Vec<String>,
    pub fees : TransactionFees
}

/**
    the nested "txn" type in ParseResponseWithFeesRequest
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionFees {
    pub fees: i32,
    #[serde(rename = "ref")]
    pub reference: String,
    pub root_hash: String,
    pub audit_path: Vec<String>,
    pub inputs: Vec<(String, i32)>,
    pub outputs: Vec<(String, u32)>,
    #[serde(rename = "txnMetadata")]
    pub tnx_meta_data: TransactionMetaData,
    pub req_signature: RequireSignature,
}

/**
    the nested "txn" type in ParseResponseWithFeesRequest
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub protocol_version : i32,
    #[serde(rename = "type")]
    pub txn_type : String,
    #[serde(rename = "metadata")]
    pub meta_data: TransactionMetaData2,
    pub data : TransactionData
}

/**
    the nested type "meta_data" in Transaction
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMetaData2 {
    pub digest: String,
    pub req_id: i64
}

/**
    the nested type "data" in Transaction
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionData {
    pub alias: String,
    pub dest : String,
    #[serde(rename = "verkey")]
    pub ver_key : String,
}

/**
    for parse_response_with_fees_handler output utxo_json
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ParseResponseWithFeesReply {
    pub ver : i32,
    pub utxo_json : Vec<UTXO>,
}

impl ParseResponseWithFeesReply {
    /**
        Converts ParseResponseWithFees (which should be input via indy-sdk) to ParseResponseWithFeesReply
        please note:  use of this function moves ParseResponseWithFees and it cannot be used again
        after this call
    */
    pub fn from_response(base : ParseResponseWithFees) -> Result<ParseResponseWithFeesReply, ErrorCode> {
        let mut utxos: Vec<UTXO> = vec![];

        // according to the documentation, don't need the inputs.  Only the outputs
        // and seq_no which are part 2 and 3 of the tuple
        let outputs = &base.request.fees.outputs;

        for output in outputs {
            let output_address : String = output.0.to_string();
            let amount: u32 = output.1;
            let qualified_address: String = add_qualifer_to_address(&output_address);
            let seq_no: i32 = ParseResponseWithFeesReply::find_seq_no(&base, &output_address);

            let txo = match (TXO { address: qualified_address.to_string(), seq_no }).to_json() {
                Ok(s) => s,
                Err(err) => {
                    error!("JSON serialization error: {:?}", err);
                    return Err(ErrorCode::CommonInvalidState);
                }
            };

            let utxo: UTXO = UTXO { payment_address: qualified_address.to_string(), txo, amount, extra: "".to_string()};

            utxos.push(utxo);
        }

        let reply: ParseResponseWithFeesReply = ParseResponseWithFeesReply { ver : 1, utxo_json : utxos};
        return Ok(reply);
    }

    fn find_seq_no(base : &ParseResponseWithFees, output_address: &String) -> i32 {
        let inputs = &base.request.fees.inputs;

        for input in inputs {
            let input_address : String = input.0.to_string();

            if input_address == output_address.to_string() {
                return input.1;
            }

        }

        return -1;
    }
}

#[cfg(test)]
mod parse_response_with_fees_handler_tests {
    #[allow(unused_imports)]

    use logic::address::{ADDRESS_LEN, VERKEY_LEN, ADDRESS_CHECKSUM_LEN};
    use utils::json_conversion::{JsonDeserialize, JsonSerialize};
    use utils::random::{rand_req_id, rand_string};
    use super::*;

    // "fees" : [ [ (3) ], [ (2) ], int ]

    static PARSE_RESPONSE_WITH_FEES_JSON: &'static str = r#"{
            "op": "REPLY",
            "protocolVersion": 1,
            "request":
            {
                "txn":
                {
                    "data":
                    {
                        "alias": "508867",
                        "dest": "8Wv7NMbsMiNSmNa3iC6fG7",
                        "verkey": "56b9wim9b3dYXzzc8wnm8RZePbyuMoWw5XUXxL4Y9gFZ"
                    },
                    "metadata":
                    {
                        "digest": "54289ff3f7853891e2ba9f4edb4925a0028840008395ea717df8b1f757c4fc77",
                        "reqId": 1529697827844532830
                    },
                    "protocolVersion": 2,
                    "type": "1"
                },
                "ver": 1,
                "txnMetadata":
                {
                    "seqNo": 13,
                    "txnTime": 1529697829
                },
                "reqSignature":
                {
                    "type": "ED25519",
                    "values":
                    [
                        {
                            "from": "MSjKTWkPLtYoPEaTF1TUDb",
                            "value": "5Ngg5fQ4NtqdzgN3kSjdRKo6ffeq5sP264TmzxvGGQX3ieJzP9hCeUCu7RkmAhLjzqZ2Z5y8FLSptWxetS8FCmcs"
                        }
                    ]
                },
                "rootHash": "FePFuqEX6iJ1SP5DkYn9WTXQrThxqevEkxYXyCxyX4Fd",
                "auditPath":
                [
                    "CWQ9keGzhBqyMRLvp7XbMr7da7yUbEU4qGTfJ2KNxMM6",
                    "2S9HAxKukY2hxUoEC718fhywF3KRfwPnEQvRsoN168EV"
                ],
                "fees":
                {
                    "inputs":
                    [
                        ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 2]
                    ],
                    "outputs":
                    [
                        ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 9]
                    ],
                    "fees": 4,
                    "ref": "1:13",
                    "reqSignature":
                    {
                        "type": "ED25519",
                        "values":
                        [
                            {
                                "from": "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                                "value": "5Z7ktpfVQAhj2gMFR8L6JnG7fQQJzqWwqrDgXQP1CYf2vrjKPe2a27borFVuAcQh2AttoejgAoTzJ36wfyKxu5ox"
                            }
                        ]
                    },
                    "txnMetadata":
                    {
                        "seqNo": 2,
                        "txnTime": 1529697829
                    },
                    "rootHash": "A8qwQKyKUMd3PnJTKe4bXRzajCUVgSd1J1A7jdahhNW6",
                    "auditPath": ["Gyw5iBPPs4KSiEoAXQcjv8jw1VWsFjTVyCkm1Zp9E3Pa"]
                }
            }
        }"#;

    static PARSE_RESPONSE_WITH_MULTIPLE_FEES_JSON: &'static str = r#"{
            "op": "REPLY",
            "protocolVersion": 1,
            "request":
            {
                "txn":
                {
                    "data":
                    {
                        "alias": "508867",
                        "dest": "8Wv7NMbsMiNSmNa3iC6fG7",
                        "verkey": "56b9wim9b3dYXzzc8wnm8RZePbyuMoWw5XUXxL4Y9gFZ"
                    },
                    "metadata":
                    {
                        "digest": "54289ff3f7853891e2ba9f4edb4925a0028840008395ea717df8b1f757c4fc77",
                        "reqId": 1529697827844532830
                    },
                    "protocolVersion": 2,
                    "type": "1"
                },
                "ver": 1,
                "txnMetadata":
                {
                    "seqNo": 13,
                    "txnTime": 1529697829
                },
                "reqSignature":
                {
                    "type": "ED25519",
                    "values":
                    [
                        {
                            "from": "MSjKTWkPLtYoPEaTF1TUDb",
                            "value": "5Ngg5fQ4NtqdzgN3kSjdRKo6ffeq5sP264TmzxvGGQX3ieJzP9hCeUCu7RkmAhLjzqZ2Z5y8FLSptWxetS8FCmcs"
                        }
                    ]
                },
                "rootHash": "FePFuqEX6iJ1SP5DkYn9WTXQrThxqevEkxYXyCxyX4Fd",
                "auditPath":
                [
                    "CWQ9keGzhBqyMRLvp7XbMr7da7yUbEU4qGTfJ2KNxMM6",
                    "2S9HAxKukY2hxUoEC718fhywF3KRfwPnEQvRsoN168EV"
                ],
                "fees":
                {
                    "inputs":
                    [
                        ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 2]
                    ],
                    "outputs":
                    [
                        ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 9],
                        ["11S4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 19]
                    ],
                    "fees": 4,
                    "ref": "1:13",
                    "reqSignature":
                    {
                        "type": "ED25519",
                        "values":
                        [
                            {
                                "from": "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                                "value": "5Z7ktpfVQAhj2gMFR8L6JnG7fQQJzqWwqrDgXQP1CYf2vrjKPe2a27borFVuAcQh2AttoejgAoTzJ36wfyKxu5ox"
                            }
                        ]
                    },
                    "txnMetadata":
                    {
                        "seqNo": 2,
                        "txnTime": 1529697829
                    },
                    "rootHash": "A8qwQKyKUMd3PnJTKe4bXRzajCUVgSd1J1A7jdahhNW6",
                    "auditPath": ["Gyw5iBPPs4KSiEoAXQcjv8jw1VWsFjTVyCkm1Zp9E3Pa"]
                }
            }
        }"#;

    // Tests that valid json with one element in the "output section" is serialized to ParseResponseWithFees tyoe
    #[test]
    fn success_json_to_parse_response_with_fees() {
        let response: ParseResponseWithFees = ParseResponseWithFees::from_json(PARSE_RESPONSE_WITH_FEES_JSON).unwrap();

        // only going to test outputs since we don't use inputs
        let outputs= response.request.fees.outputs;

        assert_eq!(1, outputs.len());
    }

    // Tests that valid json with multiple elements in the "output section" is serialized to ParseResponseWithFees tyoe
    #[test]
    fn success_json_to_parse_response_with_multiple_fees() {
        let response: ParseResponseWithFees = ParseResponseWithFees::from_json(PARSE_RESPONSE_WITH_MULTIPLE_FEES_JSON).unwrap();

        // only going to test outputs since we don't use inputs
        let outputs= response.request.fees.outputs;

        assert_eq!(2, outputs.len());
    }

    // Tests that valid json with one element in the "output section" is correctly converted to ParseResponseWithFeesReply
    // through the ParseResponseWithFeesReply::from_response method
    #[test]
    fn success_parse_response_with_fees_to_reply() {
        let response: ParseResponseWithFees = ParseResponseWithFees::from_json(PARSE_RESPONSE_WITH_FEES_JSON).unwrap();

        let reply: ParseResponseWithFeesReply = ParseResponseWithFeesReply::from_response(response).unwrap();

        assert_eq!(1, reply.utxo_json.len());

    }

    // Tests that valid json with many elements in the "output section" is correctly converted to ParseResponseWithFeesReply
    // through the ParseResponseWithFeesReply::from_response method
    #[test]
    fn success_parse_response_with_multiple_fees_to_reply() {
        let response: ParseResponseWithFees = ParseResponseWithFees::from_json(PARSE_RESPONSE_WITH_MULTIPLE_FEES_JSON).unwrap();
        let reply: ParseResponseWithFeesReply = ParseResponseWithFeesReply::from_response(response).unwrap();

        assert_eq!(2, reply.utxo_json.len());

    }
}