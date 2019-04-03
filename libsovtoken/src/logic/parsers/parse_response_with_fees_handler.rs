//! types used for parse_response_with_fees_handler

use logic::address::add_qualifer_to_address;
use logic::input::Inputs;
use logic::output::Outputs;
use logic::parsers::common::{ResponseOperations,
                             UTXO,
                             TXO,
                             TransactionMetaData,
                             RequireSignature};
use logic::parsers::error_code_parser;
use ErrorCode;
use logic::type_aliases::{ProtocolVersion, TokenAmount, TxnSeqNo, TxnVersion};

/**
    for parse_response_with_fees_handler input resp_json

    used in ['parse_response_with_fees_handler']
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParseResponseWithFees {
    pub op : ResponseOperations,
    pub result: Option<ParseResponseWithFeesRequest>,
    pub protocol_version: Option<ProtocolVersion>,
    pub reason: Option<String>,
}

/**
    the nested "request" type in ParseResponseWithFees
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParseResponseWithFeesRequest {
    pub txn : Transaction,
    pub ver: TxnVersion,
    #[serde(rename = "txnMetadata")]
    pub tnx_meta_data: TransactionMetaData,
    pub req_signature: RequireSignature,
    pub root_hash: String,
    pub audit_path: Vec<String>,
    pub fees : Option<TransactionFees>
}

/**
    the nested "txn" type in ParseResponseWithFeesRequest
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionFees {
    pub root_hash: String,
    pub audit_path: Vec<String>,
    #[serde(rename = "txnMetadata")]
    pub tnx_meta_data: TransactionMetaData,
    pub req_signature: RequireSignature,
    pub txn: FeeTxn,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FeeTxn {
    pub data: FeeData,
    pub metadata: TransactionMetaData2
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FeeData {
    pub fees: TokenAmount,
    pub inputs: Inputs,
    pub outputs: Outputs,
    #[serde(rename = "ref")]
    pub reference: String,
}

/**
    the nested "txn" type in ParseResponseWithFeesRequest
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub protocol_version : ProtocolVersion,
    #[serde(rename = "type")]
    pub txn_type : String,
    #[serde(rename = "metadata")]
    pub meta_data: TransactionMetaData2,
}

/**
    the nested type "meta_data" in Transaction
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMetaData2 {
    pub digest: String,
    pub req_id: u64//ReqId
}

/**
    for parse_response_with_fees_handler output utxo_json
*/
pub type ParseResponseWithFeesReply = Vec<UTXO>;

/**
    Converts ParseResponseWithFees (which should be input via indy-sdk) to ParseResponseWithFeesReply
    please note:  use of this function moves ParseResponseWithFees and it cannot be used again
    after this call
*/
pub fn from_response(base : ParseResponseWithFees) -> Result<Option<ParseResponseWithFeesReply>, ErrorCode> {
    match base.op {
        ResponseOperations::REPLY => {
            let result = base.result.ok_or(ErrorCode::CommonInvalidStructure)?;
            let mut utxos: Vec<UTXO> = vec![];

            // according to the documentation, don't need the inputs.  Only the outputs
            // and seq_no which are part 2 and 3 of the tuple
            let fees = match &result.fees {
                Some(fees) => fees,
                None => {return Ok(None)}
            };
            let outputs = &fees.txn.data.outputs;
            let seq_no: TxnSeqNo = fees.tnx_meta_data.seq_no;

            for output in outputs {
                let amount: TokenAmount = output.amount;
                let qualified_address: String = add_qualifer_to_address(&output.recipient);

                let txo = (TXO { address: qualified_address.to_string(), seq_no }).to_libindy_string()?;

                let utxo: UTXO = UTXO { recipient: qualified_address.to_string(), receipt: txo, amount, extra: "".to_string()};

                utxos.push(utxo);
            }

            Ok(Some(utxos))
        }
        ResponseOperations::REQNACK | ResponseOperations::REJECT => {
            let reason = base.reason.ok_or(ErrorCode::CommonInvalidStructure)?;
            Err(error_code_parser::parse_error_code_from_string(&reason))
        }
    }

}

#[cfg(test)]
mod parse_response_with_fees_handler_tests {
    use utils::json_conversion::{JsonDeserialize};
    use super::*;

    // "fees" : [ [ (3) ], [ (2) ], int ]

    static PARSE_RESPONSE_WITH_FEES_JSON: &'static str = r#"{
            "op": "REPLY",
            "protocolVersion": 1,
            "result":
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
                        "reqId": 152969782
                    },
                    "protocolVersion": 2,
                    "type": "1"
                },
                "ver": "1",
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
                    "txn": {
                        "data": {
                            "inputs":
                            [
                                {"address": "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", "seqNo": 2}
                            ],
                            "outputs":
                            [
                                {"address": "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", "amount": 9}
                            ],
                            "fees": 4,
                            "ref": "1:13"
                        },
                        "metadata":
                        {
                            "digest": "54289ff3f7853891e2ba9f4edb4925a0028840008395ea717df8b1f757c4fc77",
                            "reqId": 152969782
                        },
                        "protocolVersion": 2,
                        "type": "1"
                    },
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

    static PARSE_RESPONSE_WITH_NO_FEES_JSON: &'static str = r#"{
            "op": "REPLY",
            "protocolVersion": 1,
            "result":
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
                        "reqId": 152969782
                    },
                    "protocolVersion": 2,
                    "type": "1"
                },
                "ver": "1",
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
                ]
            }
        }"#;

    static PARSE_RESPONSE_WITH_MULTIPLE_FEES_JSON: &'static str = r#"{
            "op": "REPLY",
            "protocolVersion": 1,
            "result":
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
                        "reqId": 152969782
                    },
                    "protocolVersion": 2,
                    "type": "1"
                },
                "ver": "1",
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
                    "txn": {
                        "data": {
                            "inputs":
                            [
                                {"address": "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", "seqNo": 2}
                            ],
                            "outputs":
                            [
                                {"address": "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", "amount": 9},
                                {"address": "11S4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", "amount": 19}
                            ],
                            "fees": 4,
                            "ref": "1:13"
                        },
                        "metadata":
                        {
                            "digest": "54289ff3f7853891e2ba9f4edb4925a0028840008395ea717df8b1f757c4fc77",
                            "reqId": 152969782
                        },
                        "protocolVersion": 2,
                        "type": "1"
                    },
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

    static PARSE_RESPONSE_WITH_FEES_JSON_NO_PROTOCOL_VERSION: &'static str = r#"{
            "op": "REPLY",
            "result":
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
                        "reqId": 152969782
                    },
                    "protocolVersion": 2,
                    "type": "1"
                },
                "ver": "1",
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
                    "txn":
                    {
                        "data":
                        {
                            "inputs":
                            [
                                {"address": "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", "seqNo": 2}
                            ],
                            "outputs":
                            [
                                {"address": "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", "amount": 9}
                            ],
                            "fees": 4,
                            "ref": "1:13"
                        },
                        "metadata":
                        {
                            "digest": "54289ff3f7853891e2ba9f4edb4925a0028840008395ea717df8b1f757c4fc77",
                            "reqId": 152969782
                        },
                        "protocolVersion": 2,
                        "type": "1"
                    },

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
        let outputs = response.result.unwrap().fees.unwrap().txn.data.outputs;

        assert_eq!(1, outputs.len());
    }

    #[test]
    fn success_json_to_parse_response_with_no_fees() {
        let response: ParseResponseWithFees = ParseResponseWithFees::from_json(PARSE_RESPONSE_WITH_NO_FEES_JSON).unwrap();

        // only going to test outputs since we don't use inputs
        assert_eq!(from_response(response), Ok(None));
    }

    // Tests that valid json with multiple elements in the "output section" is serialized to ParseResponseWithFees tyoe
    #[test]
    fn success_json_to_parse_response_with_multiple_fees() {
        let response: ParseResponseWithFees = ParseResponseWithFees::from_json(PARSE_RESPONSE_WITH_MULTIPLE_FEES_JSON).unwrap();

        // only going to test outputs since we don't use inputs
        let outputs = response.result.unwrap().fees.unwrap().txn.data.outputs;

        assert_eq!(2, outputs.len());
    }

    // Tests that valid json with one element in the "output section" is correctly converted to ParseResponseWithFeesReply
    // through the ParseResponseWithFeesReply::from_response method
    #[test]
    fn success_parse_response_with_fees_to_reply() {
        let response: ParseResponseWithFees = ParseResponseWithFees::from_json(PARSE_RESPONSE_WITH_FEES_JSON).unwrap();

        let reply: ParseResponseWithFeesReply = from_response(response).unwrap().unwrap();

        assert_eq!(1, reply.len());

    }

    // Tests that valid json with many elements in the "output section" is correctly converted to ParseResponseWithFeesReply
    // through the ParseResponseWithFeesReply::from_response method
    #[test]
    fn success_parse_response_with_multiple_fees_to_reply() {
        let response: ParseResponseWithFees = ParseResponseWithFees::from_json(PARSE_RESPONSE_WITH_MULTIPLE_FEES_JSON).unwrap();
        let reply: ParseResponseWithFeesReply = from_response(response).unwrap().unwrap();

        assert_eq!(2, reply.len());

    }

    // This test is for TOK-251
    #[test]
    fn success_json_to_parse_response_with_fees_no_protocol_version() {
        let response: ParseResponseWithFees = ParseResponseWithFees::from_json(PARSE_RESPONSE_WITH_FEES_JSON_NO_PROTOCOL_VERSION).unwrap();

        // only going to test outputs since we don't use inputs
        let outputs = response.result.unwrap().fees.unwrap().txn.data.outputs;

        assert_eq!(1, outputs.len());
    }
}