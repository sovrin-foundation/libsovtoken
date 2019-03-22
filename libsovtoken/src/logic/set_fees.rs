//! This module is for ???

use ErrorCode;
use libc::c_char;
use logic::config::set_fees_config::{SetFees, SetFeesMap};
use logic::did::Did;
use serde_json;
use utils::constants::general::{JsonCallback, JsonCallbackUnwrapped};
use utils::ffi_support::string_from_char_ptr;

const NYM: &'static str = "1";
const ATTRIB: &'static str = "100";
const SCHEMA: &'static str = "101";
const CRED_DEF: &'static str = "102";
const REVOC_REG_DEF: &'static str = "113";
const REVOC_REG_ENTRY: &'static str = "114";
const XFER_PUBLIC: &'static str = "10001";

fn txn_name_to_code(txn: &str) -> String {
    match txn {
        "NYM" => NYM.to_string(),
        "ATTRIB" => ATTRIB.to_string(),
        "SCHEMA" => SCHEMA.to_string(),
        "CRED_DEF" => CRED_DEF.to_string(),
        "REVOC_REG_DEF" => REVOC_REG_DEF.to_string(),
        "REVOC_REG_ENTRY" => REVOC_REG_ENTRY.to_string(),
        "XFER_PUBLIC" => XFER_PUBLIC.to_string(),
        val @ _ => val.to_string()
    }
}

type DeserializedArguments<'a> = (Option<Did<'a>>, SetFees, JsonCallbackUnwrapped);

pub fn deserialize_inputs<'a>(
    did: *const c_char,
    fees_json: *const c_char,
    cb: JsonCallback
) -> Result<DeserializedArguments<'a>, ErrorCode> {
    trace!("logic::set_fees::deserialize_inputs >> did: {:?}, fees_json: {:?}", did, fees_json);
    let cb = cb.ok_or(ErrorCode::CommonInvalidStructure)?;

    let did = Did::from_pointer(did).map(|did| {
        did.validate().or(Err(ErrorCode::CommonInvalidStructure))
    });

    let did = opt_res_to_res_opt!(did)?;

    let set_fees_json = string_from_char_ptr(fees_json)
        .ok_or(ErrorCode::CommonInvalidStructure).map_err(map_err_err!())?;

    let set_fees_map: SetFeesMap = serde_json::from_str(&set_fees_json).map_err(map_err_err!())
        .or(Err(ErrorCode::CommonInvalidStructure))?;

    let set_fees_map: SetFeesMap = set_fees_map.iter()
        .map(|(key, val)| (txn_name_to_code(key), val.clone())).collect();

    let set_fees = SetFees::new(set_fees_map)
        .validate().map_err(map_err_err!())
        .or(Err(ErrorCode::CommonInvalidStructure))?;

    let res = Ok((did, set_fees, cb));
    trace!("logic::set_fees::deserialize_inputs << res: {:?}", res);
    return res;
}

#[cfg(test)]
mod test_deserialize_inputs {
    use super::*;
    use std::ptr;
    use utils::test::default;
    use utils::ffi_support::{c_pointer_from_str};

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
        let (did, _, _) = result.unwrap();
        assert_eq!(None, did);
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
        let invalid_fees = json_c_pointer!({
            "fees" : {
                "4": 2,
                "20000": 5,
            }
        });

        let result = call_deserialize_inputs(None, Some(invalid_fees), None);

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn deserialize_invalid_fees_string_values() {
        let invalid_fees = json_c_pointer!({
            "4": "2",
            "20000": "5",
        });

        let result = call_deserialize_inputs(None, Some(invalid_fees), None);

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn deserialize_fees_key_not_string_int() {
        let invalid_fees = json_c_pointer!({
            "XFER_PUBLIC": 5,
            "3": 1,
        });

        let (_, fees, _) = call_deserialize_inputs(None, Some(invalid_fees), None).unwrap();

        assert_eq!(fees.fees.len(), 2);
        assert_eq!(fees.fees.get("10001"), Some(&5));
        assert_eq!(fees.fees.get("3"), Some(&1));
    }

    #[test]
    fn deserialize_valid_arguments() {
        let result = call_deserialize_inputs(None, None, None);
        assert!(result.is_ok());
    }
}