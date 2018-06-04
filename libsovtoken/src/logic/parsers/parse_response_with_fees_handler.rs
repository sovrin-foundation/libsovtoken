//! types used for parse_payment_response_handler
#![allow(unused_variables)]
#![allow(unused_imports)]

use logic::input::Inputs;
use logic::output::{Outputs, Output};
/**
    for parse_response_with_fees_handler input resp_json
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParseResponseWithFees {
    pub fees: (Inputs, Outputs, i32),
}


/**
    for parse_response_with_fees_handler output utxo_json
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ParseResponseWithFeesReply {
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


impl ParseResponseWithFeesReply {
    /**
        Converts ParsePaymentReply (which should be input via indy-sdk) to ParsePaymentReply
        please note:  use of this function moves ParsePaymentResponse and it cannot be used again
        after this call
    */
    pub fn from_response(base : ParseResponseWithFees) -> ParseResponseWithFeesReply {
        let mut utxos: Vec<UTXO> = vec![];

        let outputs: Outputs = base.fees.1;
        let seq_no: i32 = base.fees.2;

        for output in outputs {

            println!("output -> {:?}", output);

            let txo: TXO = TXO { address: output.address.to_string(), seq_no };
            let utxo: UTXO = UTXO { payment_address: output.address.to_string(), txo, amount : output.amount, extra: "".to_string()};

            utxos.push(utxo);
        }

        let reply: ParseResponseWithFeesReply = ParseResponseWithFeesReply { ver : 1, utxo_json : utxos};
        return reply;
    }
}

#[cfg(test)]
mod parse_response_with_fees_handler_tests {
    #[allow(unused_imports)]

    use utils::json_conversion::{JsonDeserialize, JsonSerialize};
    use utils::random::{rand_req_id, rand_string};
    use super::*;

    // "fees" : [ [ (3) ], [ (2) ], int ]

    static PARSE_RESPONSE_WITH_FEES_JSON: &'static str = r#"{
                "fees": [
                    [
                        ["QEb3MVVWv1McB8YpgXAvj8SbZDLRRHaPpWt9jFMgfRss3CYBH", 2, "5Z7ktpfVQAhj2gMFR8L6JnG7fQQJzqWwqrDgXQP1CYf2vrjKPe2a27borFVuAcQh2AttoejgAoTzJ36wfyKxu5ox"]
                    ],
                    [
                        ["2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC", 11]
                    ],
                    3
                ]
            }"#;

    #[test]
    fn success_json_to_parse_response_with_fees() {
        let response: ParseResponseWithFees = ParseResponseWithFees::from_json(PARSE_RESPONSE_WITH_FEES_JSON).unwrap();
    }

    #[test]
    fn success_parse_response_with_fees_to_reply() {
        let response: ParseResponseWithFees = ParseResponseWithFees::from_json(PARSE_RESPONSE_WITH_FEES_JSON).unwrap();
        let reply: ParseResponseWithFeesReply = ParseResponseWithFeesReply::from_response(response);
    }
}