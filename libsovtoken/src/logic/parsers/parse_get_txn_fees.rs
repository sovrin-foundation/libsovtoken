//!
use std::collections::HashMap;

use base64;
use serde_json;
use serde_json::Error;
use ErrorCode;
use libc::c_char;

use logic::parsers::common::{ResponseOperations, StateProof,
                             extract_result_and_state_proof_from_node_reply,
                             KeyValuesInSP, KeyValueSimpleData, ParsedSP};
use utils::json_conversion::JsonDeserialize;
use utils::ffi_support::c_pointer_from_string;
use utils::constants::txn_fields::FEES;
use logic::type_aliases::{ProtocolVersion, TokenAmount, ReqId};

/**
    Structure for parsing GET_FEES request

    # parameters
    op - the operation type received
    protocol_version - the protocol version of the format of the transaction
    result - the payload containing data relevant to the GET_FEES transaction
*/

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParseGetTxnFeesResponse {
    pub op : ResponseOperations,
    pub protocol_version: Option<ProtocolVersion>,
    pub result : ParseGetTxnFeesResult
}

/**
    Structure of the result value within the GET_FEES request

    # parameters
    identifier - The DID this request was submitted from
    req_id - Unique ID number of the request with transaction
    txn_type - the type of transaction that was submitted
    fees - A key:value map with the transaction type as the key and the cost as the value
    state proof - a merkle tree proof used to verify the transaction has been added to the ledger
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParseGetTxnFeesResult {
    pub identifier : String,
    pub req_id : ReqId,
    // This is to change the json key to adhear to the functionality on ledger
    #[serde(rename = "type")]
    pub txn_type : String,
    pub fees : HashMap<String, TokenAmount>,
    // This is being renamed back to the snake case because that is what the JSON object key expects
    #[serde(rename = "state_proof", skip_serializing_if = "Option::is_none")]
    pub state_proof : Option<StateProof>
}

pub fn parse_fees_from_get_txn_fees_response(response : String) -> Result<String, Error> {
    trace!("logic::parsers::parse_fees_from_get_txn_fees_response >> response: {:?}", response);
    let fees_response : ParseGetTxnFeesResponse =
            ParseGetTxnFeesResponse::from_json(&response).map_err(map_err_err!())?;
    let res = serde_json::to_string(&fees_response.result.fees).map_err(map_err_err!());
    trace!("logic::parsers::parse_fees_from_get_txn_fees_response << result: {:?}", res);
    return res;
}

pub fn get_fees_state_proof_extractor(reply_from_node: *const c_char, parsed_sp: *mut *const c_char) -> ErrorCode {
    // TODO: The following errors should have logs
    let (result, state_proof) = match extract_result_and_state_proof_from_node_reply(reply_from_node) {
        Ok((r, s)) => (r, s),
        Err(_) => return ErrorCode::CommonInvalidStructure
    };
    let fees = match result.get(FEES) {
        Some(f) => f.to_owned(),
        None => return ErrorCode::CommonInvalidStructure
    };

    // TODO: Make sure JSON serialisation preserves order
    let kvs_to_verify = KeyValuesInSP::Simple(KeyValueSimpleData {
        kvs: vec![(base64::encode(FEES), Some(fees.to_string()))]
    });
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
            trace!("JSON representation of ParsedSP for get fees {:?}", &s);
            unsafe { *parsed_sp = c_pointer_from_string(s); }
            return ErrorCode::Success;
        },
        Err(_) => return ErrorCode::CommonInvalidStructure
    }
}

#[cfg(test)]
mod parse_fees_responses_test {
    use base64;
    use super::{parse_fees_from_get_txn_fees_response, get_fees_state_proof_extractor,
                ErrorCode, ParsedSP, KeyValuesInSP, KeyValueSimpleData};
    use serde_json::{Value, Error};
    use serde_json;
    use std::ffi::CString;
    use utils::ffi_support::string_from_char_ptr;

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
        assert!(json_error_bool);
    }

    #[test]
    fn test_reply_without_fees() {
        let invalid_json = r#"{ "op" : "REPLY", "result": {"reqId": 83955, "state_proof": {"proof_nodes": "29qFIGZlZXOT0pF7IjEiOjQsIjEwMDAxIjo4fQ==", "root_hash": "5BU5Rc3sRtTJB6tVprGiTSqiRaa9o6ei11MjH4Vu16ms", "multi_signature": {"participants": ["Gamma", "Delta", "Beta"], "value": {"timestamp": 1530059419, "state_root_hash": "5BU5Rc3sRtTJB6tVprGiTSqiRaa9o6ei11MjH4Vu16ms", "ledger_id": 2, "txn_root_hash": "AKboMiJZJm247Sa7GsKQo5Ba8ukgxTQ3DsLc2pyVuDkU", "pool_state_root_hash": "J3ATG63R2JKHDCdpKpQf81FTNyQg2Vgz7Pu1ZHZw6zNy"}, "signature": "Qk67ePVhxdjHivAf8H4Loy1hN5zfb1dq79VSJKYx485EAXmj44PASpp8gj2faysdN8CNzSoUVvXgd3U4P2CA7VkwD7FHKUuviAFJfRQ68FnpUS8hVuqn6PAuv9RGUobohcJnKJ8CVKxr5i3Zn2JNXbk7AqeYRZQ2egq8fdoP3woPW7"}}, "type": "20001", "identifier": "6ouriXMZkLeHsuXrN1X1fd"}}"#;
        let json_str = CString::new(invalid_json).unwrap();
        let json_str_ptr = json_str.as_ptr();

        let mut new_str_ptr = ::std::ptr::null();

        let return_error = get_fees_state_proof_extractor(json_str_ptr, &mut new_str_ptr);
        assert_eq!(return_error, ErrorCode::CommonInvalidStructure);
    }

    #[test]
    fn test_reply_with_fees() {
        let valid_json = r#"{ "op" : "REPLY", "result": {"reqId": 83955, "state_proof": {"proof_nodes": "29qFIGZlZXOT0pF7IjEiOjQsIjEwMDAxIjo4fQ==", "root_hash": "5BU5Rc3sRtTJB6tVprGiTSqiRaa9o6ei11MjH4Vu16ms", "multi_signature": {"participants": ["Gamma", "Delta", "Beta"], "value": {"timestamp": 1530059419, "state_root_hash": "5BU5Rc3sRtTJB6tVprGiTSqiRaa9o6ei11MjH4Vu16ms", "ledger_id": 2, "txn_root_hash": "AKboMiJZJm247Sa7GsKQo5Ba8ukgxTQ3DsLc2pyVuDkU", "pool_state_root_hash": "J3ATG63R2JKHDCdpKpQf81FTNyQg2Vgz7Pu1ZHZw6zNy"}, "signature": "Qk67ePVhxdjHivAf8H4Loy1hN5zfb1dq79VSJKYx485EAXmj44PASpp8gj2faysdN8CNzSoUVvXgd3U4P2CA7VkwD7FHKUuviAFJfRQ68FnpUS8hVuqn6PAuv9RGUobohcJnKJ8CVKxr5i3Zn2JNXbk7AqeYRZQ2egq8fdoP3woPW7"}}, "type": "20001", "identifier": "6ouriXMZkLeHsuXrN1X1fd", "fees": {"1": 4, "10001": 8}}}"#;
        let json_str = CString::new(valid_json).unwrap();
        let json_str_ptr = json_str.as_ptr();

        let mut new_str_ptr = ::std::ptr::null();

        let return_error = get_fees_state_proof_extractor(json_str_ptr, &mut new_str_ptr);
        assert_eq!(return_error, ErrorCode::Success);

        let expected_parsed_sp = vec![ParsedSP {
            proof_nodes: String::from("29qFIGZlZXOT0pF7IjEiOjQsIjEwMDAxIjo4fQ=="),
            root_hash: String::from("5BU5Rc3sRtTJB6tVprGiTSqiRaa9o6ei11MjH4Vu16ms"),
            kvs_to_verify: KeyValuesInSP::Simple(KeyValueSimpleData { kvs: vec![(base64::encode("fees"), Some(json!({"1": 4, "10001": 8}).to_string()))] }),
            multi_signature: json!({
                "participants": ["Gamma", "Delta", "Beta"],
                "value": {"timestamp": 1530059419, "state_root_hash": "5BU5Rc3sRtTJB6tVprGiTSqiRaa9o6ei11MjH4Vu16ms", "ledger_id": 2, "txn_root_hash": "AKboMiJZJm247Sa7GsKQo5Ba8ukgxTQ3DsLc2pyVuDkU", "pool_state_root_hash": "J3ATG63R2JKHDCdpKpQf81FTNyQg2Vgz7Pu1ZHZw6zNy"}, "signature": "Qk67ePVhxdjHivAf8H4Loy1hN5zfb1dq79VSJKYx485EAXmj44PASpp8gj2faysdN8CNzSoUVvXgd3U4P2CA7VkwD7FHKUuviAFJfRQ68FnpUS8hVuqn6PAuv9RGUobohcJnKJ8CVKxr5i3Zn2JNXbk7AqeYRZQ2egq8fdoP3woPW7"
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