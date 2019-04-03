//!

use ErrorCode;
use libc::c_char;
use utils::ffi_support::string_from_char_ptr;
use utils::constants::txn_fields::{RESULT, STATE_PROOF};
use std::str;
use serde_json;
use serde_json::{Value as SJsonValue};
use logic::address;
use logic::type_aliases::{TokenAmount, TxnSeqNo};

/**
    enumeration matches values for the op field in json
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub enum ResponseOperations {
    REPLY,
    REJECT,
    REQNACK,
}


/**
    UTXO is the structure for the data member utxo_json

    used by [`ParsePaymentReply`], [`ParseResponseWithFeesReply`]
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UTXO {
    pub recipient: String,
    pub receipt: String,
    pub amount: TokenAmount,
    pub extra: String,
}

/**
   TXO is the structure for the data member txo of UTXO structure
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TXO {
    pub address: String,
    pub seq_no: TxnSeqNo,
}


impl TXO {
    pub fn to_libindy_string(&self) -> Result<String, ErrorCode> {
        address::txo_to_string(self)
    }

    pub fn from_libindy_string(txo_str: &str) -> Result<Self, serde_json::Error> {
        address::string_to_txo(txo_str)
    }
}

/**
    the nested type "req_signature" in inputs in parse response methods
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequireSignature {
    #[serde(rename = "type")]
    pub sig_type: String,
    pub values: Vec<SignatureValues>,
}

/**
    the nested type "values" in RequiredSignature
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SignatureValues {
    pub from: String,
    pub value: String,
}

/**
    the nested type "tnx_meta_data" in inputs in parse response methods
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMetaData {
    pub seq_no: TxnSeqNo,
    pub txn_time: u32,
}

/**
    Structure of the state proof value within the Result structure

    # parameters
    root_hash - the Merkle root hash of the state trie at the time of response by the ledger
    proof_nodes - the list of hashes necessary to verify the root_hash
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct StateProof {
    pub multi_signature : Option<serde_json::Value>,
    pub root_hash : Option<String>,
    pub proof_nodes : Option<String>
}

/**
 Variants of representation for items to verify against SP Trie
 Right now 2 options are specified:
 - simple array of key-value pair
 - whole subtrie
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum KeyValuesInSP {
    Simple(KeyValueSimpleData),
    SubTrie(KeyValuesSubTrieData),
}

/**
 Subtrie variant of `KeyValuesInSP`.

 In this case Client (libindy) should construct subtrie and append it into trie based on `proof_nodes`.
 After this preparation each kv pair can be checked.
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct KeyValuesSubTrieData {
    /// base64-encoded common prefix of each pair in `kvs`. Should be used to correct merging initial trie and subtrie
    pub sub_trie_prefix: Option<String>,
    pub kvs: Vec<(String /* b64-encoded key_suffix */, Option<String /* val */>)>,
}

/**
 Simple variant of `KeyValuesInSP`.

 All required data already present in parent SP Trie (built from `proof_nodes`).
 `kvs` can be verified directly in parent trie
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct KeyValueSimpleData {
    pub kvs: Vec<(String /* key */, Option<String /* val */>)>
}

/**
 Single item to verification:
 - SP Trie with RootHash
 - BLS MS
 - set of key-value to verify
*/

#[derive(Serialize, Deserialize, Debug)]
    pub struct ParsedSP {
    /// encoded SP Trie transferred from Node to Client
    pub proof_nodes: String,
    /// RootHash of the Trie, start point for verification. Should be same with appropriate filed in BLS MS data
    pub root_hash: String,
    /// entities to verification against current SP Trie
    pub kvs_to_verify: KeyValuesInSP,
    /// BLS MS data for verification
    pub multi_signature: serde_json::Value,
}

pub fn extract_result_and_state_proof_from_node_reply(reply_from_node: *const c_char) -> Result<(SJsonValue, StateProof), ErrorCode> {
    // TODO: The following errors should have messages
    let reply = match string_from_char_ptr(reply_from_node) {
        Some(r) => r,
        None => return Err(ErrorCode::CommonInvalidStructure)
    };

    let json_reply: SJsonValue = serde_json::from_str::<SJsonValue>(&reply)
        .or(Err(ErrorCode::CommonInvalidStructure))?;

    let result: SJsonValue = match json_reply.get(RESULT) {
        Some(r) => r.clone(),
        None => return Err(ErrorCode::CommonInvalidStructure)
    };

    let state_proof = match result.get(STATE_PROOF) {
        Some(sp) => sp.to_owned(),
        None => return Err(ErrorCode::CommonInvalidStructure)
    };

    match serde_json::from_value(state_proof) {
        Ok(s) => Ok((result, s)),
        Err(_) => Err(ErrorCode::CommonInvalidStructure)
    }
}

#[cfg(test)]
mod common_tests {
    use super::*;
    use std::ffi::CString;

    pub fn test_invalid_json(invalid_json: &str) {
        let json_str = CString::new(invalid_json).unwrap();
        let json_str_ptr = json_str.as_ptr();

        let return_error = extract_result_and_state_proof_from_node_reply(
            json_str_ptr).unwrap_err();

        assert_eq!(return_error, ErrorCode::CommonInvalidStructure);
    }

    #[test]
    fn test_extraction_with_invalid_reply_json() {
        let invalid_json = r#"{ "some_key : "value"}"#;
        test_invalid_json(invalid_json);
    }

    #[test]
    fn test_extraction_with_result_absent() {
        let invalid_json = r#"{ "op" : "REPLY"}"#;
        test_invalid_json(invalid_json);
    }

    #[test]
    fn test_extraction_with_state_proof_absent() {
        let invalid_json = r#"{ "op" : "REPLY", "result": {"reqId": 83955, "type": "20001", "identifier": "6ouriXMZkLeHsuXrN1X1fd", "fees": {"1": 4, "10001": 8}}}"#;
        test_invalid_json(invalid_json);
    }

    #[test]
    fn test_extraction_with_state_proof_present_with_insufficient_keys() {
        // Remove key `proof_nodes` from `state_proof`
        let json1 = r#"{ "op" : "REPLY", "result": {"reqId": 83955, "state_proof": {"root_hash": "5BU5Rc3sRtTJB6tVprGiTSqiRaa9o6ei11MjH4Vu16ms", "multi_signature": {"participants": ["Gamma", "Delta", "Beta"], "value": {"timestamp": 1530059419, "state_root_hash": "5BU5Rc3sRtTJB6tVprGiTSqiRaa9o6ei11MjH4Vu16ms", "ledger_id": 2, "txn_root_hash": "AKboMiJZJm247Sa7GsKQo5Ba8ukgxTQ3DsLc2pyVuDkU", "pool_state_root_hash": "J3ATG63R2JKHDCdpKpQf81FTNyQg2Vgz7Pu1ZHZw6zNy"}, "signature": "Qk67ePVhxdjHivAf8H4Loy1hN5zfb1dq79VSJKYx485EAXmj44PASpp8gj2faysdN8CNzSoUVvXgd3U4P2CA7VkwD7FHKUuviAFJfRQ68FnpUS8hVuqn6PAuv9RGUobohcJnKJ8CVKxr5i3Zn2JNXbk7AqeYRZQ2egq8fdoP3woPW7"}}, "type": "20001", "identifier": "6ouriXMZkLeHsuXrN1X1fd", "fees": {"1": 4, "10001": 8}}}"#;
        let json_str = CString::new(json1).unwrap();
        let json_str_ptr = json_str.as_ptr();
        let (_, state_proof) = extract_result_and_state_proof_from_node_reply(json_str_ptr).unwrap();
        assert!(state_proof.multi_signature.is_some());
        assert!(state_proof.root_hash.is_some());
        assert!(state_proof.proof_nodes.is_none());

        // Remove key `root_hash` from `state_proof`
        let json2 = r#"{ "op" : "REPLY", "result": {"reqId": 83955, "state_proof": {"multi_signature": {"participants": ["Gamma", "Delta", "Beta"], "value": {"timestamp": 1530059419, "state_root_hash": "5BU5Rc3sRtTJB6tVprGiTSqiRaa9o6ei11MjH4Vu16ms", "ledger_id": 2, "txn_root_hash": "AKboMiJZJm247Sa7GsKQo5Ba8ukgxTQ3DsLc2pyVuDkU", "pool_state_root_hash": "J3ATG63R2JKHDCdpKpQf81FTNyQg2Vgz7Pu1ZHZw6zNy"}, "signature": "Qk67ePVhxdjHivAf8H4Loy1hN5zfb1dq79VSJKYx485EAXmj44PASpp8gj2faysdN8CNzSoUVvXgd3U4P2CA7VkwD7FHKUuviAFJfRQ68FnpUS8hVuqn6PAuv9RGUobohcJnKJ8CVKxr5i3Zn2JNXbk7AqeYRZQ2egq8fdoP3woPW7"}}, "type": "20001", "identifier": "6ouriXMZkLeHsuXrN1X1fd", "fees": {"1": 4, "10001": 8}}}"#;
        let json_str = CString::new(json2).unwrap();
        let json_str_ptr = json_str.as_ptr();
        let (_, state_proof) = extract_result_and_state_proof_from_node_reply(json_str_ptr).unwrap();
        assert!(state_proof.multi_signature.is_some());
        assert!(state_proof.root_hash.is_none());
        assert!(state_proof.proof_nodes.is_none());
    }

    #[test]
    fn test_extraction_with_valid_state_proof() {
        let valid_json = r#"{ "op" : "REPLY", "result": {"reqId": 83955, "state_proof": {"proof_nodes": "29qFIGZlZXOT0pF7IjEiOjQsIjEwMDAxIjo4fQ==", "root_hash": "5BU5Rc3sRtTJB6tVprGiTSqiRaa9o6ei11MjH4Vu16ms", "multi_signature": {"participants": ["Gamma", "Delta", "Beta"], "value": {"timestamp": 1530059419, "state_root_hash": "5BU5Rc3sRtTJB6tVprGiTSqiRaa9o6ei11MjH4Vu16ms", "ledger_id": 2, "txn_root_hash": "AKboMiJZJm247Sa7GsKQo5Ba8ukgxTQ3DsLc2pyVuDkU", "pool_state_root_hash": "J3ATG63R2JKHDCdpKpQf81FTNyQg2Vgz7Pu1ZHZw6zNy"}, "signature": "Qk67ePVhxdjHivAf8H4Loy1hN5zfb1dq79VSJKYx485EAXmj44PASpp8gj2faysdN8CNzSoUVvXgd3U4P2CA7VkwD7FHKUuviAFJfRQ68FnpUS8hVuqn6PAuv9RGUobohcJnKJ8CVKxr5i3Zn2JNXbk7AqeYRZQ2egq8fdoP3woPW7"}}, "type": "20001", "identifier": "6ouriXMZkLeHsuXrN1X1fd", "fees": {"1": 4, "10001": 8}}}"#;
        let json_str = CString::new(valid_json).unwrap();
        let json_str_ptr = json_str.as_ptr();

        let (result, state_proof) = extract_result_and_state_proof_from_node_reply(
            json_str_ptr).unwrap();

        let expected_state_proof = StateProof {
            multi_signature : Some(json!({
                "participants": ["Gamma", "Delta", "Beta"],
                "value": {
                    "timestamp": 1530059419,
                    "state_root_hash": "5BU5Rc3sRtTJB6tVprGiTSqiRaa9o6ei11MjH4Vu16ms",
                    "ledger_id": 2,
                    "txn_root_hash": "AKboMiJZJm247Sa7GsKQo5Ba8ukgxTQ3DsLc2pyVuDkU",
                    "pool_state_root_hash": "J3ATG63R2JKHDCdpKpQf81FTNyQg2Vgz7Pu1ZHZw6zNy",
                },
                "signature": "Qk67ePVhxdjHivAf8H4Loy1hN5zfb1dq79VSJKYx485EAXmj44PASpp8gj2faysdN8CNzSoUVvXgd3U4P2CA7VkwD7FHKUuviAFJfRQ68FnpUS8hVuqn6PAuv9RGUobohcJnKJ8CVKxr5i3Zn2JNXbk7AqeYRZQ2egq8fdoP3woPW7"
            })),
            root_hash : Some(String::from("5BU5Rc3sRtTJB6tVprGiTSqiRaa9o6ei11MjH4Vu16ms")),
            proof_nodes : Some(String::from("29qFIGZlZXOT0pF7IjEiOjQsIjEwMDAxIjo4fQ=="))
        };

        let expected_result = json!({
            "reqId": 83955,
            "type": "20001",
            "identifier": "6ouriXMZkLeHsuXrN1X1fd",
            "fees": {"1": 4, "10001": 8}
        });

        assert_eq!(expected_state_proof.multi_signature, state_proof.multi_signature);
        assert_eq!(expected_state_proof.root_hash, state_proof.root_hash);
        assert_eq!(expected_state_proof.proof_nodes, state_proof.proof_nodes);
        assert_eq!(expected_result.get("reqId").unwrap(), result.get("reqId").unwrap());
        assert_eq!(expected_result.get("type").unwrap(), result.get("type").unwrap());
        assert_eq!(expected_result.get("identifier").unwrap(), result.get("identifier").unwrap());
        assert_eq!(expected_result.get("fees").unwrap(), result.get("fees").unwrap());
    }
}