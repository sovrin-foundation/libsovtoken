//! types used for parse_get_utxo_response_handler

use logic::responses::ResponseOperations;

/**
    for parse_get_utxo_response_handler input parameter resp_json
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParseGetUtxoResponse {
    pub op : ResponseOperations,
    pub protocol_version: i32,
    pub result : ParseGetUtxoResponseResult,
}

/**
    ParseGetUtxoResponseResult is the structure for the result
    member of ParseGetUtxoResponse
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParseGetUtxoResponseResult {
    #[serde(rename = "type")]
    pub txn_type : String,
    pub address : String,
    pub identifier: String,
    pub req_id: u32,
    pub outputs : Vec<(String, i32, i32)>,
}

/**
   for parse_get_utxo_response_handler output parameter utxo_json
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ParseGetUtxoReply {
    pub ver : i32,
    pub utxo_json : Vec<UTXO>,
}

/**
    UTXO is the structure for the data member utxo_json for the
    ParseGetUtxoReply type
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

impl ParseGetUtxoReply {
    /**
        Converts ParseGetUtxoResponse (which should be input via indy-sdk) to ParseGetUtxoReply
        please note:  use of this function moves ParseGetUtxoResponse and it cannot be used again
        after this call
    */
    pub fn from_response(base : ParseGetUtxoResponse) -> ParseGetUtxoReply {
        let mut utxos: Vec<UTXO> = vec![];

        for unspent_output in base.result.outputs {

            let (address, seq_no, amount) = unspent_output;

            let txo: TXO = TXO { address, seq_no };
            let utxo: UTXO = UTXO { payment_address: base.result.address.to_string(), txo, amount, extra: "".to_string() };

            utxos.push(utxo);
        }

        let reply: ParseGetUtxoReply = ParseGetUtxoReply { ver : 1, utxo_json : utxos};
        return reply;
    }
}

#[cfg(test)]
mod parse_get_uto_responses_tests {

    use logic::responses::ResponseOperations;
    use utils::random::{rand_req_id, rand_string};
    use super::*;

    #[test]
    fn success_parse_get_utxo_reply_from_response() {

        let address: String = rand_string(32);
        let identifier: String = rand_req_id().to_string();
        let mut outputs: Vec<(String, i32, i32)> = Vec::new();

        outputs.push((rand_string(32), 1, 10));
        outputs.push((rand_string(32), 2, 20));

        let outputs_len: usize = outputs.len();

        let result: ParseGetUtxoResponseResult = ParseGetUtxoResponseResult {
            txn_type : "1002".to_string(),
            address,
            identifier,
            req_id: 123457890,
            outputs
        };

        let response: ParseGetUtxoResponse = ParseGetUtxoResponse {
            op : ResponseOperations::REPLY,
            protocol_version: 1,
            result
        };

        let reply: ParseGetUtxoReply = ParseGetUtxoReply::from_response(response);

        assert_eq!(outputs_len, reply.utxo_json.len());

    }

    #[test]
    fn success_parse_get_utxo_reply_from_response_with_empty_outputs() {
        let address: String = rand_string(32);
        let identifier: String = rand_req_id().to_string();
        let outputs: Vec<(String, i32, i32)> = Vec::new();

        let outputs_len: usize = outputs.len();

        let result: ParseGetUtxoResponseResult = ParseGetUtxoResponseResult {
            txn_type : "1002".to_string(),
            address,
            identifier,
            req_id: 123457890,
            outputs
        };

        let response: ParseGetUtxoResponse = ParseGetUtxoResponse {
            op : ResponseOperations::REPLY,
            protocol_version: 1,
            result
        };

        let reply: ParseGetUtxoReply = ParseGetUtxoReply::from_response(response);

        assert_eq!(outputs_len, reply.utxo_json.len());
    }
}