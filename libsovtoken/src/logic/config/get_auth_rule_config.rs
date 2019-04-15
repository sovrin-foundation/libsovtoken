/*!
    Structures for the [`build_get_txn_fees_handler`]

    [`build_get_txn_fees_handler`]: ../../../api/fn.build_get_txn_fees_handler.html
 */
use logic::did::Did;
use logic::request::Request;
use utils::constants::txn_types::GET_AUTH_RULE;

/**
    Struct for [`build_get_txn_fees_handler`]

    Can build a `Request<GetAuthRuleRequest>` which can be serialized into json.

    ```
        use sovtoken::logic::config::get_auth_rule_config::GetAuthRuleRequest;
        use sovtoken::logic::did::Did;

        let identifier = String::from("hgrhyNXqW4KNTz4wwiV8v");
        let did = Did::new(&identifier).validate().unwrap();
        let get_auth_rule_request = GetAuthRuleRequest::new().as_request(Some(did));
        let json_pointer = get_auth_rule_request.serialize_to_pointer().unwrap();
    ```

    [`Request<GetAuthRuleRequest>`]: ../../request/struct.Request.html
    [`build_get_txn_fees_handler`]: ../../../api/fn.build_get_txn_fees_handler.html
 */
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct GetAuthRuleRequest {
    #[serde(rename = "type")]
    txn_type: String,
}

impl GetAuthRuleRequest {
    
    /**
        Creates a new [`GetAuthRuleRequest`] struct.

        [`GetAuthRuleRequest`]: ./struct.GetAuthRuleRequest.html
    */
    pub fn new() -> GetAuthRuleRequest {
        return GetAuthRuleRequest {
            txn_type: GET_AUTH_RULE.to_string(),
        };
    }

    /**
        Transform `self` to a [`Request<GetAuthRuleRequest>`] struct.

        [`Request<GetAuthRuleRequest>`]: ../../request/struct.Request.html
    */
    pub fn as_request(self, identifier: Option<Did>) -> Request<GetAuthRuleRequest> {
        return Request::new(self, identifier.map(String::from));
    }
}

#[cfg(test)]
mod get_auth_rule_config_test {
    use super::*;
    use serde_json;
    use utils::json_conversion::{JsonSerialize};
    use utils::ffi_support::{str_from_char_ptr};
    use utils::random::rand_string;
    use utils::constants::general::PROTOCOL_VERSION;

    fn initial_get_auth_rule_request() -> Request<GetAuthRuleRequest> {
        let identifier: String = rand_string(21);
        let did = Did::new(&identifier);
        return GetAuthRuleRequest::new().as_request(Some(did));
    }

    fn assert_get_auth_rule_request<F>(expected: serde_json::Value, f: F)
        where F: Fn(&mut Request<GetAuthRuleRequest>) -> ()
    {
        let mut get_fee_req = initial_get_auth_rule_request();
        f(&mut get_fee_req);
        let get_auth_rule_req_c_string = get_fee_req.serialize_to_cstring().unwrap();
        let get_auth_rule_req_json_str = str_from_char_ptr(get_auth_rule_req_c_string.as_ptr()).unwrap();
        let deserialized_get_auth_rule_request: Request<GetAuthRuleRequest> = serde_json::from_str(get_auth_rule_req_json_str).unwrap();
        assert_eq!(deserialized_get_auth_rule_request.protocol_version, PROTOCOL_VERSION);

        let operation_json_value: serde_json::Value = serde_json::from_str(&deserialized_get_auth_rule_request.operation.to_json().unwrap()).unwrap();
        assert_eq!(operation_json_value, expected);
    }

    #[test]
    fn create_request_with_fees_config() {
        let identifier: String = rand_string(21);
        let did = Did::new(&identifier);
        let request = GetAuthRuleRequest::new().as_request(Some(did));
        assert_eq!(request.operation.txn_type, GET_AUTH_RULE.to_string());
    }

    #[test]
    fn valid_request() {
        assert_get_auth_rule_request(
            json!({
                "type": GET_AUTH_RULE,
            }),
            |_fees_req| {}
        );
    }
}