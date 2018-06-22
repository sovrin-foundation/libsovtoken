//! types used for parse_response_with_fees_handler
#![allow(unused_variables)]
#![allow(unused_imports)]

use logic::parsers::common::{UTXO, TXO};
use logic::input::Inputs;
use logic::output::{Outputs, Output};

/**
    for parse_response_with_fees_handler input resp_json

    used in ['parse_response_with_fees_handler']
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


impl ParseResponseWithFeesReply {
    /**
        Converts ParseResponseWithFees (which should be input via indy-sdk) to ParseResponseWithFeesReply
        please note:  use of this function moves ParseResponseWithFees and it cannot be used again
        after this call
    */
    pub fn from_response(base : ParseResponseWithFees) -> ParseResponseWithFeesReply {
        let mut utxos: Vec<UTXO> = vec![];

        // according to the documentation, don't need the inputs.  Only the outputs
        // and seq_no which are part 2 and 3 of the tuple
        let outputs: Outputs = base.fees.1;
        let seq_no: i32 = base.fees.2;

        for output in outputs {
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

    use logic::address::{ADDRESS_LEN, VERKEY_LEN, CHECKSUM_LEN};
    use utils::json_conversion::{JsonDeserialize, JsonSerialize};
    use utils::random::{rand_req_id, rand_string};
    use super::*;

    // "fees" : [ [ (3) ], [ (2) ], int ]

    static PARSE_RESPONSE_WITH_FEES_JSON: &'static str = r#"{
                "fees": [
                    [
                        ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 2, "5Z7ktpfVQAhj2gMFR8L6JnG7fQQJzqWwqrDgXQP1CYf2vrjKPe2a27borFVuAcQh2AttoejgAoTzJ36wfyKxu5ox"]
                    ],
                    [
                        ["2s6tmsmPaZG2pXgD7AG7YCyXtfFd5s6Ro2MXCcKhAC94JFYaq1", 11]
                    ],
                    3
                ]
            }"#;

    static PARSE_RESPONSE_WITH_MULTIPLE_FEES_JSON: &'static str = r#"{
                "fees": [
                    [
                        ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 2, "5Z7ktpfVQAhj2gMFR8L6JnG7fQQJzqWwqrDgXQP1CYf2vrjKPe2a27borFVuAcQh2AttoejgAoTzJ36wfyKxu5ox"]
                    ],
                    [
                        ["2s6tmsmPaZG2pXgD7AG7YCyXtfFd5s6Ro2MXCcKhAC94JFYaq1", 11],
                        ["H4NWNV3GutBRcxgkGoomhwnFtqiiMG1HF45avHtJXyspCwQMb", 10]
                    ],
                    3
                ]
            }"#;

    // Tests that valid json with one element in the "output section" is serialized to ParseResponseWithFees tyoe
    #[test]
    fn success_json_to_parse_response_with_fees() {
        let response: ParseResponseWithFees = ParseResponseWithFees::from_json(PARSE_RESPONSE_WITH_FEES_JSON).unwrap();

        // only going to test outputs since we don't use inputs
        let outputs: Outputs = response.fees.1;

        assert_eq!(1, outputs.len());
        assert_eq!(3, response.fees.2);
    }

    // Tests that valid json with multiple elements in the "output section" is serialized to ParseResponseWithFees tyoe
    #[test]
    fn success_json_to_parse_response_with_multiple_fees() {
        let response: ParseResponseWithFees = ParseResponseWithFees::from_json(PARSE_RESPONSE_WITH_MULTIPLE_FEES_JSON).unwrap();

        // only going to test outputs since we don't use inputs
        let outputs: Outputs = response.fees.1;

        assert_eq!(2, outputs.len());
        assert_eq!(3, response.fees.2);
    }

    // Tests that valid json with one element in the "output section" is correctly converted to ParseResponseWithFeesReply
    // through the ParseResponseWithFeesReply::from_response method
    #[test]
    fn success_parse_response_with_fees_to_reply() {
        let response: ParseResponseWithFees = ParseResponseWithFees::from_json(PARSE_RESPONSE_WITH_FEES_JSON).unwrap();

        let reply: ParseResponseWithFeesReply = ParseResponseWithFeesReply::from_response(response);

        assert_eq!(1, reply.utxo_json.len());

        for utxo in reply.utxo_json {
            let amount: u32 = utxo.amount;
            let mut found_address: bool = false;

            // if this next statement is outside the prior for, there is a move error.
            // yes this is a cheat but its a unit test function...
            let outputs: Vec<Output> = ParseResponseWithFees::from_json(PARSE_RESPONSE_WITH_FEES_JSON).unwrap().fees.1.to_vec();

            for output in outputs {
                if utxo.payment_address == output.address {
                    found_address = true;
                    assert_eq!(amount, output.amount, "amounts did not match in reply (ParseResponseWithFeesReply)");
                }
            }
            assert_eq!(true, found_address, "did not find address reply (ParseResponseWithFeesReply)");
        }

    }

    // Tests that valid json with many elements in the "output section" is correctly converted to ParseResponseWithFeesReply
    // through the ParseResponseWithFeesReply::from_response method
    #[test]
    fn success_parse_response_with_multiple_fees_to_reply() {
        let response: ParseResponseWithFees = ParseResponseWithFees::from_json(PARSE_RESPONSE_WITH_MULTIPLE_FEES_JSON).unwrap();
        let reply: ParseResponseWithFeesReply = ParseResponseWithFeesReply::from_response(response);

        assert_eq!(2, reply.utxo_json.len());

        for utxo in reply.utxo_json {
            let amount: u32 = utxo.amount;
            let mut found_address: bool = false;

            // if this next statement is outside the prior for, there is a move error.
            // yes this is a cheat but its a unit test function...
            let outputs: Vec<Output> = ParseResponseWithFees::from_json(PARSE_RESPONSE_WITH_MULTIPLE_FEES_JSON).unwrap().fees.1.to_vec();

            for output in outputs {
                if utxo.payment_address == output.address {
                    found_address = true;
                    assert_eq!(amount, output.amount, "amounts did not match in reply (ParseResponseWithFeesReply)");
                }
            }

            assert_eq!(true, found_address, "did not find address reply (ParseResponseWithFeesReply)");
        }
    }
}