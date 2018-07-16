/*!
Logic for the [`create_payment_address_handler`].

[`create_payment_address_handler`]: sovtoken::api::create_payment_address_handler
*/

use indy::ErrorCode;
use std::os::raw::c_char;
use logic::config::payment_address_config::PaymentAddressConfig;
use utils::constants::general::{JsonCallback, JsonCallbackUnwrapped};
use utils::ffi_support::string_from_char_ptr;
use utils::json_conversion::JsonDeserialize;

type DeserializedArguments = (PaymentAddressConfig, JsonCallbackUnwrapped);

/**
Deserialize the [`create_payment_address_handler`] arguments.

[`create_payment_address_handler`]: sovtoken::api::create_payment_address_handler
*/
pub fn deserialize_arguments(
    json_config: *const c_char,
    cb: JsonCallback,
) -> Result<DeserializedArguments, ErrorCode> {
    let cb = cb.ok_or(ErrorCode::CommonInvalidStructure)?;

    let json_config_string : String = string_from_char_ptr(json_config)
        .ok_or(ErrorCode::CommonInvalidStructure)
        .map_err(map_err_err!())?;

    debug!("api::create_payment_address_handler json_config_string >> {:?}", json_config_string);

    // TODO: Only continue when seed is missing, not on any error.
    let config = PaymentAddressConfig::from_json(&json_config_string)
        .map_err(map_err_trace!())
        .unwrap_or(PaymentAddressConfig { seed: "".to_string() });

    debug!("api::create_payment_address_handler PaymentAddressConfig >> {:?}", config);

    Ok((config, cb))
}

#[cfg(test)]
mod deserialize_arguments_test {
    use super::*;
    use std::ptr;
    use utils::test::default;

    pub fn call_deserialize_arguments(
        config_json: Option<*const c_char>,
        cb: Option<JsonCallback>
    ) -> Result<DeserializedArguments, ErrorCode> {
        let config_json = config_json.unwrap_or_else(default::create_address_config);
        let cb = cb.unwrap_or(Some(default::empty_callback_string));

        deserialize_arguments(config_json, cb);
    }

    #[test]
    fn test_null_config()
    {
        let result = call_deserialize_arguments(Some(ptr::null()), None);
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn test_empty_callback()
    {
        let result = call_deserialize_arguments(None, Some(None));
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    #[ignore]
    // TODO: Validate the seed is 32 characters long.
    fn test_config_with_bad_seed()
    {
        // The seed needs to be 32 characters long.
        let config_pointer = json_c_pointer!({
            "seed": "declivity"
        });

        let result = call_deserialize_arguments(Some(config_pointer), None);
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err());
    }

    #[test]
    fn test_config_without_seed()
    {
        let config_pointer = json_c_pointer!({});
        let result = call_deserialize_arguments(Some(config_pointer), None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_with_seed()
    {
        let config_pointer = json_c_pointer!({
            "seed": "qeWFjZkt9Cr4mhh1VQMrQrKF8a1CeXqN"
        });
        let result = call_deserialize_arguments(Some(config_pointer), None);
        assert!(result.is_ok());
    }


    #[test]
    fn valid_arguments()
    {
        let result = call_deserialize_arguments(None, None);
        assert!(result.is_ok());
    }
}