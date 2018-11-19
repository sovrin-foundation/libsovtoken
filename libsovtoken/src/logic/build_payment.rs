//! what is this module for?

use indy::ErrorCode;
use libc::c_char;
use logic::config::payment_config::PaymentRequest;
use logic::input::Inputs;
use logic::output::Outputs;
use logic::xfer_payload::XferPayload;
use utils::ffi_support::{string_from_char_ptr, c_pointer_from_str};
use utils::base58::{IntoBase58, FromBase58};
use serde_json;

type BuildPaymentRequestCb = extern fn(ch: i32, err: i32, request_json: *const c_char) -> i32;
type DeserializedArguments = (Inputs, Outputs, Option<String>, BuildPaymentRequestCb);

pub fn deserialize_inputs(
    inputs_json: *const c_char,
    outputs_json: *const c_char,
    extra: *const c_char,
    cb: Option<BuildPaymentRequestCb>
) -> Result<DeserializedArguments, ErrorCode> {
    trace!("logic::build_payment::deserialize_inputs >> inputs_json: {:?}, outputs_json: {:?}, extra: {:?}", inputs_json, outputs_json, extra);
    let cb = cb.ok_or(ErrorCode::CommonInvalidStructure)?;

    let inputs_json = string_from_char_ptr(inputs_json)
        .ok_or(ErrorCode::CommonInvalidStructure).map_err(map_err_err!())?;
    debug!("Converted inputs_json pointer to string >>> {:?}", inputs_json);
    
    let outputs_json = string_from_char_ptr(outputs_json)
        .ok_or(ErrorCode::CommonInvalidStructure).map_err(map_err_err!())?;
    debug!("Converted outputs_json pointer to string >>> {:?}", outputs_json);

    let inputs: Inputs = serde_json::from_str(&inputs_json).map_err(map_err_err!())
        .or(Err(ErrorCode::CommonInvalidStructure))?;
    debug!("Deserialized input_json >>> {:?}", inputs);

    let outputs: Outputs = serde_json::from_str(&outputs_json).map_err(map_err_err!())
        .or(Err(ErrorCode::CommonInvalidStructure))?;
    debug!("Deserialized output_json >>> {:?}", outputs);

    let extra = string_from_char_ptr(extra);
    debug!("Deserialized extra >>> {:?}", extra);

    trace!("logic::build_payment::deserialize_inputs << inputs: {:?}, outputs: {:?}, extra: {:?}", inputs, outputs, extra);
    return Ok((inputs, outputs, extra, cb));
}

pub fn handle_signing(
    command_handle: i32,
    signed_payload: Result<XferPayload, ErrorCode>,
    cb: BuildPaymentRequestCb
) {
    let (error_code, pointer) = match build_payment_request_pointer(signed_payload) {
        Ok(request_pointer) => (ErrorCode::Success, request_pointer),
        Err(ec) => (ec, c_pointer_from_str("")),
    };
    
    cb(command_handle, error_code as i32, pointer);
}


fn build_payment_request_pointer(
    signed_payload: Result<XferPayload, ErrorCode>
) -> Result<*const c_char, ErrorCode> {
    let signed_payload = signed_payload?;
    debug!("Signed payload >>> {:?}", signed_payload);

    if signed_payload.signatures.is_none() {
        error!("Building an unsigned payment request.");
        return Err(ErrorCode::CommonInvalidStructure);
    }

    let identifier = signed_payload.inputs[0].address.clone();
    let identifier = identifier.as_bytes().from_base58_check();
    let identifier = identifier.map(|s| s.into_base58()).map_err(|_| ErrorCode::CommonInvalidStructure)?;

    let payment_request = PaymentRequest::new(signed_payload)
        .as_request(identifier);
    debug!("payment_request >>> {:?}", payment_request);

    return payment_request
        .serialize_to_pointer()
        .map_err(|e| {
            map_err_err!()(e);
            return ErrorCode::CommonInvalidState;
        });
}


#[cfg(test)]
mod test_deserialize_inputs {
    use indy::ErrorCode;
    use libc::c_char;
    use std::ptr;
    use utils::test::default;

    use super::{
        BuildPaymentRequestCb,
        DeserializedArguments,
        deserialize_inputs,
    };


    pub fn call_deserialize_inputs(
        inputs_json: Option<*const c_char>,
        outputs_json: Option<*const c_char>,
        extra: Option<*const c_char>,
        cb: Option<Option<BuildPaymentRequestCb>>
    ) -> Result<DeserializedArguments, ErrorCode> {
        let inputs_json = inputs_json.unwrap_or_else(default::inputs_json_pointer);
        let outputs_json = outputs_json.unwrap_or_else(default::outputs_json_pointer);
        let extra = extra.unwrap_or(ptr::null());
        let cb = cb.unwrap_or(Some(default::empty_callback_string));

        return deserialize_inputs(inputs_json, outputs_json, extra, cb);
    }

    #[test]
    fn deserialize_empty_inputs() {
        let result = call_deserialize_inputs(Some(ptr::null()), None, None, None);
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn deserialize_empty_outputs() {
        let result = call_deserialize_inputs(None, Some(ptr::null()), None, None);
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn deserialize_empty_callback() {
        let result = call_deserialize_inputs(None, None, None, Some(None));
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn deserialize_invalid_inputs() {
        let inputs_json = json_c_pointer!({
            "ver": 1,
            "inputs": {
                "address": "pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81f",
                "seqNo": 2
            }
        });
        let result = call_deserialize_inputs(Some(inputs_json), None, None, None);
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn deserialize_invalid_outputs() {
        let outputs_json = json_c_pointer!({
            "ver": 1,
            "outputs": {
                "address": "pay:sov:a8QAXMjRwEGoGLmMFEc5sTcntZxEF1BpqAs8GoKFa9Ck81fo7",
                "amount": 10,
                "seqNo": 5,
            }
        });
        let result = call_deserialize_inputs(None, Some(outputs_json), None, None);
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn deserialize_valid() {
        let result = call_deserialize_inputs(None, None, None, None);
        assert!(result.is_ok());
    }
}


#[cfg(test)]
mod test_handle_signing {
    use super::*;
    use indy::utils::results::ResultHandler;
    use logic::request::Request;
    use utils::test::{default, callbacks};


    fn call_handle_signing(input_payload: Result<XferPayload, ErrorCode>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = callbacks::cb_ec_string();
        handle_signing(command_handle, input_payload, cb.unwrap());
        ResultHandler::one(ErrorCode::Success, receiver)
    }

    #[test]
    fn test_error_code() {
        let signed_payload_result = Err(ErrorCode::CommonInvalidParam1);
        let result = call_handle_signing(signed_payload_result);
        assert_eq!(ErrorCode::CommonInvalidParam1, result.unwrap_err());
    }

    #[test]
    fn test_xfer_without_signatures() {
        let unsigned_payload = default::xfer_payload_unsigned();
        let result = call_handle_signing(Ok(unsigned_payload));
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn test_signed_xfer_payload() {
        let signed_payload = default::xfer_payload_signed();
        let result = call_handle_signing(Ok(signed_payload)).unwrap();
        let request: Request<serde_json::value::Value> = serde_json::from_str(&result).unwrap();
        assert_eq!("10001", request.operation.get("type").unwrap());
        assert_eq!(
            "iTQzpdRdugkJ2gLD5vW5c159dncSL9jbAtu3WfPcb8qWD9bUd",
            request.operation.get("inputs").unwrap().as_array().unwrap()
                .get(0).unwrap().as_object().unwrap()
                .get("address").unwrap().as_str().unwrap()
        );
        assert_eq!("7LSfLv2S6K7zMPrgmJDkZoJNhWvWRzpU7qt9uMR5yz8G".to_string(), request.identifier);
    }
}