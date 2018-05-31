use indy::{ErrorCode};
use libc::c_char;
use logic::fees::{Inputs, Outputs, Fees, InputSigner};
use serde_json;
use utils::ffi_support::{string_from_char_ptr};

type SerdeMap = serde_json::Map<String, serde_json::value::Value>;
type AddRequestFeesCb = extern fn(command_handle_: i32, err: ErrorCode, req_with_fees_json: *const c_char) -> ErrorCode;
type DeserializedArguments = (Inputs, Outputs, SerdeMap, AddRequestFeesCb);

/**
 * Deserializes arguments of [`add_request_fees_handler`]
 */
pub fn deserialize_inputs (
    req_json: *const c_char,
    inputs_json: *const c_char,
    outputs_json: *const c_char,
    cb: Option<AddRequestFeesCb>
) -> Result<DeserializedArguments, ErrorCode> {

    let cb = cb.ok_or(ErrorCode::CommonInvalidStructure)?;

    let request_json = string_from_char_ptr(req_json).ok_or(ErrorCode::CommonInvalidStructure)?;
    debug!("Converted request_json pointer into string >>> {:?}", request_json);

    let inputs_json = string_from_char_ptr(inputs_json).ok_or(ErrorCode::CommonInvalidStructure)?;
    debug!("Converted inputs_json pointer to string >>> {:?}", inputs_json);

    let outputs_json = string_from_char_ptr(outputs_json).ok_or(ErrorCode::CommonInvalidStructure)?;
    debug!("Converted outputs_json pointer to string >>> {:?}", outputs_json);

    let inputs: Inputs = serde_json::from_str(&inputs_json).or(Err(ErrorCode::CommonInvalidStructure))?;
    debug!("Deserialized inputs >>> {:?}", inputs);

    let outputs: Outputs = serde_json::from_str(&outputs_json).or(Err(ErrorCode::CommonInvalidStructure))?;
    debug!("Deserialized outputs >>> {:?}", outputs);

    let request_json_object: serde_json::Value = serde_json::from_str(&request_json).or(Err(ErrorCode::CommonInvalidStructure))?;
    trace!("Converted request_json to serde::json::Value");

    let request_json_map = request_json_object.as_object().ok_or(ErrorCode::CommonInvalidStructure)?;
    trace!("Converted request_json to hash_map");

    return Ok((
        inputs,
        outputs,
        request_json_map.to_owned(),
        cb,
    ));
}

pub fn validate_type_not_transfer(request_json_map: &SerdeMap) -> Result<(), ErrorCode> {
    let key_operation = String::from("operation");
                                                              

    trace!("Getting type from request_json");
    let transaction_type = request_json_map
        .get(&key_operation)
        .and_then(|operation| operation.get("type"))
        .ok_or(ErrorCode::CommonInvalidStructure)?;
    debug!("Request transaction type was >>> {}", transaction_type);

    if transaction_type == "10000" {
        return Err(ErrorCode::CommonInvalidStructure);
    } else {
        return Ok(());
    };
}

pub fn add_fees_to_request_and_serialize(
    wallet_handle: i32,
    inputs: Inputs,
    outputs: Outputs,
    request_json_map: SerdeMap
) -> Result<String, ErrorCode> {
    let request_json_map_with_fees = add_fees(wallet_handle, inputs, outputs, request_json_map)?;
    return serialize_request_with_fees(request_json_map_with_fees);
}

fn add_fees(wallet_handle: i32, inputs: Inputs, outputs: Outputs, mut request_json_map: SerdeMap) -> Result<SerdeMap, ErrorCode> {
    let key_fees = String::from("fees");
    let fees = signed_fees(wallet_handle, inputs, outputs)?;

    request_json_map.insert(key_fees, json!(fees));
    trace!("Added fees to request_json.");

    return Ok(request_json_map);
}

fn serialize_request_with_fees(request_json_map_with_fees: SerdeMap) -> Result<String, ErrorCode> {
    let serialized_request_with_fees = serde_json::to_string(&json!(request_json_map_with_fees))
        .or(Err(ErrorCode::CommonInvalidStructure))?;
    trace!("Serialized request_with_fees");
    
    return Ok(serialized_request_with_fees);
} 

fn signed_fees(wallet_handle: i32, inputs: Inputs, outputs: Outputs) -> Result<Fees, ErrorCode> {
    let signed_inputs = Fees::sign_inputs(wallet_handle, &inputs, &outputs)?;
    debug!("Signed inputs >>> {:?}", signed_inputs);

    let fees = Fees::new(signed_inputs, outputs);
    trace!("Created fees structure.");

    return Ok(fees);
}


#[cfg(test)]
mod test_deserialize_inputs {
    use indy::ErrorCode;
    use libc::c_char;
    use std::ptr;
    use utils::ffi_support::{c_pointer_from_string, c_pointer_from_str};
    use super::{deserialize_inputs, AddRequestFeesCb, DeserializedArguments};

    fn call_deserialize_inputs(
        req_json: Option<*const c_char>,
        inputs_json: Option<*const c_char>,
        outputs_json: Option<*const c_char>,
        cb: Option<Option<AddRequestFeesCb>>
    ) -> Result<DeserializedArguments, ErrorCode> {
        let son = json!({
            "protocolVersion": 1,
            "operation": {
                "type": 2,
            }
        });
        let default_req_json = c_pointer_from_string(son.to_string());

        let default_inputs_json = c_pointer_from_string(json!([
            {
                "paymentAddress": "pay:sov:d0kitWxupHvZ4i0NHJhoj79RcUeyt3YlwAc8Hbcy87iRLSZC",
                "sequenceNumber": 2
            },
            {
                "paymentAddress": "pay:sov:XuBhXW6gKcUAq6fmyKsdxxjOZEbLy66FEDkQwTPeoXBmTZKy",
                "sequenceNumber": 3
            }
        ]).to_string());

        let default_outputs_json = c_pointer_from_string(json!([
            {
                "paymentAddress": "pay:sov:ql33nBkjGw6szxPT6LLRUIejn9TZAYkVRPd0QJzfJ8FdhZWs",
                "amount": 10
            }
        ]).to_string());

        extern fn default_callback(_: i32, _: ErrorCode, _: *const c_char) -> ErrorCode {ErrorCode::Success};
        let req_json = req_json.unwrap_or(default_req_json);
        let inputs_json = inputs_json.unwrap_or(default_inputs_json);
        let outputs_json = outputs_json.unwrap_or(default_outputs_json);
        let cb = cb.unwrap_or(Some(default_callback));

        return deserialize_inputs(req_json, inputs_json, outputs_json, cb);
    }

    fn error_deserialize_inputs_inputs(inputs: *const c_char, error: ErrorCode) {
        let result = call_deserialize_inputs(None, Some(inputs), None, None);
        assert_eq!(error, result.unwrap_err());
    }

    fn error_deserialize_inputs_ouputs(outputs: *const c_char, error: ErrorCode) {
        let result = call_deserialize_inputs(None, None, Some(outputs), None);
        assert_eq!(error, result.unwrap_err());
    }

    fn error_deserialize_inputs_request(request: *const c_char, error: ErrorCode) {
        let result = call_deserialize_inputs(Some(request), None, None, None);
        assert_eq!(error, result.unwrap_err());
    }

    fn error_deserialize_inputs_cb(cb: Option<AddRequestFeesCb>, error: ErrorCode) {
        let result = call_deserialize_inputs(None, None, None, Some(cb));
        assert_eq!(error, result.unwrap_err());
    }

    #[test]
    fn deserialize_inputs_empty_inputs() {
        error_deserialize_inputs_inputs(ptr::null(), ErrorCode::CommonInvalidStructure);
    }

    #[test]
    fn deserialize_inputs_empty_outputs() {
        error_deserialize_inputs_ouputs(ptr::null(), ErrorCode::CommonInvalidStructure);
    }

    #[test]
    fn deserialize_inputs_empty_request() {
        error_deserialize_inputs_request(ptr::null(), ErrorCode::CommonInvalidStructure);
    }

    #[test]
    fn deserialize_inputs_empty_callback() {
        error_deserialize_inputs_cb(None, ErrorCode::CommonInvalidStructure);
    }

    #[test]
    fn deserialize_inputs_invalid_inputs_json() {
        let invalid_json = c_pointer_from_string(json!([
            {
                "paymentAddre": "pay:sov:d0kitWxupHvZ4i0NHJhoj79RcUeyt3YlwAc8Hbcy87iRLSZC",
                "sequenceNumber": 4
            }
        ]).to_string());
        error_deserialize_inputs_inputs(invalid_json, ErrorCode::CommonInvalidStructure);
    }

    #[test]
    fn deserialize_inputs_invalid_outputs_json() {
        let invalid_json = c_pointer_from_string(json!([
            {
                "paymentAddress": "pay:sov:ql33nBkjGw6szxPT6LLRUIejn9TZAYkVRPd0QJzfJ8FdhZWs",
                "amount": "10"
            }
        ]).to_string());
        error_deserialize_inputs_ouputs(invalid_json, ErrorCode::CommonInvalidStructure);
    }

    #[test]
    fn deserialize_inputs_invalid_request_json() {
        let invalid_json = c_pointer_from_str("[]");
        error_deserialize_inputs_request(invalid_json, ErrorCode::CommonInvalidStructure);
    }

    #[test]
    fn deserialize_inputs_valid() {
        let result = call_deserialize_inputs(None, None, None, None);
        assert!(result.is_ok());
    }
}