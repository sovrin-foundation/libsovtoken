use serde_json;

use logic::type_aliases::ProtocolVersion;
use logic::parsers::common::ResponseOperations;
use logic::output::Outputs;
use logic::input::Inputs;
use ErrorCode;
use logic::parsers::common::UTXO;
use logic::parsers::common::TXO;
use logic::type_aliases::TxnSeqNo;
use logic::address;

/**
    for parse_get_utxo_response_handler input parameter resp_json
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParseVerifyResponse {
    pub op : ResponseOperations,
    #[serde(rename = "protocol_version")]
    pub protocol_version: Option<ProtocolVersion>,
    pub result : Option<ParseVerifyResponseResult>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParseVerifyResponseResult {
    pub data: Option<ParseVerifyResponseResultData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParseVerifyResponseResultData {
    pub txn: Option<ParseVerifyResponseResultDataTxn>,
    pub txn_metadata: Option<TxnMetadata>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TxnMetadata {
    pub seq_no: TxnSeqNo
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParseVerifyResponseResultDataTxn {
    pub data: Option<ParseVerifyResponseResultDataTxnData>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParseVerifyResponseResultDataTxnData {
    pub outputs: Option<Outputs>,
    pub inputs: Option<Inputs>,
    pub extra: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VerifyResult {
    pub sources: Option<Vec<String>>,
    pub receipts: Option<Vec<UTXO>>,
    pub extra: Option<String>
}

fn parse_verify(resp: &str) -> Result<VerifyResult, ErrorCode> {
    let p: ParseVerifyResponse = serde_json::from_str(resp)
        .map_err(map_err_err!())
        .map_err(|_| ErrorCode::CommonInvalidStructure)?;

    let data = p.result
        .ok_or(ErrorCode::CommonInvalidStructure)?
        .data
        .ok_or(ErrorCode::PaymentSourceDoesNotExistError)?;

    let seq_no = data.txn_metadata
        .ok_or(ErrorCode::CommonInvalidStructure)?.seq_no;

    let data = data
        .txn
        .ok_or(ErrorCode::CommonInvalidStructure)?
        .data
        .ok_or(ErrorCode::CommonInvalidStructure)?;

    let mut sources: Vec<String> = vec![];
    let mut receipts: Vec<UTXO> = vec![];
    let extra = data.extra;

    if let Some(inputs) = data.inputs {
        for input in inputs {
            let address = address::address_from_unqualified_address(&input.address.to_string())?;
            sources.push(TXO { address, seq_no: input.seq_no }.to_libindy_string()?)
        }
    }

    if let Some(outputs) = data.outputs {
        for output in outputs {
            let address = address::address_from_unqualified_address(&output.recipient.to_string())?;
            receipts.push(UTXO {
                recipient: address.clone(),
                receipt: TXO { address, seq_no }.to_libindy_string()?,
                amount: output.amount,
                extra: extra.as_ref().unwrap_or(&"".to_string()).to_string(),
            })
        }
    }

    Ok(VerifyResult {
        sources: Some(sources),
        receipts: Some(receipts),
        extra,
    })
}

pub fn parse_response(resp: &str) -> Result<String, ErrorCode> {
    let r = parse_verify(resp)?;
    serde_json::to_string(&r).map_err(map_err_err!()).map_err(|_| ErrorCode::CommonInvalidStructure)
}

#[cfg(test)]
mod test_parse_verify {
    use super::*;

    const VALID_REQUEST: &str =
        r#"{
            "op": "REPLY",
            "result": {
                "data": {
                    "auditPath": [],
                    "reqSignature": {
                        "type": "ED25519",
                        "values": [
                            {
                                "from": "V4SGRU86Z58d6TV7PBUe6f",
                                "value": "5VCyi9onqjESFe5QaVQvFjb3bJZnNn4JgYDrPxYd3nyqUWbhJc5wqupbq3bacjbbRQBnKa8YKGZXKmP9q6Qtc8Mu"
                            }
                        ]
                    },
                    "rootHash": "BHWhyWrYLyYREHeadHZGSvCp9pbxTE5jE9jM2cV3RM2z",
                    "txn": {
                        "data": {
                            "outputs": [
                                {
                                    "address": "sM2S2UJVkh9FEZLo7bNJD5aw3u98v6eryDyN9ehB2iW7kia4M",
                                    "amount": 10
                                }
                            ]
                        },
                        "metadata": {
                            "digest": "a922288dc7b7ffa5a4dc93050d80a8134c70d68e8654600b0237e80210349d80",
                            "from": "V4SGRU86Z58d6TV7PBUe6f",
                            "reqId": 3787223578
                        },
                        "protocolVersion": 2,
                        "type": "10000"
                    },
                    "txnMetadata": {"seqNo": 1, "txnTime": 1532341475},
                    "ver": "1"
                },
                "identifier": "Th7MpTaRZVRYnPiabds81Y",
                "reqId": 1532360723205721420,
                "seqNo": 1,
                "type": "3"
            }
        }"#;

    #[test]
    pub fn parse_verify_works() {
        let res = parse_verify(VALID_REQUEST).unwrap();
        println!("{:?}", serde_json::to_string(&res));
    }
}