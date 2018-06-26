use indy::ErrorCode;
use libc::c_char;
use logic::input::{Inputs, InputConfig};
use logic::output::{Outputs, OutputConfig};
use utils::ffi_support::string_from_char_ptr;
use serde_json;

type BuildPaymentRequestCb = extern fn(ch: i32, err: i32, request_json: *const c_char) -> i32;
type DeserializedArguments = (Inputs, Outputs, BuildPaymentRequestCb);

pub fn deserialize_inputs(
    inputs_json: *const c_char,
    outputs_json: *const c_char,
    cb: Option<BuildPaymentRequestCb>
) -> Result<DeserializedArguments, ErrorCode> {
    let cb = cb.ok_or(ErrorCode::CommonInvalidStructure)?;

    let inputs_json = string_from_char_ptr(inputs_json)
        .ok_or(ErrorCode::CommonInvalidStructure)?;
    debug!("Converted inputs_json pointer to string >>> {:?}", inputs_json);
    

    let outputs_json = string_from_char_ptr(outputs_json)
        .ok_or(ErrorCode::CommonInvalidStructure)?;
    debug!("Converted outputs_json pointer to string >>> {:?}", outputs_json);

    let input_config: InputConfig = serde_json::from_str(&inputs_json)
        .or(Err(ErrorCode::CommonInvalidStructure))?;
    debug!("Deserialized input_json >>> {:?}", input_config);

    let output_config: OutputConfig = serde_json::from_str(&outputs_json)
        .or(Err(ErrorCode::CommonInvalidStructure))?;
    debug!("Deserialized output_json >>> {:?}", output_config);


    return Ok((input_config.inputs, output_config.outputs, cb));
}

#[cfg(test)]
mod test_deserialize_inputs {
    use utils::ffi_support::c_pointer_from_string;
use indy::ErrorCode;
    use libc::c_char;
    use std::ptr;
    use utils::default;

    use super::{
        BuildPaymentRequestCb,
        DeserializedArguments,
        deserialize_inputs,
    };


    pub fn call_deserialize_inputs(
        inputs_json: Option<*const c_char>,
        outputs_json: Option<*const c_char>,
        cb: Option<Option<BuildPaymentRequestCb>>
    ) -> Result<DeserializedArguments, ErrorCode> {
        let inputs_json = inputs_json.unwrap_or_else(default::inputs_json_pointer);
        let outputs_json = outputs_json.unwrap_or_else(default::outputs_json_pointer);
        let cb = cb.unwrap_or(Some(default::empty_callback_string));

        return deserialize_inputs(inputs_json, outputs_json, cb);
    }

    #[test]
    fn deserialize_empty_inputs() {
        let result = call_deserialize_inputs(Some(ptr::null()), None, None);
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn deserialize_empty_outputs() {
        let result = call_deserialize_inputs(None, Some(ptr::null()), None);
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn deserialize_empty_callback() {
        let result = call_deserialize_inputs(None, None, Some(None));
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn deserialize_invalid_inputs() {
        let inputs_json = c_pointer_from_string(json!({
            "ver": 1,
            "inputs": {
                "address": "pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81f",
                "seqNo": 2
            }
        }).to_string());
        let result = call_deserialize_inputs(Some(inputs_json), None, None);
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn deserialize_invalid_outputs() {
        let outputs_json = c_pointer_from_string(json!({
            "ver": 1,
            "outputs": {
                "address": "pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7",
                "amount": 10,
                "seqNo": 5,
            }
        }).to_string());
        let result = call_deserialize_inputs(None, Some(outputs_json), None);
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn deserialize_valid() {
        let result = call_deserialize_inputs(None, None, None);
        assert!(result.is_ok());
    }
}