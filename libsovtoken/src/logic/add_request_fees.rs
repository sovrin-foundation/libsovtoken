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