#![warn(unused_imports)]
#[allow(unused_imports)]

use serde::{Serialize, Deserialize};
use serde_json::{Value, Error};
use std::collections::HashMap;
use logic::request::Request;

const SET_FEES: &str = "20000";

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

    pub fn new(fees: HashMap<String, u64>) -> Request<SetFeesRequest> {
        let fee = SetFeesRequest {
            txn_type: SET_FEES,
            fees,
        };
        return Request::new(fee);
    }

    pub fn from_fee_config(fee: Fees) -> Request<SetFeesRequest> {
        return SetFeesRequest::new(fee.fees);
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


    // fees_txn_handler requires that a valid fees transaction is serialized. This tests that
    // the serializing structure for fees works correctly
    fn initial_set_fee_request() -> Request<SetFeesRequest> {
        let mut fees_map = HashMap::new();
        fees_map.insert(String::from("AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"), 10 as u64);
        return SetFeesRequest::new(fees_map);
    }

    fn assert_set_fee_request<F>(expected: serde_json::Value, f: F)
        where F: Fn(&mut Request<SetFeesRequest>) -> ()
    {
        let mut set_fee_req = initial_set_fee_request();
        f(&mut set_fee_req);
        let set_fee_req_c_string = set_fee_req.serialize_to_cstring().unwrap();
        let set_fee_req_json_str = str_from_char_ptr(set_fee_req_c_string.as_ptr()).unwrap();
        let deserialized_set_fee_request: Request<SetFeesRequest> = serde_json::from_str(set_fee_req_json_str).unwrap();
        assert_eq!(deserialized_set_fee_request.protocol_version, 1);

        let operation_json_value: serde_json::Value = serde_json::from_str(&deserialized_set_fee_request.operation.to_json().unwrap()).unwrap();
        assert_eq!(operation_json_value, expected);
    }

    #[test]
    fn create_request_with_fees_config() {
        let mut fees_map = HashMap::new();
        fees_map.insert(String::from("AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"), 10 as u64);
        let fees_config = Fees {
            fees: fees_map.clone()
        };
        let request = SetFeesRequest::from_fee_config(fees_config);
        assert_eq!(request.operation.fees, fees_map);
    }

    #[test]
    fn valid_request() {
        assert_set_fee_request(
            json!({
                "type": SET_FEES,
                "fees": {"AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja":10},
            }),
            |_fees_req| {}
        );
    }
}