extern crate env_logger;
extern crate indyrs as indy;
extern crate libc;
extern crate serde_json;
extern crate sovtoken;

use indy::future::Future;

use sovtoken::ErrorCode;

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
                    {
                        "address": "dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
                        "seqNo": 1
                    }
                ],
                "outputs":
                [
                    {
                        "address": "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                        "amount": 13
                    },
                    {
                        "address": "24xHHVDRq97Hss5BxiTciEDsve7nYNx1pxAMi9RAvcWMouviSY",
                        "amount": 13
                    },
                    {
                        "address": "mNYFWv9vvoQVCVLrSpbU7ZScthjNJMQxMs3gREQrwcJC1DsG5",
                        "amount": 13
                    },
                    {
                        "address": "dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
                        "amount": 1
                    }
                ]
            },
            "metadata":
            {
                "digest": "228af6a0c773cbbd575bf4e16f9144c2eaa615fa81fdcc3d06b83e20a92e5989",
                "from": "6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1",
                "reqId": 152968241
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

#[test]
pub fn parse_payment_response_works() {
    sovtoken::api::sovtoken_init();
    let resp = indy::payments::parse_payment_response("sov", PARSE_PAYMENT_RESPONSE_JSON).wait().unwrap();
    let resp: Vec<serde_json::Value> = serde_json::from_str(&resp).unwrap();
    assert_eq!(resp.len(), 4);
    for utxo in resp {
        utxo["recipient"].as_str().unwrap();
        utxo["receipt"].as_str().unwrap();
        utxo["amount"].as_u64().unwrap();
    }
}

#[test]
pub fn parse_payment_response_works_for_invalid() {
    sovtoken::api::sovtoken_init();
    let resp = indy::payments::parse_payment_response("sov", "123").wait().unwrap_err();
    assert_eq!(resp.error_code, ErrorCode::CommonInvalidStructure);
}