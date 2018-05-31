//!
//!
use serde::Serialize;
use serde_json;
use super::responses::ResponseOperations;

/**
*/
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseGetUtxoResponseResultOutput {
    pub huh: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseGetUtxoResponseResult {
    #[serde(rename = "type")]
    pub tnx_type : String,
    pub address : String,
    pub identifier: String,
    pub req_id: u32,
    pub outputs : Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseGetUtxoResponse {
    pub op : ResponseOperations,
    pub protocol_version: i32,
    pub result : ParseGetUtxoResponseResult,
}

/**

*/
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TXO {
    pub address: String,
    pub seq_no: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UTXO {
    pub payment_address: String,
    pub txo: TXO,
    pub amount: u32,
    pub extra: String,
}

#[derive(Serialize, Deserialize)]
pub struct ParseGetUtxoReply {
    pub ver : i32,
    pub utxo_json : Vec<UTXOJson>,
}