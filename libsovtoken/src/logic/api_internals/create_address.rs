/*!
Logic for the [`create_payment_address_handler`].

[`create_payment_address_handler`]: sovtoken::api::create_payment_address_handler
*/

use std::os::raw::c_char;

use ErrorCode;
use logic::config::payment_address_config::PaymentAddressConfig;
use utils::constants::general::{JsonCallback, JsonCallbackUnwrapped};
use utils::ffi_support::{string_from_char_ptr, cstring_from_str, c_pointer_from_str};
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

/**
Create a callback for address creation.
*/
pub fn create_address_cb(command_handle: i32, cb: JsonCallbackUnwrapped) -> impl Fn(String, ErrorCode) {
    move | payment_address: String, error_code: ErrorCode | {
        if error_code != ErrorCode::Success {
            error!("create payment address failed ErrorCode={:?}", error_code);
            cb(command_handle, error_code as i32, c_pointer_from_str(""));
            return;
        }

        debug!("create_payment_address_handler returning payment address of '{}'", &payment_address);
        let payment_address_cstring = cstring_from_str(payment_address);
        let payment_address_ptr = payment_address_cstring.as_ptr();

        cb(command_handle, ErrorCode::Success as i32, payment_address_ptr);   
    }
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

        deserialize_arguments(config_json, cb)
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
    fn test_valid_arguments()
    {
        let result = call_deserialize_arguments(None, None);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod create_address_cb_test {
    use super::*;
    use utils::test::callbacks;
    use std::sync::mpsc::RecvError;

    fn call_callback(address: String, error_code: ErrorCode)
        -> Result<(ErrorCode, String), RecvError>
    {
        let (receiver, command_handle, cb) = callbacks::cb_ec_string();
        create_address_cb(command_handle, cb.unwrap())(address, error_code);
        receiver.recv()
    }

    #[test]
    fn test_cb_called_on_error()
    {
        let result = call_callback(
            String::from("pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja"),
            ErrorCode::CommonInvalidState
        ).unwrap();

        assert_eq!(ErrorCode::CommonInvalidState, result.0);
        assert_eq!("", result.1);
    }

    #[test]
    fn test_cb_called_on_success()
    {
        let address = String::from("pay:sov:AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja");
        let result = call_callback(
            address.clone(),
            ErrorCode::Success
        ).unwrap();

        assert_eq!(ErrorCode::Success, result.0);
        assert_eq!(address, result.1);
    }
}