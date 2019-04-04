use ErrorCode;
use libc::c_char;
use logic::address;
use logic::config::output_mint_config::MintRequest;
use logic::did::Did;
use serde_json;
use utils::constants::general::{JsonCallback, JsonCallbackUnwrapped};
use utils::ffi_support::{string_from_char_ptr};
use logic::output::Outputs;

type DeserializedArguments<'a> = (Option<Did<'a>>, Outputs, Option<String>, JsonCallbackUnwrapped);

pub fn deserialize_inputs<'a>(
    did: *const c_char,
    outputs_json: *const c_char,
    extra: *const c_char,
    cb: JsonCallback
) -> Result<DeserializedArguments<'a>, ErrorCode> {
    trace!("logic::minting::deserialize_inputs >> did: {:?}, outputs_json: {:?}, extra: {:?}", did, outputs_json, extra);
    let cb = cb.ok_or(ErrorCode::CommonInvalidStructure)?;
    trace!("Unwrapped callback.");

    let did = Did::from_pointer(did).map(
        |did| {
            did.validate().map_err(map_err_err!()).or(Err(ErrorCode::CommonInvalidStructure))
        }
    );
    let did = opt_res_to_res_opt!(did)?;
    debug!("Converted did pointer to string >>> {:?}", did);

    let outputs_json = string_from_char_ptr(outputs_json)
        .ok_or(ErrorCode::CommonInvalidStructure)?;
    debug!("Converted outputs_json pointer to string >>> {:?}", outputs_json);

    let outputs: Outputs = serde_json::from_str(&outputs_json)
        .or(Err(ErrorCode::CommonInvalidStructure))?;
    debug!("Deserialized output_json >>> {:?}", outputs);

    let extra = string_from_char_ptr(extra);
    debug!("Deserialized extra >>> {:?}", extra);

    trace!("logic::minting::deserialize_inputs << did: {:?}, outputs: {:?}, extra: {:?}", did, outputs, extra);
    return Ok((did, outputs, extra, cb));
}

pub fn build_mint_request(
    did: Option<Did>,
    mut outputs: Outputs,
    extra: Option<String>,
) -> Result<*const c_char, ErrorCode> {
    trace!("logic::minting::build_mint_request >> did: {:?}, outputs: {:?}", did, outputs);

    for output in &mut outputs {
        let address = address::unqualified_address_from_address(&output.recipient)?;
        output.recipient = address;
    }
    trace!("Stripped pay:sov: from outputs");

    let mint_request = MintRequest::from_config(outputs, did, extra);
    info!("Built a mint request >>> {:?}", mint_request);

    let ptr = mint_request.serialize_to_pointer()
        .or(Err(ErrorCode::CommonInvalidStructure));

    trace!("logic::minting::build_mint_request << res: {:?}", ptr);
    ptr
}

#[cfg(test)]
mod test_build_mint_request {
    use super::*;
    use std::ptr::null;
    use logic::output::Output;
    use utils::base58::IntoBase58;
    use utils::constants::txn_types::MINT_PUBLIC;
    use utils::ffi_support::{c_pointer_from_str};
    use utils::test::default;

    pub fn call_deserialize_inputs<'a>(
        did: Option<*const c_char>,
        outputs_json: Option<*const c_char>,
        extra: Option<*const c_char>,
        cb: Option<JsonCallback>
    ) -> Result<DeserializedArguments<'a>, ErrorCode> {
        let req_json = did.unwrap_or_else(default::did);
        let outputs_json = outputs_json.unwrap_or_else(default::outputs_json_pointer);
        let extra = extra.unwrap_or(null());
        let cb = cb.unwrap_or(Some(default::empty_callback_string));

        return deserialize_inputs(req_json, outputs_json, extra, cb);
    }

    #[test]
    fn build_mint_request_invalid_address() {
        let outputs = vec![
            Output::new(String::from("pad:sov:E9LNHk8shQ6xe2RfydzXDSsyhWC6vJaUeKE2mmc6mWraDfmKm"), 12)
        ];

        let did = Did::new(&"en32ansFeZNERIouv2xA");
        let result = build_mint_request(Some(did), outputs, None);
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn build_mint_request_valid() {
        let output_config_pointer = json_c_pointer!([{
            "address": "pay:sov:E9LNHk8shQ6xe2RfydzXDSsyhWC6vJaUeKE2mmc6mWraDfmKm",
            "amount": 12
        }]);

        let did_str = &"1123456789abcdef".as_bytes().into_base58();
        let (did, output_config, _, _) = call_deserialize_inputs(
            Some(c_pointer_from_str(did_str)),
            Some(output_config_pointer),
            None,
            None
        ).unwrap();

        let result = build_mint_request(did.into(), output_config, None).unwrap();
        let mint_request_json = string_from_char_ptr(result).unwrap();
        let mint_value: serde_json::value::Value = serde_json::from_str(&mint_request_json).unwrap();

        let expected = json!({
            "identifier": did_str,
            "operation": {
                "type": MINT_PUBLIC,
                "outputs": [
                    {"address":"E9LNHk8shQ6xe2RfydzXDSsyhWC6vJaUeKE2mmc6mWraDfmKm", "amount":12}
                ]
            },
        });

        assert_eq!(expected.get("operation"), mint_value.get("operation"));
        assert_eq!(expected.get("identifier"), mint_value.get("identifier"));
    }
}

#[cfg(test)]
mod test_deserialize_inputs {
    use super::*;
    use std::ptr;
    use self::test_build_mint_request::call_deserialize_inputs;
    use utils::ffi_support::{c_pointer_from_str};


    #[test]
    fn deserialize_empty_did() {
        let result = call_deserialize_inputs(Some(ptr::null()), None, None, None);
        let (did, _, _, _) = result.unwrap();
        assert_eq!(None, did);
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
    fn deserialize_did_invalid_length() {
        let did = c_pointer_from_str("MyFakeDidWithALengthThatIsTooLong");
        let result = call_deserialize_inputs(Some(did), None, None, None);
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn deserialize_outputs_invalid_structure() {
        // Invalid because there is no ver field.
        let outputs = json_c_pointer!({
            "outputs": [
                {
                    "address": "pay:sov:E9LNHk8shQ6xe2RfydzXDSsyhWC6vJaUeKE2mmc6mWraDfmKm",
                    "amount": 10
                }
            ]
        });
        let result = call_deserialize_inputs(None, Some(outputs), None, None);
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn deserialize_valid_arguments() {
        let result = call_deserialize_inputs(None, None, None, None);
        assert!(result.is_ok());
    }

}