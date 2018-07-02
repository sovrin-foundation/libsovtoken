use api::{JsonCallback, JsonCallbackUnwrapped};
use indy::ErrorCode;
use libc::c_char;
use logic::address;
use logic::config::output_mint_config::MintRequest;
use logic::did::Did;
use serde_json;
use utils::ffi_support::{string_from_char_ptr};
use logic::output::Outputs;

type DeserializedArguments<'a> = (Did<'a>, Outputs, JsonCallbackUnwrapped);

pub fn deserialize_inputs<'a>(
    did: *const c_char,
    outputs_json: *const c_char,
    cb: JsonCallback
) -> Result<DeserializedArguments<'a>, ErrorCode> {
    trace!("logic::minting::deserialize_inputs >> did: {:?}, outputs_json: {:?}", did, outputs_json);
    let cb = cb.ok_or(ErrorCode::CommonInvalidStructure)?;
    trace!("Unwrapped callback.");

    let did = Did::from_pointer(did)
        .ok_or(ErrorCode::CommonInvalidStructure)?
        .validate()
        .or(Err(ErrorCode::CommonInvalidStructure))?;
    debug!("Converted did pointer to string >>> {:?}", did);

    let outputs_json = string_from_char_ptr(outputs_json)
        .ok_or(ErrorCode::CommonInvalidStructure)?;
    debug!("Converted outputs_json pointer to string >>> {:?}", outputs_json);

    let outputs: Outputs = serde_json::from_str(&outputs_json)
        .or(Err(ErrorCode::CommonInvalidStructure))?;
    debug!("Deserialized output_json >>> {:?}", outputs);

    trace!("logic::minting::deserialize_inputs << did: {:?}, outputs: {:?}", did, outputs);
    return Ok((did, outputs, cb));
}

pub fn build_mint_request(
    did: Did,
    mut outputs: Outputs
) -> Result<*const c_char, ErrorCode> {
    trace!("logic::minting::build_mint_request >> did: {:?}, outputs: {:?}", did, outputs);

    for output in &mut outputs {
        let address = address::unqualified_address_from_address(&output.address)?;
        output.address = address;
    }
    trace!("Stripped pay:sov: from outputs");

    let mint_request = MintRequest::from_config(outputs, did);
    info!("Built a mint request >>> {:?}", mint_request);

    let ptr = mint_request.serialize_to_pointer()
        .or(Err(ErrorCode::CommonInvalidStructure));

    trace!("logic::minting::build_mint_request << res: {:?}", ptr);
    ptr
}

#[cfg(test)]
mod test_build_mint_request {
    use std::ptr;
    use rust_base58::ToBase58;

    use super::*;

    use logic::output::Output;
    use utils::default;
    use utils::constants::txn_types::MINT_PUBLIC;
    use utils::ffi_support::{c_pointer_from_string, c_pointer_from_str};

    fn call_deserialize_inputs<'a>(
        did: Option<*const c_char>,
        outputs_json: Option<*const c_char>,
        cb: Option<JsonCallback>
    ) -> Result<DeserializedArguments<'a>, ErrorCode> {
        let req_json = did.unwrap_or_else(default::did);
        let outputs_json = outputs_json.unwrap_or_else(default::outputs_json_pointer);
        let cb = cb.unwrap_or(Some(default::empty_callback_string));

        return deserialize_inputs(req_json, outputs_json, cb);
    }

    #[test]
    fn build_mint_request_invalid_address() {
        let outputs = vec![
                Output::new(String::from("pad:sov:E9LNHk8shQ6xe2RfydzXDSsyhWC6vJaUeKE2mmc6mWraDfmKm"), 12, None)
            ];

        let did = Did::new(&"en32ansFeZNERIouv2xA");
        let result = build_mint_request(did, outputs);
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn build_mint_request_valid() {
        let output_config_value = json!([{
                "paymentAddress": "pay:sov:E9LNHk8shQ6xe2RfydzXDSsyhWC6vJaUeKE2mmc6mWraDfmKm",
                "amount": 12
            }]);

        let did_str = &"1123456789abcdef".as_bytes().to_base58();
        let (did, output_config, _) = call_deserialize_inputs(
            Some(c_pointer_from_str(did_str)),
            Some(c_pointer_from_string(output_config_value.to_string())),
            None
        ).unwrap();

        let result = build_mint_request(did.into(), output_config).unwrap();
        let mint_request_json = string_from_char_ptr(result).unwrap();
        let mint_value: serde_json::value::Value = serde_json::from_str(&mint_request_json).unwrap();

        let expected = json!({
            "identifier": did_str,
            "operation": {
                "type": MINT_PUBLIC,
                "outputs": [
                    ["E9LNHk8shQ6xe2RfydzXDSsyhWC6vJaUeKE2mmc6mWraDfmKm", 12]
                ]
            },
        });

        assert_eq!(expected.get("operation"), mint_value.get("operation"));
        assert_eq!(expected.get("identifier"), mint_value.get("identifier"));
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
    fn deserialize_outputs_invalid_structure() {
        // Invalid because there is no ver field.
        let outputs = c_pointer_from_string(json!({
            "outputs": [
                {
                    "address": "pay:sov:E9LNHk8shQ6xe2RfydzXDSsyhWC6vJaUeKE2mmc6mWraDfmKm",
                    "amount": 10
                }
            ]
        }).to_string());
        let result = call_deserialize_inputs(None, Some(outputs), None);
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn deserialize_valid_arguments() {
        let result = call_deserialize_inputs(None, None, None);
        assert!(result.is_ok());
    }

}