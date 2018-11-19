/*!
 *  Defines structure and implementation for OutputConfig and MintRequest
 *  these are the structures for the [`build_mint_txn_handler`]
 * 
 *  [`build_mint_txn_handler`]: ../../../api/fn.build_mint_txn_handler.html
 */

use logic::did::Did;
use logic::request::Request;
use logic::output::Output;
use utils::constants::txn_types::MINT_PUBLIC;
use logic::output::Outputs;

/**
 *  A struct which can be transformed into a mint JSON object for [`build_mint_txn_handler`]
 *  
 *  [`build_mint_txn_handler`]: ../../../api/fn.build_mint_txn_handler.html
 */
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct MintRequest {
    #[serde(rename = "type")]
    txn_type: String,
    outputs: Vec<(Output)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<String>,
}

impl MintRequest {

    /**
     * Creates a new `MintRequest` with `outputs`
     */
    pub fn new(outputs: Vec<Output>, identifier : Option<Did>, extra: Option<String>) -> Request<MintRequest> {
        let mint = MintRequest {
            txn_type: MINT_PUBLIC.to_string(),
            outputs,
            extra,
        };

        return Request::new(mint, identifier.map(String::from));
    }

    /**
     * Creates a new `MintRequest` from an [`OutputConfig`].
     * [`OutputConfig`]: ../general/struct.OutputConfig.html
     */
    pub fn from_config(mint_config: Outputs, identifier : Option<Did>, extra: Option<String>) -> Request<MintRequest> {
        return MintRequest::new(mint_config, identifier, extra);
    }
}

// this test ensures that the deserialized JSON is serialized correctly
#[cfg(test)]
mod output_mint_config_test {
    use super::*;
    use serde_json;
    use logic::output::OutputConfig;
    use utils::constants::general::PROTOCOL_VERSION;
    use utils::ffi_support::str_from_char_ptr;
    use utils::json_conversion::{JsonDeserialize, JsonSerialize};
    use utils::random::rand_string;


    #[test]
    fn serializing_mint_struct_config() {
        let output = Output::new(String::from("E9LNHk8shQ6xe2RfydzXDSsyhWC6vJaUeKE2mmc6mWraDfmKm"), 10);
        let mint : OutputConfig = OutputConfig {
            ver: 1,
            outputs: vec![output],
        };
        assert_eq!(mint.to_json().unwrap(), r#"{"ver":1,"outputs":[{"address":"E9LNHk8shQ6xe2RfydzXDSsyhWC6vJaUeKE2mmc6mWraDfmKm","amount":10}]}"#);
    }

    fn initial_mint_request() -> Request<MintRequest> {
        let identifier: String = rand_string(21);
        let did = Did::new(&identifier);
        let output = Output::new(String::from("E9LNHk8shQ6xe2RfydzXDSsyhWC6vJaUeKE2mmc6mWraDfmKm"), 10);
        let outputs = vec![output];
        return MintRequest::new(outputs, Some(did), None);
    }

    fn assert_mint_request<F>(expected: serde_json::Value, f: F)
        where F: Fn(&mut Request<MintRequest>) -> ()
    {
        let mut mint_req = initial_mint_request();
        f(&mut mint_req);
        let mint_req_c_string = mint_req.serialize_to_cstring().unwrap();
        let mint_req_json_str = str_from_char_ptr(mint_req_c_string.as_ptr()).unwrap();
        let deserialized_mint_request: Request<MintRequest> = Request::<MintRequest>::from_json(mint_req_json_str).unwrap();
        assert_eq!(deserialized_mint_request.protocol_version, PROTOCOL_VERSION);

        let operation_json_value : serde_json::Value = serde_json::from_str(&deserialized_mint_request.operation.to_json().unwrap()).unwrap();
        assert_eq!(operation_json_value, expected);
    }

    #[test]
    fn create_request_with_mint_config() {
        let identifier: String = rand_string(21);
        let did = Did::new(&identifier);
        let output = Output::new(String::from("E9LNHk8shQ6xe2RfydzXDSsyhWC6vJaUeKE2mmc6mWraDfmKm"), 10);
        let outputs = vec![output];
        let request = MintRequest::from_config(outputs.clone(), Some(did), None);
        assert_eq!(request.operation.outputs, outputs);
    }

    #[test]
    fn valid_request() {
        assert_mint_request(
            json!({
                "type": MINT_PUBLIC,
                "outputs": [{
                    "address":"E9LNHk8shQ6xe2RfydzXDSsyhWC6vJaUeKE2mmc6mWraDfmKm",
                    "amount":10
                }],
            }),
            |_mint_req| {}
        )
    }
}
