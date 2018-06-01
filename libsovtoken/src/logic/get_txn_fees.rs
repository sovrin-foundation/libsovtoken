use super::responses::ResponseOperations;

/**
Structure for parsing GET_FEES request
*/

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParseGetTxnFeesResponse {
    pub op : ResponseOperation,
    pub protocol_version : i32,
    pub result : ParseGetTxnFeesResult
}


#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParseGetTxnFeesResult {
    identifier : String,
    req_id : i32,
    txn_type : String,
    fees : Vec<(String, i32)>
}