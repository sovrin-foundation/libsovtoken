//! Constants/Statics that cannot be grouped otherwise

use std::os::raw::c_char;

pub static PAYMENT_METHOD_NAME: &str = "sov";
pub static PAYMENT_ADDRESS_QUALIFIER: &'static str = "pay:sov:";
pub static TXO_QUALIFIER: &str = "txo:sov:";

pub const PROTOCOL_VERSION: u32 = 2;
pub const LEDGER_ID: &str = "1001";

/**
Defines a callback to communicate results to Indy-sdk as type

# Params
- command_handle __same value as the API inputted command handle__
- err __error code__
- json_pointer __JSON results. Format is defined by the API.__
*/
pub type JsonCallback = Option<JsonCallbackUnwrapped>;
pub type JsonCallbackUnwrapped = extern fn(command_handle: i32, err: i32, json_pointer: *const c_char) -> i32;

pub type JsonI64Callback = Option<JsonI64CallbackUnwrapped>;
pub type JsonI64CallbackUnwrapped = extern fn(command_handle: i32, err: i32, json_pointer: *const c_char, num: i64) -> i32;
