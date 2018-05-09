#![warn(unused_imports)]
#[allow(unused_imports)]

use serde::{Serialize, Deserialize};
use serde_json::{Value, Error};
use std::collections::HashMap;
use logic::request::Request;

#[derive(Serialize, Deserialize)]
pub struct Fees {
   pub  fees: HashMap<String, u64>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct SetFeesRequest {
    #[serde(rename = "type")]
    txn_type: &'static str,
    fees:  HashMap<String, u64>,
}

impl SetFeesRequest {

    pub fn new(fees: HashMap<String, u64>, did: String) -> Request<SetFeesRequest> {
        let fee = SetFeesRequest {
            txn_type: "20000",
            fees,
        };
        return Request::new(fee, did);
    }

    pub fn from_fee(fee: Fees, did: String) -> Request<SetFeesRequest> {
        return SetFeesRequest::new(fee.fees, did);
    }
}

#[cfg(test)]
mod fees_config_test {
    use super::*;
    use std::ffi::CString;
    use utils::ffi_support::{str_from_char_ptr, cstring_from_str};
    use serde_json;
    use serde_json::{Value, Error};
    use utils::json_conversion::{JsonDeserialize, JsonSerialize};

    // TESTING GLOBAL VARS
    static TEST_OP_JSON: &'static str = r#"{"fees":{"ThisIsomeBizzareDIdsgivenTOme":1001}}"#;
    static TEST_SIGS_JSON: &'static str = r#"{"signatures":{"one":"two","three":"four"}}"#;
    static TEST_OPS_JSON: &'static str = r#"{"type":"FEE","fees":{"ThisIsomeBizzareDIdsgivenTOme":1001,"ThisIsomeBizzareDIdsgivenTOme1":1001}}"#;

    // fees_txn_handler requires that a valid fees transaction is serialized. This tests that
    // the serializing structure for fees works correctly
    fn initial_set_fee_request() -> Request<SetFeesRequest> {
        let mut fees_map = HashMap::new();
        fees_map.insert(String::from("AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"), 10 as u64);
        let did = String::from("EFlzewrfDSfesaiuhgvcxFhhgpeBUddgseaGIUdFU");
        return SetFeesRequest::new(fees_map, did);
    }

    fn assert_set_fee_request<F>(expected: serde_json::Value, signatures: HashMap<String, String>, f: F)
        where F: Fn(&mut Request<SetFeesRequest>) -> ()
    {
        let mut set_fee_req = initial_set_fee_request();
        f(&mut set_fee_req);
        let set_fee_req_c_string = set_fee_req.serialize_to_cstring().unwrap();
        let set_fee_req_json_str = str_from_char_ptr(set_fee_req_c_string.as_ptr()).unwrap();
        let deserialized_set_fee_request: Request<SetFeesRequest> = serde_json::from_str(set_fee_req_json_str).unwrap();
        assert_eq!(deserialized_set_fee_request.identifier, "EFlzewrfDSfesaiuhgvcxFhhgpeBUddgseaGIUdFU");
        assert_eq!(deserialized_set_fee_request.signatures, signatures);

        let operation_json_value: serde_json::Value = serde_json::from_str(&deserialized_set_fee_request.operation.to_json().unwrap()).unwrap();
        assert_eq!(operation_json_value, expected);
    }

    #[test]
    fn unsigned_request() {
        assert_set_fee_request(
            json!({
                "type": "20000",
                "fees": {"AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja":10},
            }),
            HashMap::new(),
            |_set_fee_req| {}
        );
    }
}