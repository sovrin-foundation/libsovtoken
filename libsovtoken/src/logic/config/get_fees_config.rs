/*!
    Structures for the [`build_get_txn_fees_handler`]

    [`build_get_txn_fees_handler`]: ../../../api/fn.build_get_txn_fees_handler.html
 */
use logic::did::Did;
use logic::request::Request;
use utils::constants::txn_types::GET_FEES;

/**
    Struct for [`build_get_txn_fees_handler`]

    Can build a `Request<GetFeesRequest>` which can be serialized into json.

    ```
        use sovtoken::logic::config::get_fees_config::GetFeesRequest;
        use sovtoken::logic::did::Did;

        let identifier = String::from("hgrhyNXqW4KNTz4wwiV8v");
        let did = Did::new(&identifier).validate().unwrap();
        let get_fees_request = GetFeesRequest::new().as_request(did);
        let json_pointer = get_fees_request.serialize_to_pointer().unwrap();
    ```

    [`Request<GetFeesRequest>`]: ../../request/struct.Request.html
    [`build_get_txn_fees_handler`]: ../../../api/fn.build_get_txn_fees_handler.html
 */
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct GetFeesRequest {
    #[serde(rename = "type")]
    txn_type: String,
}

impl GetFeesRequest {
    
    /**
        Creates a new [`GetFeesRequest`] struct.

        [`GetFeesRequest`]: ./struct.GetFeesRequest.html
    */
    pub fn new() -> GetFeesRequest {
        return GetFeesRequest {
            txn_type: GET_FEES.to_string(),
        };
    }

    /**
        Transform `self` to a [`Request<GetFeesRequest>`] struct.

        [`Request<GetFeesRequest>`]: ../../request/struct.Request.html
    */
    pub fn as_request(self, identifier: Did) -> Request<GetFeesRequest> {
        return Request::new(self, Some(String::from(identifier)));
    }
}

#[cfg(test)]
mod get_fees_config_test {
    use super::*;
    use serde_json;
    use utils::json_conversion::{JsonSerialize};
    use utils::ffi_support::{str_from_char_ptr};
    use utils::random::rand_string;
    use utils::constants::general::PROTOCOL_VERSION;

    fn initial_get_fee_request() -> Request<GetFeesRequest> {
        let identifier: String = rand_string(21);
        let did = Did::new(&identifier);
        return GetFeesRequest::new().as_request(did);
    }

    fn assert_get_fee_request<F>(expected: serde_json::Value, f: F)
        where F: Fn(&mut Request<GetFeesRequest>) -> ()
    {
        let mut get_fee_req = initial_get_fee_request();
        f(&mut get_fee_req);
        let get_fee_req_c_string = get_fee_req.serialize_to_cstring().unwrap();
        let get_fee_req_json_str = str_from_char_ptr(get_fee_req_c_string.as_ptr()).unwrap();
        let deserialized_get_fee_request: Request<GetFeesRequest> = serde_json::from_str(get_fee_req_json_str).unwrap();

        assert_eq!(deserialized_get_fee_request.protocol_version, PROTOCOL_VERSION);

        let operation_json_value: serde_json::Value = serde_json::from_str(&deserialized_get_fee_request.operation.to_json().unwrap()).unwrap();
        assert_eq!(operation_json_value, expected);
    }

    #[test]
    fn create_request_with_fees_config() {
        let identifier: String = rand_string(21);
        let did = Did::new(&identifier);
        let request = GetFeesRequest::new().as_request(did);
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