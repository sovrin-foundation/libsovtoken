/*!
 *  Defines structure and implementation of getFeesRequest
 *  these are the structures for the [`build_get_fees_txn_handler`]
 *
 *  [`build_get_fees_txn_handler`]: ../../../api/fn.build_fees_txn_handler.html
 */
use logic::did::Did;
use logic::request::Request;
use utils::constants::txn_types::GET_FEES;

/**
 *  Json config to customize [`build_get_fees_txn_handler`]
 *
 *  [`build_get_fees_txn_handler`]: ../../../api/fn.build_get_fees_txn_handler.html
 */
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct GetFeesRequest {
    #[serde(rename = "type")]
    txn_type: String,
}

impl GetFeesRequest {
    pub fn new(identifier : Did) -> Request<GetFeesRequest> {
        let req = GetFeesRequest {
            txn_type: GET_FEES.to_string(),
        };
        return Request::new(req, Some(String::from(identifier)));
    }
}

#[cfg(test)]
mod get_fees_config_test {
    use super::*;
    use serde_json;
    use utils::json_conversion::{JsonSerialize};
    use utils::ffi_support::{str_from_char_ptr};
    use utils::random::rand_string;

    fn initial_get_fee_request() -> Request<GetFeesRequest> {
        let identifier: String = rand_string(21);
        let did = Did::new(&identifier);
        return GetFeesRequest::new(did);
    }

    fn assert_get_fee_request<F>(expected: serde_json::Value, f: F)
        where F: Fn(&mut Request<GetFeesRequest>) -> ()
    {
        let mut get_fee_req = initial_get_fee_request();
        f(&mut get_fee_req);
        let get_fee_req_c_string = get_fee_req.serialize_to_cstring().unwrap();
        let get_fee_req_json_str = str_from_char_ptr(get_fee_req_c_string.as_ptr()).unwrap();
        let deserialized_get_fee_request: Request<GetFeesRequest> = serde_json::from_str(get_fee_req_json_str).unwrap();
        assert_eq!(deserialized_get_fee_request.protocol_version, 1);

        let operation_json_value: serde_json::Value = serde_json::from_str(&deserialized_get_fee_request.operation.to_json().unwrap()).unwrap();
        assert_eq!(operation_json_value, expected);
    }

    #[test]
    fn create_request_with_fees_config() {
        let identifier: String = rand_string(21);
        let did = Did::new(&identifier);
        let get_fees_config = GetFeesRequest {
            txn_type: GET_FEES.to_string()
        };
        let request = GetFeesRequest::new(did);
        assert_eq!(request.operation.txn_type, GET_FEES.to_string());
    }

    #[test]
    fn valid_request() {
        assert_get_fee_request(
            json!({
                "type": GET_FEES,
            }),
            |_fees_req| {}
        );
    }
}