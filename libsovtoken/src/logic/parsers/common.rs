//!

/**
    enumeration matches values for the op field in json
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub enum ResponseOperations {
    REPLY,
    REJECT,
    REQNACK,
}


/**
    UTXO is the structure for the data member utxo_json

    used by [`ParsePaymentReply`], [`ParseGetUtxoReply`], [`ParseResponseWithFeesReply`]
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


/**
    the nested type "req_signature" in inputs in parse response methods
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequireSignature {
    #[serde(rename = "type")]
    pub sig_type: String,
    pub values: Vec<SignatureValues>,
}

/**
    the nested type "values" in RequiredSignature
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SignatureValues {
    pub from: String,
    pub value: String,
}

/**
    the nested type "tnx_meta_data" in inputs in parse response methods
*/
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMetaData {
    pub seq_no: i32,
    pub txn_time: u32,
}
