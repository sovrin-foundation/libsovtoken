//! TODO ???

use ErrorCode;
use libc::c_char;
use logic::xfer_payload::{XferPayload, serialize_signature};
use logic::input::Inputs;
use logic::output::Outputs;
use serde_json;
use utils::ffi_support::{string_from_char_ptr, c_pointer_from_string, c_pointer_from_str};
use logic::indy_sdk_api::crypto_api::CryptoSdk;
use utils::constants::txn_types::XFER_PUBLIC;
use utils::constants::txn_fields::FEES;
use utils::constants::general::JsonCallbackUnwrapped;
use sha2::{Sha256, Digest};
use hex::ToHex;

type SerdeMap = serde_json::Map<String, serde_json::value::Value>;
type AddRequestFeesCb = extern fn(command_handle_: i32, err: i32, req_with_fees_json: *const c_char) -> i32;
type DeserializedArguments = (Inputs, Outputs, Option<String>, SerdeMap, AddRequestFeesCb);

/**
 * Deserializes arguments of [`add_request_fees_handler`]
 */
pub fn deserialize_inputs (
    req_json: *const c_char,
    inputs_json: *const c_char,
    outputs_json: *const c_char,
    extra: *const c_char,
    cb: Option<AddRequestFeesCb>
) -> Result<DeserializedArguments, ErrorCode> {
    debug!("logic::add_request_fees::deserialize_inputs >> req_json: {:?}, inputs_json: {:?}, outputs_json: {:?}", req_json, inputs_json, outputs_json);

    let cb = cb.ok_or(ErrorCode::CommonInvalidStructure).map_err(map_err_err!())?;

    let request_json = string_from_char_ptr(req_json).ok_or(ErrorCode::CommonInvalidStructure).map_err(map_err_err!())?;
    debug!("Converted request_json pointer into string >>> {:?}", request_json);

    let inputs_json = string_from_char_ptr(inputs_json).ok_or(ErrorCode::CommonInvalidStructure).map_err(map_err_err!())?;
    debug!("Converted inputs_json pointer to string >>> {:?}", inputs_json);

    let outputs_json = string_from_char_ptr(outputs_json).ok_or(ErrorCode::CommonInvalidStructure).map_err(map_err_err!())?;
    debug!("Converted outputs_json pointer to string >>> {:?}", outputs_json);

    let extra = string_from_char_ptr(extra);
    debug!("Converted extra pointer to string >>> {:?}", extra);

    let inputs: Inputs = serde_json::from_str(&inputs_json).map_err(map_err_err!()).or(Err(ErrorCode::CommonInvalidStructure))?;
    debug!("Deserialized input_json >>> {:?}", inputs);

    let outputs: Outputs = serde_json::from_str(&outputs_json).map_err(map_err_err!()).or(Err(ErrorCode::CommonInvalidStructure))?;
    debug!("Deserialized output_json >>> {:?}", outputs);

    let request_json_object: serde_json::Value = serde_json::from_str(&request_json).map_err(map_err_err!()).or(Err(ErrorCode::CommonInvalidStructure))?;
    trace!("Converted request_json to serde::json::Value");

    let request_json_map = request_json_object.as_object().ok_or(ErrorCode::CommonInvalidStructure).map_err(map_err_err!())?;
    trace!("Converted request_json to hash_map");

    debug!("Deserialized values: inputs: {:?}, outputs: {:?}, request_json_map: {:?}", inputs, outputs, request_json_map);
    return Ok((
        inputs,
        outputs,
        extra,
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
    extra: Option<String>,
    request_json_map: SerdeMap,
    cb: Box<Fn(Result<String, ErrorCode>) + Send + Sync>
) -> Result<(), ErrorCode> {
    trace!("logic::add_request_fees::add_fees_to_request_and_serialize >> wallet_handle: {:?}, inputs: {:?}, outputs: {:?}, request_json_map: {:?}", wallet_handle, inputs, outputs, request_json_map);
    let res = add_fees(wallet_handle, inputs, outputs, extra, request_json_map, Box::new(move |request_json_map_updated|{
        let rm_fees = request_json_map_updated.map(|request_json_map_with_fees| serialize_request_with_fees(request_json_map_with_fees));
        match rm_fees {
            Ok(some) => cb(some),
            Err(e) => cb(Err(e))
        }
    }));
    trace!("logic::add_request_fees::add_fees_to_request_and_serialize >> result: {:?}", res);
    res
}

/**
Creates a callback for when the signing is complete and fees are added.
*/
pub fn closure_cb_response(command_handle: i32, cb: JsonCallbackUnwrapped) -> impl Fn(Result<String, ErrorCode>) {
    move |res| {
        trace!("add_request_fees::closure_cb_response Request with fees >> {:?}", res);
        match res {
            Ok(res) => cb(command_handle, ErrorCode::Success as i32, c_pointer_from_string(res)),
            Err(e) => cb(command_handle, e as i32, c_pointer_from_str("")),
        };
    }    
}


/*
    Methods "private" (aka not exported from this module)

    KEEP all public methods above
*/

fn add_fees(wallet_handle: i32, inputs: Inputs, outputs: Outputs, extra: Option<String>, request_json_map: SerdeMap, cb: Box<Fn(Result<SerdeMap, ErrorCode>) + Send + Sync>) -> Result<(), ErrorCode> {
    let txn_serialized = serialize_signature(request_json_map.clone().into())?;
    let mut hasher = Sha256::default();
    hasher.input(txn_serialized.as_bytes());
    let txn_digest = Some(hasher.result().to_hex());
    signed_fees(wallet_handle, inputs, outputs, extra, &txn_digest, Box::new(move |fees| {
        trace!("Added fees to request_json.");
        match fees {
            Ok(fees) => {
                let mut map = request_json_map.clone();
                map.insert(FEES.to_string(), json!([fees.inputs, fees.outputs, fees.signatures]));
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

fn signed_fees(wallet_handle: i32, inputs: Inputs, outputs: Outputs, extra: Option<String>, txn_digest: &Option<String>, cb: Box<Fn(Result<XferPayload, ErrorCode>) + Send + Sync>) -> Result<(), ErrorCode> {
    let fees = XferPayload::new(inputs, outputs, extra);
    fees.sign_fees(&CryptoSdk{}, wallet_handle, txn_digest, cb)?;
    Ok(())
}


#[cfg(test)]
mod test_deserialize_inputs {
    use libc::c_char;
    use ErrorCode;
    use serde_json;
    use std::ptr;
    use utils::constants::txn_types::XFER_PUBLIC;
    use utils::test::default;

    use super::{deserialize_inputs, AddRequestFeesCb, DeserializedArguments};
    use super::validate_type_not_transfer;

    pub fn call_deserialize_inputs(
        req_json: Option<*const c_char>,
        inputs_json: Option<*const c_char>,
        outputs_json: Option<*const c_char>,
        extra: Option<*const c_char>,
        cb: Option<Option<AddRequestFeesCb>>
    ) -> Result<DeserializedArguments, ErrorCode> {
        let default_req_json = json_c_pointer!({
            "protocolVersion": 2,
            "operation": {
                "type": 2,
            }
        });

        let req_json = req_json.unwrap_or(default_req_json);
        let inputs_json = inputs_json.unwrap_or_else(default::inputs_json_pointer);
        let outputs_json = outputs_json.unwrap_or_else(default::outputs_json_pointer);
        let extra = extra.unwrap_or(ptr::null());
        let cb = cb.unwrap_or(Some(default::empty_callback_string));

        return deserialize_inputs(req_json, inputs_json, outputs_json, extra, cb);
    }

    fn error_deserialize_inputs_inputs(inputs: *const c_char, error: ErrorCode) {
        let result = call_deserialize_inputs(None, Some(inputs), None, None, None);
        assert_eq!(error, result.unwrap_err());
    }

    fn error_deserialize_inputs_ouputs(outputs: *const c_char, error: ErrorCode) {
        let result = call_deserialize_inputs(None, None, Some(outputs), None, None);
        assert_eq!(error, result.unwrap_err());
    }

    fn error_deserialize_inputs_request(request: *const c_char, error: ErrorCode) {
        let result = call_deserialize_inputs(Some(request), None, None, None, None);
        assert_eq!(error, result.unwrap_err());
    }

    fn error_deserialize_inputs_cb(cb: Option<AddRequestFeesCb>, error: ErrorCode) {
        let result = call_deserialize_inputs(None, None, None, None, Some(cb));
        assert_eq!(error, result.unwrap_err());
    }

    fn deserialize_request_json(json_pointer: *const c_char) -> serde_json::Map<String, serde_json::value::Value> {
        let (_, _, _, request, _) = call_deserialize_inputs(Some(json_pointer), None, None, None, None).unwrap();
        return request;
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
        let invalid_json = json_c_pointer!([
            {
                "addres": "pay:sov:d0kitWxupHvZ4i0NHJhoj79RcUeyt3YlwAc8Hbcy87iRLSZC",
                "seqNo": 4
            }
        ]);
        error_deserialize_inputs_inputs(invalid_json, ErrorCode::CommonInvalidStructure);
    }

    #[test]
    fn deserialize_inputs_invalid_outputs_json() {
        let invalid_json = json_c_pointer!([
            {
                "address": "pay:sov:ql33nBkjGw6szxPT6LLRUIejn9TZAYkVRPd0QJzfJ8FdhZWs",
                "amount": "10"
            }
        ]);
        error_deserialize_inputs_ouputs(invalid_json, ErrorCode::CommonInvalidStructure);
    }

    #[test]
    fn deserialize_inputs_invalid_request_json() {
        let invalid_json = json_c_pointer!([]);
        error_deserialize_inputs_request(invalid_json, ErrorCode::CommonInvalidStructure);
    }

    #[test]
    fn deserialize_inputs_valid() {
        let result = call_deserialize_inputs(None, None, None, None, None);
        assert!(result.is_ok());
    }

    #[test]
    fn no_operation_in_request() {
        let request = deserialize_request_json(json_c_pointer!({
            "wise_advice": "When life gives you lemons, squeeze them in someone's eyes."
        }));
        let error = validate_type_not_transfer(&request).unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, error);
    }

    #[test]
    fn no_txn_type_in_operation() {
        let request = deserialize_request_json(json_c_pointer!({
            "operation": {
                "remove_wisdom_teeth": "painful"
            }
        }));
        let error = validate_type_not_transfer(&request).unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, error);
    }

    #[test]
    fn txn_type_is_transfer() {
        let request = deserialize_request_json(json_c_pointer!({
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
        let request = deserialize_request_json(json_c_pointer!({
            "operation": {
                "type": typ
            },
            "extra_field": "Doesn't check it is known request."
        }));

        let validated = validate_type_not_transfer(&request);
        assert!(validated.is_ok());
    }
}

#[cfg(test)]
mod closure_cb_response_test {
    use super::*;
    use utils::test::callbacks;
    use std::sync::mpsc::RecvError;

    fn call_callback(result: Result<String, ErrorCode>)
        -> Result<(ErrorCode, String), RecvError>
    {
        let (receiver, command_handle, cb) = callbacks::cb_ec_string();
        closure_cb_response(command_handle, cb.unwrap())(result);
        receiver.recv()
    }
    
    #[test]
    fn test_cb_called_on_error() {
        let result = call_callback(Err(ErrorCode::CommonInvalidState)).unwrap();
        assert_eq!(ErrorCode::CommonInvalidState, result.0);
        assert_eq!(String::from(""), result.1);
    }

    #[test]
    fn test_cb_called_on_success() {
        let result = call_callback(Ok(String::from("Heyahh"))).unwrap();
        assert_eq!(ErrorCode::Success, result.0);
        assert_eq!(String::from("Heyahh"), result.1);
    }
}
