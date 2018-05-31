/*!
 *  Defines structure and implementation SetFeesConfig and SetFeesRequest
 *  these are the structures for the [`build_fees_txn_handler`]
 *
 *  [`build_fees_txn_handler`]: ../../../api/fn.build_fees_txn_handler.html
 */
use std::collections::HashMap;
use logic::request::Request;

const SET_FEES: &str = "20000";

/**
 *  Json config to customize [`build_fees_txn_handler`]
 *
 *  [`build_fees_txn_handler`]: ../../../api/fn.build_fees_txn_handler.html
 */
#[derive(Serialize, Deserialize)]
pub struct SetFeesConfig {
   pub fees: HashMap<String, u64>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct SetFeesRequest {
    #[serde(rename = "type")]
    txn_type: &'static str,
    fees:  HashMap<String, u64>,
}

/**
 * A struct that can be transformed into a SetFeesConfig JSON object.
 */
impl SetFeesRequest {

    pub fn new(fees: HashMap<String, u64>, identifier : String) -> Request<SetFeesRequest> {
        let fee = SetFeesRequest {
            txn_type: SET_FEES,
            fees,
        };
        return Request::new(fee, identifier);
    }

    pub fn from_fee_config(fee: SetFeesConfig, identifier: String) -> Request<SetFeesRequest> {
        return SetFeesRequest::new(fee.fees, identifier);
    }

}

#[cfg(test)]
mod fees_config_test {
    use super::*;
    use serde_json;
    use utils::json_conversion::{JsonSerialize};
    use utils::ffi_support::{str_from_char_ptr};
    use utils::random::rand_string;

    // fees_txn_handler requires that a valid fees transaction is serialized. This tests that
    // the serializing structure for fees works correctly
    fn initial_set_fee_request() -> Request<SetFeesRequest> {
        let identifier: String = rand_string(21);
        let mut fees_map = HashMap::new();
        fees_map.insert(String::from("AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"), 10 as u64);
        return SetFeesRequest::new(fees_map, identifier);
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
        let identifier: String = rand_string(21);
        let mut fees_map = HashMap::new();
        fees_map.insert(String::from("AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"), 10 as u64);
        let fees_config = SetFeesConfig {
            fees: fees_map.clone()
        };
        let request = SetFeesRequest::from_fee_config(fees_config, identifier);
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