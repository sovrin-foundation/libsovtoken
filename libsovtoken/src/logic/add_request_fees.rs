//! TODO ???

use indy::ErrorCode;
use libc::c_char;
use logic::xfer_payload::XferPayload;
use logic::input::Inputs;
use logic::output::Outputs;
use serde_json;
use utils::ffi_support::{string_from_char_ptr};
use logic::indy_sdk_api::crypto_api::CryptoSdk;
use utils::constants::txn_types::XFER_PUBLIC;

type SerdeMap = serde_json::Map<String, serde_json::value::Value>;
type AddRequestFeesCb = extern fn(command_handle_: i32, err: i32, req_with_fees_json: *const c_char) -> i32;
type DeserializedArguments = (Inputs, Outputs, SerdeMap, AddRequestFeesCb);

pub static FEES_KEY_IN_REQ_RESP: &'static str = "fees";

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
    debug!("Deserialized input_json >>> {:?}", inputs);

    let outputs: Outputs = serde_json::from_str(&outputs_json).or(Err(ErrorCode::CommonInvalidStructure))?;
    debug!("Deserialized output_json >>> {:?}", outputs);

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

    if transaction_type == XFER_PUBLIC {
        return Err(ErrorCode::CommonInvalidStructure);
    } else {
        return Ok(());
    };
}

pub fn add_fees_to_request_and_serialize(
    wallet_handle: i32,
    inputs: Inputs,
    outputs: Outputs,
    request_json_map: SerdeMap,
    cb: Box<Fn(Result<String, ErrorCode>) + Send + Sync>
) -> Result<(), ErrorCode> {
    add_fees(wallet_handle, inputs, outputs, request_json_map, Box::new(move |request_json_map_updated|{
        let rm_fees = request_json_map_updated.map(|request_json_map_with_fees| serialize_request_with_fees(request_json_map_with_fees));
        match rm_fees {
            Ok(some) => cb(some),
            Err(e) => cb(Err(e))
        }
    }))?;
    Ok(())
}


/*
    Methods "private" (aka not exported from this module)

    KEEP all public methods above
*/

fn add_fees(wallet_handle: i32, inputs: Inputs, outputs: Outputs, request_json_map: SerdeMap, cb: Box<Fn(Result<SerdeMap, ErrorCode>) + Send + Sync>) -> Result<(), ErrorCode> {
    signed_fees(wallet_handle, inputs, outputs, Box::new(move |fees| {
        trace!("Added fees to request_json.");
        match fees {
            Ok(fees) => {
                let mut map = request_json_map.clone();
                map.insert(FEES_KEY_IN_REQ_RESP.to_string(), json!(fees));
                cb(Ok(map.clone()));
            }
            Err(err) => {
                cb(Err(err))
            }
        }
    }))?;

    Ok(())
}

fn serialize_request_with_fees(request_json_map_with_fees: SerdeMap) -> Result<String, ErrorCode> {
    trace!("fee_map: {:?}", request_json_map_with_fees);
    let serialized_request_with_fees = serde_json::to_string(&json!(request_json_map_with_fees))
        .or(Err(ErrorCode::CommonInvalidStructure))?;
    trace!("Serialized request_with_fees");
    
    return Ok(serialized_request_with_fees);
} 

fn signed_fees(wallet_handle: i32, inputs: Inputs, outputs: Outputs, cb: Box<Fn(Result<XferPayload, ErrorCode>) + Send + Sync>) -> Result<(), ErrorCode> {
    let fees = XferPayload::new(inputs, outputs);
    fees.sign(&CryptoSdk{}, wallet_handle, cb)?;
    Ok(())
}


#[cfg(test)]
mod test_deserialize_inputs {
    use indy::ErrorCode;
    use libc::c_char;
    use std::ptr;
    use utils::default;
    use utils::ffi_support::{c_pointer_from_string, c_pointer_from_str};
    use super::{deserialize_inputs, AddRequestFeesCb, DeserializedArguments};

    pub fn call_deserialize_inputs(
        req_json: Option<*const c_char>,
        inputs_json: Option<*const c_char>,
        outputs_json: Option<*const c_char>,
        cb: Option<Option<AddRequestFeesCb>>
    ) -> Result<DeserializedArguments, ErrorCode> {
        let default_req_json = c_pointer_from_string(json!({
            "protocolVersion": 1,
            "operation": {
                "type": 2,
            }
        }).to_string());

        let req_json = req_json.unwrap_or(default_req_json);
        let inputs_json = inputs_json.unwrap_or_else(default::inputs_json_pointer);
        let outputs_json = outputs_json.unwrap_or_else(default::outputs_json_pointer);
        let cb = cb.unwrap_or(Some(default::empty_callback_string));

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
                "addres": "pay:sov:d0kitWxupHvZ4i0NHJhoj79RcUeyt3YlwAc8Hbcy87iRLSZC",
                "seqNo": 4
            }
        ]).to_string());
        error_deserialize_inputs_inputs(invalid_json, ErrorCode::CommonInvalidStructure);
    }

    #[test]
    fn deserialize_inputs_invalid_outputs_json() {
        let invalid_json = c_pointer_from_string(json!([
            {
                "address": "pay:sov:ql33nBkjGw6szxPT6LLRUIejn9TZAYkVRPd0QJzfJ8FdhZWs",
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

    use env_logger;

    #[test]
    fn deserialize_inputs_valid() {
        env_logger::init();
        let result = call_deserialize_inputs(None, None, None, None);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod test_validate_type_not_transfer {
    use indy::ErrorCode;
    use serde_json;
    use super::validate_type_not_transfer;
    use super::test_deserialize_inputs::call_deserialize_inputs;
    use utils::ffi_support::c_pointer_from_string;
    use utils::constants::txn_types::XFER_PUBLIC;

    fn deserialize_request_json(json: serde_json::value::Value) -> serde_json::Map<String, serde_json::value::Value> {
        let request_c_pointer = c_pointer_from_string(json.to_string());
        let (_, _, request, _) = call_deserialize_inputs(Some(request_c_pointer), None, None, None).unwrap();
        return request;
    }

    #[test]
    fn no_operation_in_request() {
        let request = deserialize_request_json(json!({
            "wise_advice": "When life gives you lemons, squeeze them in someone's eyes."
        }));
        let error = validate_type_not_transfer(&request).unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, error);
    }

    #[test]
    fn no_txn_type_in_operation() {
        let request = deserialize_request_json(json!({
            "operation": {
                "remove_wisdom_teeth": "painful"
            }
        }));
        let error = validate_type_not_transfer(&request).unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, error);
    }

    #[test]
    fn txn_type_is_transfer() {
        let request = deserialize_request_json(json!({
            "operation": {
                "type": XFER_PUBLIC,
            }
        }));
        let error = validate_type_not_transfer(&request).unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, error);
    }

    #[test]
    fn txn_type_not_transfer() {
        let typ = "30000";
        assert_ne!(typ, XFER_PUBLIC);
        let request = deserialize_request_json(json!({
            "operation": {
                "type": typ
            },
            "extra_field": "Doesn't check it is known request."
        }));

        let validated = validate_type_not_transfer(&request);
        assert!(validated.is_ok());
    }
}
