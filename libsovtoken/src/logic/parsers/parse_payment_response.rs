//! types used for parse_payment_response_handler
#![allow(unused_variables)]
#![allow(unused_imports)]

use indy::ErrorCode;
use logic::address::add_qualifer_to_address;
use logic::parsers::common::{ResponseOperations,
                             UTXO,
                             TXO,
                             TransactionMetaData,
                             RequireSignature,
                             SignatureValues};
use utils::json_conversion::JsonSerialize;

/**
    for parse_payment_response_handler input resp_json
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParsePaymentResponse {
    pub op : ResponseOperations,
    pub protocol_version: i32,
    pub result : ParsePaymentResponseResult,
}

/**
    The nested type named "result in ParsePaymentResponse
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParsePaymentResponseResult {
    pub txn: Transaction,
    pub req_signature: RequireSignature,
    #[serde(rename = "txnMetadata")]
    pub tnx_meta_data: TransactionMetaData,
    pub ver: String,
    pub audit_path: Vec<String>,
    pub root_hash: String,
}

/**
    the nested type "tnx" in ParsePaymentResponseResult
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    #[serde(rename = "type")]
    pub txn_type : String,
    pub protocol_version : i32,
    #[serde(rename = "metadata")]
    pub meta_data: TransactionMetaData2,
    pub data: TransactionData,
}

/**
   the nested type "data" in Transaction
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionData {
    pub extra: Option<String>,
    pub inputs : Vec<(String, i32)>,
    pub outputs: Vec<(String, u32)>,
}

/**
    the nested type "meta_data" in Transaction
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMetaData2 {
    pub digest: String,
    pub from: String,
    pub req_id: i64
}


/**
    for parse_payment_response_handler output utxo_json
*/
pub type ParsePaymentReply = Vec<UTXO>;

/**
    Converts ParsePaymentReply (which should be input via indy-sdk) to ParsePaymentReply
    please note:  use of this function moves ParsePaymentResponse and it cannot be used again
    after this call
*/
pub fn from_response(base : ParsePaymentResponse) -> Result<ParsePaymentReply, ErrorCode> {
    let mut utxos: Vec<UTXO> = vec![];

    for unspent_output in base.result.txn.data.outputs {

        let (address, amount) = unspent_output;
        let qualified_address: String = add_qualifer_to_address(&address);
        let seq_no: i32 = base.result.tnx_meta_data.seq_no;
        let txo = (TXO { address: qualified_address.to_string(), seq_no}).to_libindy_string()?;
        let utxo: UTXO = UTXO { payment_address: qualified_address, txo, amount, extra: "".to_string() };

        utxos.push(utxo);
    }

    Ok(utxos)
}

#[cfg(test)]
mod parse_payment_response_tests {
    #[allow(unused_imports)]

    use logic::parsers::common::{ResponseOperations, UTXO, TXO};
    use utils::json_conversion::{JsonDeserialize, JsonSerialize};
    use utils::random::{rand_req_id, rand_string};
    use super::*;

    static PARSE_PAYMENT_RESPONSE_JSON: &'static str = r#"{
        "op": "REPLY",
        "protocolVersion": 2,
        "result":
        {
            "txn":
            {
                "data":
                {
                    "inputs":
                    [
                        ["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", 1]
                    ],
                    "outputs":
                    [
                        ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 13],
                        ["24xHHVDRq97Hss5BxiTciEDsve7nYNx1pxAMi9RAvcWMouviSY", 13],
                        ["mNYFWv9vvoQVCVLrSpbU7ZScthjNJMQxMs3gREQrwcJC1DsG5", 13],
                        ["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", 1]
                    ]
                },
                "metadata":
                {
                    "digest": "228af6a0c773cbbd575bf4e16f9144c2eaa615fa81fdcc3d06b83e20a92e5989",
                    "from": "6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1",
                    "reqId": 1529682415342024
                },
                "protocolVersion": 2,
                "type": "10001"
            },
            "reqSignature":
            {
                "type": "ED25519",
                "values":
                [
                    {
                        "from": "dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
                        "value": "4fFVD1HSVLaVdMpjHU168eviqWDxKrWYx1fRxw4DDLjg4XZXwya7UdcvVty81pYFcng244tS36WbshCeznC8ZN5Z"
                    }
                ]
            },
            "txnMetadata":
            {
                "seqNo": 2,
                "txnTime": 1529682415
            },
            "ver": "1",
            "auditPath": ["5NtSQUXaZvETP1KEWi8LaxSb9gGa2Qj31xKQoimNxCAT"],
            "rootHash": "GJFwiQt9r7n25PqM1oXBtRceXCeoqoCBcJmRH1c8fVTs"
        }
    }"#;

    static PAYMENT_RESPONSE_TRANSACTION_JSON: &'static str = r#"{
            "data":
            {
                "inputs":
                [
                    ["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", 1]
                ],
                "outputs":
                [
                    ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 13],
                    ["24xHHVDRq97Hss5BxiTciEDsve7nYNx1pxAMi9RAvcWMouviSY", 13],
                    ["mNYFWv9vvoQVCVLrSpbU7ZScthjNJMQxMs3gREQrwcJC1DsG5", 13],
                    ["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", 1]
                ]
            },
            "metadata":
            {
                "digest": "228af6a0c773cbbd575bf4e16f9144c2eaa615fa81fdcc3d06b83e20a92e5989",
                "from": "6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1",
                "reqId": 1529682415342024
            },
            "protocolVersion": 2,
            "type": "10001"
    }"#;

    // Ensures Transaction structure correctly deserializes
    #[test]
    fn success_parse_payment_transaction_from_json() {
        let response: Transaction = Transaction::from_json(PAYMENT_RESPONSE_TRANSACTION_JSON).unwrap();

        assert_eq!(response.txn_type, "10001");
        assert_eq!(response.protocol_version, 2);
    }

    // the PARSE_PAYMENT_RESPONSE_JSON is valid per the documentation.   If serde correctly serializes it
    // into ParsePaymentResponse then we know the ParsePaymentResponse structure matches
    #[test]
    fn success_parse_payment_response_from_json() {

        let response: ParsePaymentResponse = ParsePaymentResponse::from_json(PARSE_PAYMENT_RESPONSE_JSON).unwrap();

        assert_eq!(response.op, ResponseOperations::REPLY);
    }

    // this test passes when the valid JSON defined in PARSE_PAYMENT_RESPONSE_JSON is correctly serialized into
    // ParsePaymentResponse which is then successfully converted to ParsePaymentReply and then into json
    #[test]
    fn success_response_json_to_reply_json() {
        let response: ParsePaymentResponse = ParsePaymentResponse::from_json(PARSE_PAYMENT_RESPONSE_JSON).unwrap();
        let number_of_outputs: usize = response.result.txn.data.outputs.len();
        let reply: ParsePaymentReply = from_response(response).unwrap();

        assert_eq!(reply.len(), number_of_outputs);

    }


}