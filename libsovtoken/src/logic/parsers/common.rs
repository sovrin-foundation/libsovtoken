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
            .map_err(|e| serde_json::Error::io(io::ErrorKind::InvalidInput.into()))?;
        let json = str::from_utf8(&json_u8)
            .map_err(|e| serde_json::Error::io(io::ErrorKind::InvalidInput.into()))?;
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
