//! types used for parse_get_utxo_response_handler

use base64;
use ErrorCode;
use libc::c_char;
use logic::parsers::common::{ResponseOperations, TXO, StateProof, ParsedSP, KeyValuesInSP,
                             KeyValueSimpleData, extract_result_and_state_proof_from_node_reply};
use logic::parsers::error_code_parser;
use logic::type_aliases::{TokenAmount, TxnSeqNo, ProtocolVersion, ReqId};
use logic::address;
use serde_json;
use utils::constants::txn_fields::OUTPUTS;
use utils::ffi_support::c_pointer_from_string;

type UTXOs = Vec<UTXOInner>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UTXOInner {
    pub address: String,
    pub seq_no: TxnSeqNo,
    pub amount: TokenAmount
}

/**
    for parse_get_utxo_response_handler input parameter resp_json
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParseGetUtxoResponse {
    pub op : ResponseOperations,
    #[serde(rename = "protocol_version")]
    pub protocol_version: Option<ProtocolVersion>,
    pub result : Option<ParseGetUtxoResponseResult>,
    pub reason: Option<String>,
}

/**
    ParseGetUtxoResponseResult is the structure for the result
    member of ParseGetUtxoResponse
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParseGetUtxoResponseResult {
    #[serde(rename = "type")]
    pub txn_type : String,
    pub address : String,
    pub identifier: String,
    pub req_id : ReqId,
    pub outputs : UTXOs,
    #[serde(rename = "state_proof", skip_serializing_if = "Option::is_none")]
    pub state_proof : Option<StateProof>
}

/**
    UTXO is the structure for the data member utxo_json

    Do not confuse with [`UTXO`] from common, this one has different field names

    used by [`ParseGetUtxoReply`]
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UTXO {
    pub payment_address: String,
    pub source: String,
    pub amount: TokenAmount,
    pub extra: String,
}


/**
   for parse_get_utxo_response_handler output parameter utxo_json
*/
pub type ParseGetUtxoReply = Vec<UTXO>;

/**
    Converts ParseGetUtxoResponse (which should be input via indy-sdk) to ParseGetUtxoReply
    please note:  use of this function moves ParseGetUtxoResponse and it cannot be used again
    after this call
*/
pub fn from_response(base : ParseGetUtxoResponse) -> Result<ParseGetUtxoReply, ErrorCode> {
    match base.op {
        ResponseOperations::REPLY => {
            let result = base.result.ok_or(ErrorCode::CommonInvalidStructure)?;
            let mut utxos: Vec<UTXO> = vec![];

            for unspent_output in result.outputs {

                let payment_address = address::address_from_unqualified_address(&result.address.to_string())?;
                let txo = (TXO { address: payment_address.clone(), seq_no: unspent_output.seq_no }).to_libindy_string()?;
                let utxo: UTXO = UTXO { payment_address, source: txo, amount: unspent_output.amount, extra: "".to_string() };

                utxos.push(utxo);
            }

            Ok(utxos)
        }
        ResponseOperations::REQNACK | ResponseOperations::REJECT => {
            let reason = base.reason.ok_or(ErrorCode::CommonInvalidStructure)?;
            Err(error_code_parser::parse_error_code_from_string(&reason))
        }
    }
}

// Assumes a valid address. The delimeter `:` has to be the same as used on ledger
pub fn get_utxo_state_key(address: &str, seq_no: TxnSeqNo) -> String {
    base64::encode(&format!("{}:{}", address, seq_no))
}

pub fn get_utxo_state_proof_extractor(reply_from_node: *const c_char, parsed_sp: *mut *const c_char) -> ErrorCode {
    // TODO: The following errors should have logs
    let (result, state_proof) = match extract_result_and_state_proof_from_node_reply(reply_from_node) {
        Ok((r, s)) => (r, s),
        Err(_) => return ErrorCode::CommonInvalidStructure
    };

    // TODO: No validation of outputs being done. This has to fixed by creating an `Address` with
    // a single private field called `address` and with implementation defining `new` and a getter.
    // The `new` method will do the validation.

    let outputs: UTXOs = match result.get(OUTPUTS) {
        Some(outs) => {
            let outputs: UTXOs = match serde_json::from_value(outs.to_owned()) {
                Ok(o) => o,
                Err(_) => return ErrorCode::CommonInvalidStructure
            };
            outputs
        },
        None => return ErrorCode::CommonInvalidStructure
    };

    let mut kvs: Vec<(String, Option<String>)> = Vec::new();

    for output in outputs {
        kvs.push((get_utxo_state_key(&output.address, output.seq_no),
                  Some(output.amount.to_string())));
    }

    let kvs_to_verify = KeyValuesInSP::Simple(KeyValueSimpleData { kvs });
    let proof_nodes = match state_proof.proof_nodes {
        Some(o) => o,
        None => return ErrorCode::CommonInvalidStructure
    };
    let root_hash = match state_proof.root_hash {
        Some(o) => o,
        None => return ErrorCode::CommonInvalidStructure
    };
    let multi_signature = match state_proof.multi_signature {
        Some(o) => o,
        None => return ErrorCode::CommonInvalidStructure
    };

    let sp = vec![ParsedSP {
        proof_nodes,
        root_hash,
        kvs_to_verify,
        multi_signature,
    }];

    match serde_json::to_string(&sp) {
        Ok(s) => {
            trace!("JSON representation of ParsedSP for get utxo {:?}", &s);
            unsafe { *parsed_sp = c_pointer_from_string(s); }
            return ErrorCode::Success;
        },
        Err(_) => return ErrorCode::CommonInvalidStructure
    }
}

#[cfg(test)]
mod parse_get_utxo_responses_tests {
    use super::*;
    use std::ffi::CString;

    use logic::parsers::common::{ResponseOperations, StateProof};
    use utils::base58::IntoBase58;
    use utils::json_conversion::{JsonDeserialize, JsonSerialize};
    use utils::random::{rand_req_id, rand_string};
    use utils::constants::txn_types::GET_UTXO;
    use utils::ffi_support::string_from_char_ptr;

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

        let address: String = "00000000000000000000000000000000".as_bytes().into_base58_check();
        let identifier: String = rand_req_id().to_string();
        let mut outputs: Vec<UTXOInner> = Vec::new();

        let multi_signature = json!({
            "participants" : ["Gamma", "Alpha", "Delta"],
            "signature" : "RNUfcr74ekwBxsT7mxnT2RDFaRRYbfuhebnqQW9PsGkf1bsKC8m8DAqsFfMMLGgAy9CSWM8cyXRUdWLrKUywTajbySfy18oxxdg8ZZApGYHZtiuj6y9sbScAyMwWMmxrDErrj8DWVEVZbGMhPnSSUkmkC6SBnZtSDfdRDvHUMQVBRR".to_string(),
            "value": {
                "ledger_id": 1001,
                "pool_state_root_hash" : "9i3acxaDhCfx9jWXW2JZRoDWzRQEKo7bPBVN7VPE1Jhg".to_string(),
                "state_root_hash" : "8tJkWdp9wdz3bpb5s5hPDfrjWCQTPmsFKrSdoPmTTnea".to_string(),
                "timestamp" : 1529705683,
                "txn_root_hash" : "67khbUNo8rySwEtW2SPSsyK4rmLCS7JAN4kYnppELajc".to_string()
            }
        });

        let state_proof : StateProof = StateProof {
            multi_signature: Some(multi_signature),
            proof_nodes : Some("+I74ObM0Y3RLU1hCYnYyTXkzVEdHVWdURmpreHUxQTlKTTNTc2NkNUZ5ZFk0ZGt4bmZ3QTdxOjGEw4I0MPhRgICAgICAoKwYfN+WIsLFSOuMjp224HzlSFoSXhXc1+rE\\/vB8jh7MoF\\/sqT9NVI\\/hFuFzQ8LUFSymIKOpOG9nepF29+TB2bWOgICAgICAgICA".to_string()),
            root_hash : Some("8tJkWdp9wdz3bpb5s5hPDfrjWCQTPmsFKrSdoPmTTnea".to_string())
        };

        outputs.push(UTXOInner {
            address: rand_string(32),
            seq_no: 1,
            amount: 10
        });
        outputs.push(UTXOInner {
            address: rand_string(32),
            seq_no: 2,
            amount: 20
        });

        let outputs_len: usize = outputs.len();

        let result: ParseGetUtxoResponseResult = ParseGetUtxoResponseResult {
            txn_type : GET_UTXO.to_string(),
            address,
            identifier,
            req_id: 123457890,
            outputs,
            state_proof: Some(state_proof)
        };

        let response: ParseGetUtxoResponse = ParseGetUtxoResponse {
            op : ResponseOperations::REPLY,
            protocol_version: Some(1),
            result: Some(result),
            reason: None,
        };

        let reply: ParseGetUtxoReply = from_response(response).unwrap();

        assert_eq!(outputs_len, reply.len());

    }

    #[test]
    fn success_parse_get_utxo_reply_from_response_with_empty_outputs() {
        let address: String = rand_string(32);
        let identifier: String = rand_req_id().to_string();
        let outputs: Vec<UTXOInner> = Vec::new();

        let multi_signature = json!({
            "participants" : ["Gamma", "Alpha", "Delta"],
            "signature" : "RNUfcr74ekwBxsT7mxnT2RDFaRRYbfuhebnqQW9PsGkf1bsKC8m8DAqsFfMMLGgAy9CSWM8cyXRUdWLrKUywTajbySfy18oxxdg8ZZApGYHZtiuj6y9sbScAyMwWMmxrDErrj8DWVEVZbGMhPnSSUkmkC6SBnZtSDfdRDvHUMQVBRR".to_string(),
            "value": {
                "ledger_id": 1001,
                "pool_state_root_hash" : "9i3acxaDhCfx9jWXW2JZRoDWzRQEKo7bPBVN7VPE1Jhg".to_string(),
                "state_root_hash" : "8tJkWdp9wdz3bpb5s5hPDfrjWCQTPmsFKrSdoPmTTnea".to_string(),
                "timestamp" : 1529705683,
                "txn_root_hash" : "67khbUNo8rySwEtW2SPSsyK4rmLCS7JAN4kYnppELajc".to_string()
            }
        });

        let state_proof : StateProof = StateProof {
            multi_signature: Some(multi_signature),
            proof_nodes : Some("+I74ObM0Y3RLU1hCYnYyTXkzVEdHVWdURmpreHUxQTlKTTNTc2NkNUZ5ZFk0ZGt4bmZ3QTdxOjGEw4I0MPhRgICAgICAoKwYfN+WIsLFSOuMjp224HzlSFoSXhXc1+rE\\/vB8jh7MoF\\/sqT9NVI\\/hFuFzQ8LUFSymIKOpOG9nepF29+TB2bWOgICAgICAgICA".to_string()),
            root_hash : Some("8tJkWdp9wdz3bpb5s5hPDfrjWCQTPmsFKrSdoPmTTnea".to_string())
        };

        let outputs_len: usize = outputs.len();

        let result: ParseGetUtxoResponseResult = ParseGetUtxoResponseResult {
            txn_type : GET_UTXO.to_string(),
            address,
            identifier,
            req_id: 123457890,
            outputs,
            state_proof: Some(state_proof)
        };

        let response: ParseGetUtxoResponse = ParseGetUtxoResponse {
            op : ResponseOperations::REPLY,
            protocol_version: Some(1),
            result: Some(result),
            reason: None,
        };

        let reply: ParseGetUtxoReply = from_response(response).unwrap();

        assert_eq!(outputs_len, reply.len());
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
        let reply: ParseGetUtxoReply = from_response(response).unwrap();
        let reply_json = reply.to_json();
        assert!(reply_json.is_ok());
    }

    #[test]
    fn test_utxo_state_key() {
        let address = "dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q";
        let seq_no = 32;
        let key = get_utxo_state_key(&address, seq_no);
        assert_eq!(key, base64::encode("dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q:32"));
    }

    #[test]
    fn test_reply_without_outputs() {
        let invalid_json = r#"{
            "op": "REPLY",
            "protocol_version": 1,
            "result":
                {
                    "type": "10002",
                    "address": "dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
                    "identifier": "6ouriXMZkLeHsuXrN1X1fd",
                    "reqId": 15424,
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

        let json_str = CString::new(invalid_json).unwrap();
        let json_str_ptr = json_str.as_ptr();

        let mut new_str_ptr = ::std::ptr::null();

        let return_error = get_utxo_state_proof_extractor(json_str_ptr, &mut new_str_ptr);
        assert_eq!(return_error, ErrorCode::CommonInvalidStructure);
    }

    #[test]
    fn test_parse_state_proof_success() {
        let valid_json = r#"{
            "op": "REPLY",
            "protocol_version": 1,
            "result":
                {
                    "type": "10002",
                    "address": "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                    "identifier": "6ouriXMZkLeHsuXrN1X1fd",
                    "reqId": 15424,
                    "outputs":[
                      [
                         "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                         4,
                         1
                      ],
                      [
                         "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                         16,
                         1
                      ],
                      [
                         "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                         6,
                         1
                      ],
                      [
                         "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                         17,
                         1
                      ],
                      [
                         "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                         20,
                         1
                      ],
                      [
                         "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                         8,
                         1
                      ],
                      [
                         "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                         13,
                         1
                      ],
                      [
                         "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                         3,
                         1
                      ],
                      [
                         "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                         19,
                         1
                      ],
                      [
                         "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                         11,
                         1
                      ],
                      [
                         "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                         18,
                         1
                      ],
                      [
                         "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                         5,
                         1
                      ],
                      [
                         "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                         15,
                         1
                      ],
                      [
                         "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                         7,
                         1
                      ],
                      [
                         "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                         10,
                         1
                      ],
                      [
                         "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                         9,
                         1
                      ],
                      [
                         "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                         12,
                         1
                      ],
                      [
                         "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                         2,
                         1
                      ],
                      [
                         "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                         21,
                         1
                      ],
                      [
                         "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                         14,
                         1
                      ]
                   ],
                    "state_proof":{
                      "root_hash":"EuHbjY9oaqAXBDxLBM4KcBLASs7RK35maoHjQMbDvmw1",
                      "proof_nodes":"+QHF4hOgCBgvwaPO/KIJjOyzhA9dx8yXqPgqKY9sqKPIAZgHujTsgICA2cQggsExxCCCwTGAgICAgICAgICAgICAgICAgICAgICAgICAgICCwTH4VrQAJqUzRQSFdRSktjYXdSeGRXNkdWc2puWkJhMWVjR2RDc3NuN0toV1lKWkdUWGdMN0VzOjoEREJ+KujHB//IMaixsQMlj9+4DLVQHzu4WJczS7X8ED+G2AoMk4QTH20sQPm8C23HQjM7dFR6HIi99DdtySfD9VnTGsoDyHJeCRAIf9srqEpWYrQ1nq9jBE67eMCBK+ewpvMu2UxCCCwTHEIILBMcQggsExxCCCwTHEIILBMcQggsExxCCCwTGAgICAgICA+DnEIILBMcQggsExxCCCwTHEIILBMcQggsExxCCCwTHEIILBMcQggsExxCCCwTHEIILBMYCAgICAgID4cYCAgKDXpPuRat5Zsa2SRHuGjslN7/QaBcZvwSae8dKLWybem4CAoAzQlchQvYEDh57N1ilzx/G5Gj05oHksuf4nOK/6KGqfoF/sqT9NVI/hFuFzQ8LUFSymIKOpOG9nepF29+TB2bWOgICAgICAgICA",
                      "multi_signature":{
                         "signature":"Qz5rGskoz8xuRLdaAoA5m1He4dBbfg3RBKQ5wmvRper4yTmuKEbbXZ5jidVXYzrJymHcN3xiRYqDSkZ3JbggzWj4NQATsYRSPSc6xP768vAMHA1iNSgxhGV5uW47MSeYihrV9e9YLDjYyzuyUHkBhbWrxMoo8jtowvDMQMZ7qHMhfd",
                         "participants":[
                            "Beta",
                            "Delta",
                            "Gamma"
                         ],
                         "value":{
                            "pool_state_root_hash":"DyMrH7X17UW4k9KcsAUPLKL479dsZ6dvj3bvEAEyYNxZ",
                            "ledger_id":1001,
                            "state_root_hash":"EuHbjY9oaqAXBDxLBM4KcBLASs7RK35maoHjQMbDvmw1",
                            "txn_root_hash":"9i1knJtwTD3NToyCrHoh93HBrTnaq6CeL7F1KtZUBaBz",
                            "timestamp":1530212673
                         }
                    }
                }
           }
        }"#;

        let json_str = CString::new(valid_json).unwrap();
        let json_str_ptr = json_str.as_ptr();

        let mut new_str_ptr = ::std::ptr::null();

        let return_error = get_utxo_state_proof_extractor(json_str_ptr, &mut new_str_ptr);
        assert_eq!(return_error, ErrorCode::Success);

        let expected_parsed_sp: Vec<ParsedSP> = vec![ParsedSP {
            proof_nodes: String::from("+QHF4hOgCBgvwaPO/KIJjOyzhA9dx8yXqPgqKY9sqKPIAZgHujTsgICA2cQggsExxCCCwTGAgICAgICAgICAgICAgICAgICAgICAgICAgICCwTH4VrQAJqUzRQSFdRSktjYXdSeGRXNkdWc2puWkJhMWVjR2RDc3NuN0toV1lKWkdUWGdMN0VzOjoEREJ+KujHB//IMaixsQMlj9+4DLVQHzu4WJczS7X8ED+G2AoMk4QTH20sQPm8C23HQjM7dFR6HIi99DdtySfD9VnTGsoDyHJeCRAIf9srqEpWYrQ1nq9jBE67eMCBK+ewpvMu2UxCCCwTHEIILBMcQggsExxCCCwTHEIILBMcQggsExxCCCwTGAgICAgICA+DnEIILBMcQggsExxCCCwTHEIILBMcQggsExxCCCwTHEIILBMcQggsExxCCCwTHEIILBMYCAgICAgID4cYCAgKDXpPuRat5Zsa2SRHuGjslN7/QaBcZvwSae8dKLWybem4CAoAzQlchQvYEDh57N1ilzx/G5Gj05oHksuf4nOK/6KGqfoF/sqT9NVI/hFuFzQ8LUFSymIKOpOG9nepF29+TB2bWOgICAgICAgICA"),
            root_hash: String::from("EuHbjY9oaqAXBDxLBM4KcBLASs7RK35maoHjQMbDvmw1"),
            kvs_to_verify: KeyValuesInSP::Simple(KeyValueSimpleData { kvs: vec![
                (String::from(base64::encode("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es:4")), Some("1".to_string())),
                (String::from(base64::encode("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es:16")), Some("1".to_string())),
                (String::from(base64::encode("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es:6")), Some("1".to_string())),
                (String::from(base64::encode("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es:17")), Some("1".to_string())),
                (String::from(base64::encode("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es:20")), Some("1".to_string())),
                (String::from(base64::encode("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es:8")), Some("1".to_string())),
                (String::from(base64::encode("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es:13")), Some("1".to_string())),
                (String::from(base64::encode("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es:3")), Some("1".to_string())),
                (String::from(base64::encode("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es:19")), Some("1".to_string())),
                (String::from(base64::encode("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es:11")), Some("1".to_string())),
                (String::from(base64::encode("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es:18")), Some("1".to_string())),
                (String::from(base64::encode("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es:5")), Some("1".to_string())),
                (String::from(base64::encode("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es:15")), Some("1".to_string())),
                (String::from(base64::encode("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es:7")), Some("1".to_string())),
                (String::from(base64::encode("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es:10")), Some("1".to_string())),
                (String::from(base64::encode("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es:9")), Some("1".to_string())),
                (String::from(base64::encode("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es:12")), Some("1".to_string())),
                (String::from(base64::encode("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es:2")), Some("1".to_string())),
                (String::from(base64::encode("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es:21")), Some("1".to_string())),
                (String::from(base64::encode("2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es:14")), Some("1".to_string()))
            ] }),
            multi_signature: json!({
                "participants": ["Beta", "Delta", "Gamma"],
                "signature": "Qz5rGskoz8xuRLdaAoA5m1He4dBbfg3RBKQ5wmvRper4yTmuKEbbXZ5jidVXYzrJymHcN3xiRYqDSkZ3JbggzWj4NQATsYRSPSc6xP768vAMHA1iNSgxhGV5uW47MSeYihrV9e9YLDjYyzuyUHkBhbWrxMoo8jtowvDMQMZ7qHMhfd",
                "value": {
                    "pool_state_root_hash": "DyMrH7X17UW4k9KcsAUPLKL479dsZ6dvj3bvEAEyYNxZ",
                    "ledger_id": 1001,
                    "state_root_hash": "EuHbjY9oaqAXBDxLBM4KcBLASs7RK35maoHjQMbDvmw1",
                    "txn_root_hash": "9i1knJtwTD3NToyCrHoh93HBrTnaq6CeL7F1KtZUBaBz",
                    "timestamp": 1530212673
                }
            }),
        }];

        let json_str = string_from_char_ptr(new_str_ptr).unwrap();
        let parsed_sp: Vec<ParsedSP> = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed_sp.len(), 1);
        assert_eq!(parsed_sp[0].proof_nodes, expected_parsed_sp[0].proof_nodes);
        assert_eq!(parsed_sp[0].root_hash, expected_parsed_sp[0].root_hash);
        assert_eq!(parsed_sp[0].kvs_to_verify, expected_parsed_sp[0].kvs_to_verify);
        assert_eq!(parsed_sp[0].multi_signature, expected_parsed_sp[0].multi_signature);
    }
}