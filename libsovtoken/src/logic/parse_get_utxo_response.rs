//!
//!

use super::responses::ResponseOperations;

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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParseGetUtxoResponseResult {
    #[serde(rename = "type")]
    pub tnx_type : String,
    pub address : String,
    pub identifier: String,
    pub req_id: u32,
    pub outputs : Vec<(String, i32, i32)>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParseGetUtxoResponseResultOutput {
    pub address: String,
    pub seq_no: i32,
    pub amount: i32,
}

impl ParseGetUtxoResponseResultOutput {
    /**
        per https://github.com/evernym/libsovtoken/blob/master/doc/data_structures.md
        outputs is sent to libsovtoken as "<str: address>", <int: sequence number>, <int: amount>

        this method converts the string into ParseGetUtxoResponseResultOutput so that its easier to
        work with
    */
    pub fn from_string(string : String ) -> ParseGetUtxoResponseResultOutput  {

        let parts: Vec<&str> = string.split(",").collect::<Vec<&str>>();;
        let address: String = parts[0].to_string();
        let seq_no: i32 = parts[1].to_string().parse::<i32>().unwrap();
        let amount: i32 = parts[3].to_string().parse::<i32>().unwrap();

        return ParseGetUtxoResponseResultOutput {
            address,
            seq_no,
            amount
        };
    }
}

/**
   for parse_get_utxo_response_handler output parameter utxo_json
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ParseGetUtxoReply {
    pub ver : i32,
    pub utxo_json : Vec<UTXO>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UTXO {
    pub payment_address: String,
    pub txo: TXO,
    pub amount: i32,
    pub extra: String,
}

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

            let txo: TXO = TXO { address : base.result.address.to_string(), seq_no };
            let utxo: UTXO = UTXO { payment_address: base.result.address.to_string(), txo, amount, extra: "".to_string() };

            utxos.push(utxo);
        }

        let reply: ParseGetUtxoReply = ParseGetUtxoReply { ver : 1, utxo_json : utxos};
        return reply;
    }
}

#[cfg(test)]
mod parse_get_uto_responses_tests {

    #[test]
    fn success_parse_get_utxo_response_result_output() {

    }

}