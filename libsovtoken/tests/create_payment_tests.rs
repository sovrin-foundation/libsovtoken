//!
//! tests for Payment related functions


#![warn(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#[allow(unused_imports)]


extern crate libc;
extern crate rand;

#[macro_use] extern crate log;

extern crate sovtoken;
extern crate indy;                      // lib-sdk project

use libc::c_char;
use rand::Rng;
use std::ptr;
use std::ffi::CString;


use indy::api::ErrorCode;
use sovtoken::logic::payment_address_config::PaymentAddressConfig;
use sovtoken::utils::logger::*;
use sovtoken::utils::callbacks::*;

// ***** HELPER TEST DATA  *****
const WALLET_ID: i32 = 99;
const COMMAND_HANDLE: i32 = 1;
const TIMEOUT_SECONDS: u64 = 20;
static VALID_SEED_LEN: usize = 32;
static INVALID_CONFIG_JSON: &'static str = r#"{ "horrible" : "only on tuedays"}"#;
static VALID_CONFIG_EMPTY_SEED_JSON: &'static str = r#"{"seed":""}"#;
static TESTING_LOGGER: ConsoleLogger = ConsoleLogger;

// ***** HELPER METHODS  *****
// helper methods
fn rand_string(length : usize) -> String {
    let s = rand::thread_rng()
            .gen_ascii_chars()
            .take(length)
            .collect::<String>();

    return s;
}

extern "C" fn empty_create_payment_callback(command_handle_: i32, err: ErrorCode, payment_address: *const c_char) -> ErrorCode {
    return ErrorCode::Success;
}


// ***** UNIT TESTS *****

// the create payment requires a callback and this test ensures we have 
// receive error when no callback is provided
#[test]
fn errors_with_no_callback () {
    let return_error = sovtoken::api::create_payment_address_handler(COMMAND_HANDLE, WALLET_ID, ptr::null(), None);
    assert_eq!(return_error, ErrorCode::CommonInvalidParam3, "Expecting Callback for 'create_payment_address_handler'"); 
}


// the create payment method requires a config parameter and this test ensures that 
// a error is returned when no config is provided
#[test]
fn errors_with_no_config() {
    let cb : Option<extern fn(command_handle_: i32, err: ErrorCode, payment_address: *const c_char) -> ErrorCode> = Some(empty_create_payment_callback);
    let return_error = sovtoken::api::create_payment_address_handler(COMMAND_HANDLE, WALLET_ID, ptr::null(), cb);
    assert_eq!(return_error, ErrorCode::CommonInvalidParam2, "Expecting Config for 'create_payment_address_handler'");
}

// the create payment method requires a valid JSON format (format is described
// in create_payment_address_handler description).  Expecting error when invalid json is inputted
#[test]
fn errors_with_invalid_config_json() {

    let config_str = CString::new(INVALID_CONFIG_JSON).unwrap();
    let config_str_ptr = config_str.as_ptr();

    let cb : Option<extern fn(command_handle_: i32, err: ErrorCode, payment_address: *const c_char) -> ErrorCode> = Some(empty_create_payment_callback);
    let return_error = sovtoken::api::create_payment_address_handler(COMMAND_HANDLE, WALLET_ID, config_str_ptr, cb);

    assert_eq!(return_error, ErrorCode::CommonInvalidStructure, "Expecting Valid JSON for 'create_payment_address_handler'");
}


// this test passes valid parameters.  The callback is invoked and a valid payment address
// is returned in the call back.  The payment address format is described in
// create_payment_address_handler
#[test]
fn successfully_creates_payment_address_with_no_seed() {

    trace!("logging started for successfully_creates_payment_address_with_no_seed");

    let (receiver, command_handle, cb) = CallbackWithErrorCodeReturnUtils::closure_to_cb_ec_string_with_return();

    let config_str = CString::new(VALID_CONFIG_EMPTY_SEED_JSON).unwrap();
    let config_str_ptr = config_str.as_ptr();

    let return_error = sovtoken::api::create_payment_address_handler(command_handle, WALLET_ID, config_str_ptr, cb);
    assert_eq!(ErrorCode::Success, return_error, "api call to create_payment_address_handler failed");

    let (err, payment_address) = receiver.recv_timeout(TimeoutUtils::specific_timeout(TIMEOUT_SECONDS)).unwrap();

    assert!(payment_address.len() >= 32, "callback did not receive valid payment address");
}


// this test passes valid parameters including a seed value for the key.  The callback is invoked and a valid payment address
// is returned in the call back.  The payment address format is described in
// create_payment_address_handler
#[test]
fn success_callback_is_called() {

    trace!("logging started for success_callback_is_called");

    let (receiver, command_handle, cb) = CallbackWithErrorCodeReturnUtils::closure_to_cb_ec_string_with_return();

    let seed = rand_string(VALID_SEED_LEN);
    let config: PaymentAddressConfig = PaymentAddressConfig { seed, };

    let config_str =  config.serialize_to_cstring().unwrap();
    let config_str_ptr = config_str.as_ptr();

    let return_error = sovtoken::api::create_payment_address_handler(command_handle, WALLET_ID, config_str_ptr, cb);
    // let return_error = sovtoken::api::create_payment_address_handler(command_handle, WALLET_ID, config_str_ptr, None);
    assert_eq!(ErrorCode::Success, return_error, "api call to create_payment_address_handler failed");

    let (err, payment_address) = receiver.recv_timeout(TimeoutUtils::specific_timeout(TIMEOUT_SECONDS)).unwrap();

    assert!(payment_address.len() >= 32, "callback did not receive valid payment address");
}






