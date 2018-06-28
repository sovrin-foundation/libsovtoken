//! This module is for ???

use api::{JsonCallback, JsonCallbackUnwrapped};
use indy::ErrorCode;
use libc::c_char;
use logic::config::set_fees_config::{SetFees, SetFeesMap};
use logic::did::Did;
use serde_json;
use utils::ffi_support::string_from_char_ptr;


type DeserializedArguments<'a> = (Did<'a>, SetFees, JsonCallbackUnwrapped);

pub fn deserialize_inputs<'a>(
    did: *const c_char,
    fees_json: *const c_char,
    cb: JsonCallback
) -> Result<DeserializedArguments<'a>, ErrorCode> {
    let cb = cb.ok_or(ErrorCode::CommonInvalidStructure)?;

    let did = Did::from_pointer(did)
        .ok_or(ErrorCode::CommonInvalidStructure)?
        .validate()
        .or(Err(ErrorCode::CommonInvalidStructure))?;

    let set_fees_json = string_from_char_ptr(fees_json)
        .ok_or(ErrorCode::CommonInvalidStructure)?;

    let set_fees_map: SetFeesMap = serde_json::from_str(&set_fees_json)
        .or(Err(ErrorCode::CommonInvalidStructure))?;

    let set_fees = SetFees::new(set_fees_map)
        .validate()
        .or(Err(ErrorCode::CommonInvalidStructure))?;

    return Ok((did, set_fees, cb));
}

#[cfg(test)]
mod test_deserialize_inputs {
    use super::*;
    use std::ptr;
    use utils::default;
    use utils::ffi_support::{c_pointer_from_str, c_pointer_from_string};

    pub fn call_deserialize_inputs<'a>(
        did: Option<*const c_char>,
        set_fees_json: Option<*const c_char>,
        cb: Option<JsonCallback>
    ) -> Result<DeserializedArguments<'a>, ErrorCode> {
        let did_json = did.unwrap_or_else(default::did);
        let set_fees_json = set_fees_json.unwrap_or_else(default::set_fees_json);
        let cb = cb.unwrap_or(Some(default::empty_callback_string));

        return deserialize_inputs(did_json, set_fees_json, cb);
    }

    #[test]
    fn deserialize_empty_did() {
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
    fn deserialize_did_invalid_length() {
        let did = c_pointer_from_str("MyFakeDidWithALengthThatIsTooLong");
        let result = call_deserialize_inputs(Some(did), None, None);
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn deserialize_invalid_fees_encapsulated() {
        let invalid_fees = c_pointer_from_string(json!({
            "fees" : {
                "4": 2,
                "20000": 5,
            }
        }).to_string());

        let result = call_deserialize_inputs(None, Some(invalid_fees), None);

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn deserialize_invalid_fees_string_values() {
        let invalid_fees = c_pointer_from_string(json!({
            "4": "2",
            "20000": "5",
        }).to_string());

        let result = call_deserialize_inputs(None, Some(invalid_fees), None);

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn deserialize_invalid_fees_key_not_string_int() {
        let invalid_fees = c_pointer_from_string(json!({
            "XFER_PUBLIC": 5,
            "3": 1,
        }).to_string());

        let result = call_deserialize_inputs(None, Some(invalid_fees), None);

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn deserialize_valid_arguments() {
        let result = call_deserialize_inputs(None, None, None);
        assert!(result.is_ok());
    }
}