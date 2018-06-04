//! types used for parse_payment_response_handler

use logic::responses::ResponseOperations;

/**
    for parse_payment_response_handler input resp_json
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParsePaymentResponse {
    pub op : ResponseOperations,
    pub result : ParsePaymentResponseResult,
}

/**
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParsePaymentResponseResult {
    pub identifier: String,
    #[serde(rename = "type")]
    pub txn_type: String,
    pub seq_no: i32,
    pub tnx_time: i32,
    pub signature: Option<String>,
    pub signatures: Option<String>,
    pub extra: Option<String>,
    pub req_id: i32,
    pub inputs: Vec<(String, i32, String)>,
    pub outputs: Vec<(String, i32)>,
    pub root_hash: String,
    pub audit_path: Vec<String>
}


/**
    for parse_payment_response_handler output utxo_json
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ParsePaymentReply {
    pub ver : i32,
    pub utxo_json : Vec<UTXO>,
}

/**
    UTXO is the structure for the data member utxo_json for the
    ParsePaymentReply type
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UTXO {
    pub payment_address: String,
    pub txo: TXO,
    pub amount: i32,
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

impl ParsePaymentReply {
    /**
        Converts ParsePaymentReply (which should be input via indy-sdk) to ParsePaymentReply
        please note:  use of this function moves ParsePaymentResponse and it cannot be used again
        after this call
    */
    pub fn from_response(base : ParsePaymentResponse) -> ParsePaymentReply {
        let mut utxos: Vec<UTXO> = vec![];

        for unspent_output in base.result.outputs {

            let (address, amount) = unspent_output;

            let txo: TXO = TXO { address: address.to_string(), seq_no: 1 };
            let utxo: UTXO = UTXO { payment_address: address, txo, amount, extra: "".to_string() };

            utxos.push(utxo);
        }

        let reply: ParsePaymentReply = ParsePaymentReply { ver : 1, utxo_json : utxos};
        return reply;
    }
}


#[cfg(test)]
mod parse_get_utxo_response_tests {

}