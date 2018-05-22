/*!
 *  Defines structure and implementation inputs and outputs and submitter did
 *  these are the structures for the [`build_payment_req_handler`]
 *
 *  [`build_payment_req_handler`]: ../../../api/fn.build_payment_req_handler.html
 */
use logic::request::Request;

use logic::input::Input;
use logic::output::Output;
use super::general::{InputConfig, OutputConfig};


#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct PaymentRequest {
    #[serde(rename = "type")]
    txn_type: &'static str,
    inputs: Vec<(Input)>,
    outputs: Vec<(Output)>,
}

/**
 * A struct that can be transformed into a Fees JSON object.
 */
impl PaymentRequest {
    
    /**
     * Creates a new `PaymentRequest` with `inputs` and `outputs`
     */
    pub fn new(outputs: Vec<Output>, inputs: Vec<Input>) -> Request<PaymentRequest> {
        let fees = PaymentRequest {
            txn_type: "10000",
            inputs,
            outputs,
        };

        return Request::new(fees);
    }

    pub fn from_config(fees_output_config: OutputConfig, fees_input_config: InputConfig) -> Request<PaymentRequest> {
        return PaymentRequest::new(fees_output_config.outputs, fees_input_config.inputs );
    }
}

// this test ensures that the deserialized JSON is serialized correctly
#[cfg(test)]
mod fees_req_output_config_test {
    use super::*;
    use utils::json_conversion::JsonSerialize;

    #[test]
    fn serializing_fee_struct_output_config() {
        let output = Output::new(String::from("AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"), 10, None);

        let fee: OutputConfig = OutputConfig {
            outputs: vec![output],
        };
        assert_eq!(fee.to_json().unwrap(), r#"{"outputs":[["AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",10]]}"#);
    }
}

// this test ensures that the deserialized JSON is serialized correctly
#[cfg(test)]
mod fees_req_input_config_test {
    use super::InputConfig;
    use super::*;
    use utils::json_conversion::JsonSerialize;

    #[test]
    fn serializing_fee_struct_output_config() {

        let input = Input::new(String::from("dakjhe238yad"),30,String::from("239asdkj3298uadkljasd98u234ijasdlkj"));

        let fee: InputConfig = InputConfig {
            inputs: vec![input],
        };
        assert_eq!(fee.to_json().unwrap(), r#"{"inputs":[["dakjhe238yad",30,"239asdkj3298uadkljasd98u234ijasdlkj"]]}"#);
    }
}

#[cfg(test)]
mod fees_request_test {
    use super::*;
    use serde_json;
    use utils::ffi_support::str_from_char_ptr;
    use utils::json_conversion::{JsonDeserialize, JsonSerialize};


    fn initial_fees_request() -> Request<PaymentRequest> {
        let output = Output::new(String::from("AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"), 10, None);
        let input = Input::new(String::from("dakjhe238yad"),30,String::from("239asdkj3298uadkljasd98u234ijasdlkj"));

        let outputs = vec![output];
        let inputs = vec![input];
        return PaymentRequest::new(outputs, inputs);
    }

    fn assert_fees_request<F>(expected: serde_json::Value, f: F)
        where F: Fn(&mut Request<PaymentRequest>) -> ()
    {
        let mut fees_req = initial_fees_request();
        f(&mut fees_req);
        let fees_req_c_string = fees_req.serialize_to_cstring().unwrap();
        let fees_req_json_str = str_from_char_ptr(fees_req_c_string.as_ptr()).unwrap();
        let deserialized_fees_request: Request<PaymentRequest> = Request::<PaymentRequest>::from_json(fees_req_json_str).unwrap();
        assert_eq!(deserialized_fees_request.protocol_version, 1);

        let operation_json_value : serde_json::Value = serde_json::from_str(&deserialized_fees_request.operation.to_json().unwrap()).unwrap();
        assert_eq!(operation_json_value, expected);
    }

    #[test]
    fn create_request_with_fees_config() {
        let output = Output::new(String::from("AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"), 10, None);
        let input = Input::new(String::from("dakjhe238yad"),30,String::from("239asdkj3298uadkljasd98u234ijasdlkj"));

        let outputs = vec![output];
        let inputs = vec![input];

        let output_config = OutputConfig {
            outputs: outputs.clone()
        };

        let input_config = InputConfig {
            inputs: inputs.clone()
        };

        let request = PaymentRequest::from_config(output_config, input_config);
        assert_eq!(request.operation.outputs, outputs);
        assert_eq!(request.operation.inputs, inputs);
    }

    #[test]
    fn valid_request() {
        assert_fees_request(
            json!({
                "type": "10000",
                "outputs": [["AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",10]],
                "inputs": [["dakjhe238yad", 30, "239asdkj3298uadkljasd98u234ijasdlkj"]]
            }),
            |_fees_req| {}
        )
    }

}
