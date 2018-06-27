//!
use indy::ErrorCode;
use rust_base58::{FromBase58, ToBase58};
use std::str;
use serde_json;
use utils::json_conversion::*;
use std::io;

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

    used by [`ParsePaymentReply`], [`ParseGetUtxoReply`], [`ParseResponseWithFeesReply`]
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UTXO {
    pub payment_address: String,
    pub txo: String,
    pub amount: u32,
    pub extra: String,
}

/**
   TXO is the structure for the data member txo of UTXO structure
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TXO {
    pub address: String,
    pub seq_no: i32,
}

pub static TXO_IDENTIFIER: &str = "txo:sov:";

impl TXO {
    pub fn to_libindy_string(&self) -> Result<String, ErrorCode> {
        let temp = self.to_json()
            .map_err(|_| ErrorCode::CommonInvalidState)?
            .as_bytes().to_base58_check();
        Ok(TXO_IDENTIFIER.to_string() + &temp)
    }

    pub fn from_libindy_string(txo_str: &str) -> Result<Self, serde_json::Error> {
        let json_u8 = txo_str.replace(TXO_IDENTIFIER, "").from_base58_check()
            .map_err(|_| serde_json::Error::io(io::ErrorKind::InvalidInput.into()))?;
        let json = str::from_utf8(&json_u8)
            .map_err(|_| serde_json::Error::io(io::ErrorKind::InvalidInput.into()))?;
        TXO::from_json(json)
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
    pub seq_no: i32,
    pub txn_time: u32,
}

/* 
    For the state_proof json structure inside the result json structure
 */
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct StateProof {
    pub multi_signature : MultiSig,
    pub proof_nodes : String,
    pub root_hash : String
}

/* 
    For multi_signature inside the state_proof json structure
 */
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct MultiSig {
    pub participants : Vec<String>,
    pub signature : String,
    pub value : MultiSigValue
}

/* 
    For value structure inside the multi_signature json
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct MultiSigValue {
    pub ledger_id : i32,
    pub pool_state_root_hash : String,
    pub state_root_hash : String,
    pub timestamp : i64,
    pub txn_root_hash : String
}


#[cfg(test)]
mod common_tests {
    use super::*;
    use utils::json_conversion::{JsonDeserialize};

    #[test]
    fn success_parse_multisigvalue_struct() {
        /* fill multisig value using example from data_structures.md */
        let value = r#"{
            "ledger_id": 1001,
            "pool_state_root_hash" : "9i3acxaDhCfx9jWXW2JZRoDWzRQEKo7bPBVN7VPE1Jhg",
            "state_root_hash" : "8tJkWdp9wdz3bpb5s5hPDfrjWCQTPmsFKrSdoPmTTnea",
            "timestamp" : 1529705683,
            "txn_root_hash" : "67khbUNo8rySwEtW2SPSsyK4rmLCS7JAN4kYnppELajc"
        }"#;

        let multisigval : MultiSigValue = MultiSigValue::from_json(value).unwrap();
        
        /* compare values of fields in json string deserialize as expected */
        assert_eq!( 1001 , multisigval.ledger_id);
        assert_eq!( "9i3acxaDhCfx9jWXW2JZRoDWzRQEKo7bPBVN7VPE1Jhg" , multisigval.pool_state_root_hash);
        assert_eq!( "8tJkWdp9wdz3bpb5s5hPDfrjWCQTPmsFKrSdoPmTTnea" , multisigval.state_root_hash);
        assert_eq!( 1529705683 , multisigval.timestamp);
        assert_eq!( "67khbUNo8rySwEtW2SPSsyK4rmLCS7JAN4kYnppELajc" , multisigval.txn_root_hash);
    }
    
    #[test]
    fn fail_parse_multisigvalue_struct() {
        let value = r#"{
            "ledger_id": 1001,
            "pool_state_root_hash" : "9i3acxaDhCfx9jWXW2JZRoDWzRQEKo7bPBVN7VPE1Jhg",
            "state_root_hash" : "8tJkWdp9wdz3bpb5s5hPDfrjWCQTPmsFKrSdoPmTTnea",
            "timestamp" : NOT_VALID_JSON,
            "txn_root_hash" : "67khbUNo8rySwEtW2SPSsyK4rmLCS7JAN4kYnppELajc"
        }"#;

        let multisigval = MultiSigValue::from_json(value);
        let json_error_bool: bool = multisigval.is_err();
        assert!(json_error_bool);
    }
}