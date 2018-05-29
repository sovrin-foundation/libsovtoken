/*!
 *  Defines structure and implementation Fees and SetFeesRequest
 *  these are the structures for the [`build_fees_txn_handler`]
 *
 *  [`build_fees_txn_handler`]: ../../../api/fn.build_fees_txn_handler.html
 */
use std::collections::HashMap;
use logic::request::Request;

const SET_FEES: &str = "20000";

/**
 *  Json config to customize [`build_fees_txn_handler`]
 *
 *  [`build_fees_txn_handler`]: ../../../api/fn.build_fees_txn_handler.html
 */
#[derive(Serialize, Deserialize)]
pub struct Fees {
    pub  fees: HashMap<String, u64>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct getFeesRequest {
    #[serde(rename = "type")]
    txn_type: &'static str,
    fees:  HashMap<String, u64>,
}