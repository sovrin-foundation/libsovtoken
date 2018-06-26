//! types used for parse_get_utxo_response_handler
#![allow(unused_variables)]
#![allow(unused_imports)]

use logic::parsers::common::{ResponseOperations, UTXO, TXO, StateProof};
use utils::json_conversion::JsonSerialize;
use indy::ErrorCode;

/**
    for parse_get_utxo_response_handler input parameter resp_json
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParseGetUtxoResponse {
    pub op : ResponseOperations,
    #[serde(rename = "protocol_version")]
    pub protocol_version: Option<i32>,
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
    pub req_id : i64,
    pub outputs : Vec<(String, i32, u32)>,
    #[serde(rename = "state_proof")]
    pub state_proof : StateProof
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
mod parse_get_utxo_responses_tests {
    #[allow(unused_imports)]

    use logic::parsers::common::{ResponseOperations, UTXO, TXO, StateProof, MultiSig, MultiSigValue};
    use utils::json_conversion::{JsonDeserialize, JsonSerialize};
    use utils::random::{rand_req_id, rand_string};
    use super::*;

    static PARSE_GET_UTXO_RESPONSE_JSON: &'static str = r#"{
        "op": "REPLY",
        "protocol_version": 1,
        "result":
            {
                "type": "10002",
                "address": "dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
                "identifier": "6ouriXMZkLeHsuXrN1X1fd",
                "reqId": 15424,
                "outputs":
                [
                    ["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", 1, 40]
                ],
                "state_proof":
                {
                    "multi_signature":
                    {
                        "participants": ["Gamma", "Alpha", "Delta"],
                        "signature": "RNUfcr74ekwBxsT7mxnT2RDFaRRYbfuhebnqQW9PsGkf1bsKC8m8DAqsFfMMLGgAy9CSWM8cyXRUdWLrKUywTajbySfy18oxxdg8ZZApGYHZtiuj6y9sbScAyMwWMmxrDErrj8DWVEVZbGMhPnSSUkmkC6SBnZtSDfdRDvHUMQVBRR",
                        "value":
                        {
                            "ledger_id": 1001,
                            "pool_state_root_hash": "9i3acxaDhCfx9jWXW2JZRoDWzRQEKo7bPBVN7VPE1Jhg",
                            "state_root_hash": "8tJkWdp9wdz3bpb5s5hPDfrjWCQTPmsFKrSdoPmTTnea",
                            "timestamp": 1529705683,
                            "txn_root_hash": "67khbUNo8rySwEtW2SPSsyK4rmLCS7JAN4kYnppELajc"
                        }
                    },
                    "proof_nodes": "+I74ObM0Y3RLU1hCYnYyTXkzVEdHVWdURmpreHUxQTlKTTNTc2NkNUZ5ZFk0ZGt4bmZ3QTdxOjGEw4I0MPhRgICAgICAoKwYfN+WIsLFSOuMjp224HzlSFoSXhXc1+rE\\/vB8jh7MoF\\/sqT9NVI\\/hFuFzQ8LUFSymIKOpOG9nepF29+TB2bWOgICAgICAgICA",
                    "root_hash": "8tJkWdp9wdz3bpb5s5hPDfrjWCQTPmsFKrSdoPmTTnea"
                }
            }
    }"#;


    #[test]
    fn success_parse_get_utxo_reply_from_response() {

        let address: String = rand_string(32);
        let identifier: String = rand_req_id().to_string();
        let mut outputs: Vec<(String, i32, u32)> = Vec::new();
        
        let value : MultiSigValue = MultiSigValue {
            ledger_id: 1001,
            pool_state_root_hash : "9i3acxaDhCfx9jWXW2JZRoDWzRQEKo7bPBVN7VPE1Jhg".to_string(),
            state_root_hash : "8tJkWdp9wdz3bpb5s5hPDfrjWCQTPmsFKrSdoPmTTnea".to_string(),
            timestamp : 1529705683,
            txn_root_hash : "67khbUNo8rySwEtW2SPSsyK4rmLCS7JAN4kYnppELajc".to_string()
        };
        
        let multi_signature : MultiSig = MultiSig {
            participants : vec!["Gamma".to_string(), "Alpha".to_string(), "Delta".to_string()],
            signature : "RNUfcr74ekwBxsT7mxnT2RDFaRRYbfuhebnqQW9PsGkf1bsKC8m8DAqsFfMMLGgAy9CSWM8cyXRUdWLrKUywTajbySfy18oxxdg8ZZApGYHZtiuj6y9sbScAyMwWMmxrDErrj8DWVEVZbGMhPnSSUkmkC6SBnZtSDfdRDvHUMQVBRR".to_string(),
            value
        };
        
        let state_proof : StateProof = StateProof {
            multi_signature,
            proof_nodes : "+I74ObM0Y3RLU1hCYnYyTXkzVEdHVWdURmpreHUxQTlKTTNTc2NkNUZ5ZFk0ZGt4bmZ3QTdxOjGEw4I0MPhRgICAgICAoKwYfN+WIsLFSOuMjp224HzlSFoSXhXc1+rE\\/vB8jh7MoF\\/sqT9NVI\\/hFuFzQ8LUFSymIKOpOG9nepF29+TB2bWOgICAgICAgICA".to_string(),
             root_hash : "8tJkWdp9wdz3bpb5s5hPDfrjWCQTPmsFKrSdoPmTTnea".to_string()
        };

        outputs.push((rand_string(32), 1, 10));
        outputs.push((rand_string(32), 2, 20));

        let outputs_len: usize = outputs.len();

        let result: ParseGetUtxoResponseResult = ParseGetUtxoResponseResult {
            txn_type : "10002".to_string(),
            address,
            identifier,
            req_id: 123457890,
            outputs,
            state_proof
        };

        let response: ParseGetUtxoResponse = ParseGetUtxoResponse {
            op : ResponseOperations::REPLY,
            protocol_version: Some(1),
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

        let value : MultiSigValue = MultiSigValue {
            ledger_id: 1001,
            pool_state_root_hash : "9i3acxaDhCfx9jWXW2JZRoDWzRQEKo7bPBVN7VPE1Jhg".to_string(),
            state_root_hash : "8tJkWdp9wdz3bpb5s5hPDfrjWCQTPmsFKrSdoPmTTnea".to_string(),
            timestamp : 1529705683,
            txn_root_hash : "67khbUNo8rySwEtW2SPSsyK4rmLCS7JAN4kYnppELajc".to_string()
        };
        
        let multi_signature : MultiSig = MultiSig {
            participants : vec!["Gamma".to_string(), "Alpha".to_string(), "Delta".to_string()],
            signature : "RNUfcr74ekwBxsT7mxnT2RDFaRRYbfuhebnqQW9PsGkf1bsKC8m8DAqsFfMMLGgAy9CSWM8cyXRUdWLrKUywTajbySfy18oxxdg8ZZApGYHZtiuj6y9sbScAyMwWMmxrDErrj8DWVEVZbGMhPnSSUkmkC6SBnZtSDfdRDvHUMQVBRR".to_string(),
            value
        };
        
        let state_proof : StateProof = StateProof {
            multi_signature,
            proof_nodes : "+I74ObM0Y3RLU1hCYnYyTXkzVEdHVWdURmpreHUxQTlKTTNTc2NkNUZ5ZFk0ZGt4bmZ3QTdxOjGEw4I0MPhRgICAgICAoKwYfN+WIsLFSOuMjp224HzlSFoSXhXc1+rE\\/vB8jh7MoF\\/sqT9NVI\\/hFuFzQ8LUFSymIKOpOG9nepF29+TB2bWOgICAgICAgICA".to_string(),
             root_hash : "8tJkWdp9wdz3bpb5s5hPDfrjWCQTPmsFKrSdoPmTTnea".to_string()
        };

        let outputs_len: usize = outputs.len();

        let result: ParseGetUtxoResponseResult = ParseGetUtxoResponseResult {
            txn_type : "10002".to_string(),
            address,
            identifier,
            req_id: 123457890,
            outputs,
            state_proof
        };

        let response: ParseGetUtxoResponse = ParseGetUtxoResponse {
            op : ResponseOperations::REPLY,
            protocol_version: Some(1),
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