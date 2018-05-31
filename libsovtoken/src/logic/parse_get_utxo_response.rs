//!
//!

use utils::json_conversion::JsonSerialize;

use super::responses::ResponseOperations;

/**
    for parse_get_utxo_response_handler input parameter resp_json
*/
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseGetUtxoResponse {
    pub op : ResponseOperations,
    pub protocol_version: i32,
    pub result : ParseGetUtxoResponseResult,
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
pub struct ParseGetUtxoResponseResultOutput {
    pub huh: String,
}

impl ParseGetUtxoResponse {
    pub fn from_json(json: String) -> ParseGetUtxoResponse {
        unimplemented!()
    }
}

/**
   for parse_get_utxo_response_handler output parameter utxo_json
*/
#[derive(Serialize, Deserialize)]
pub struct ParseGetUtxoReply {
    pub ver : i32,
    pub utxo_json : Vec<UTXO>,
}

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

impl ParseGetUtxoReply {
    pub fn from_response(base : ParseGetUtxoResponse) -> ParseGetUtxoReply {
        unimplemented!()
    }
}