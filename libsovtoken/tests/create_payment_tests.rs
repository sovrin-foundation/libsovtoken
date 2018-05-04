//!
//! tests for Payment related functions


#![warn(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#[allow(unused_imports)]

extern crate libc;

extern crate sovtoken;
extern crate indy;                      // lib-sdk project

use libc::c_char;
use std::ptr;
use std::ffi::CString;
use std::thread;

use indy::api::ErrorCode;

// ***** HELPER TEST DATA  *****
const COMMAND_HANDLE: i32 = 10;
static INVALID_CONFIG_JSON: &'static str = r#"{ "horrible" : "only on tuedays"}"#;
static VALID_CONFIG_EMPTY_SEED_JSON: &'static str = r#"{"seed":""}"#;

// ***** HELPER METHODS  *****
extern "C" fn empty_create_payment_callback(command_handle_: i32, err: ErrorCode, payment_address: *const c_char) { }

// ***** UNIT TESTS *****

// the create payment requires a callback and this test ensures we have 
// receive error when no callback is provided
#[test]
fn errors_with_no_callback () {
    let return_error = sovtoken::api::create_payment_address_handler(COMMAND_HANDLE, ptr::null(), None);
    assert_eq!(return_error, ErrorCode::CommonInvalidParam3, "Expecting Callback for 'create_payment_address_handler'"); 
}


// the create payment method requires a config parameter and this test ensures that 
// a error is returned when no config is provided
#[test]
fn errors_with_no_config() {
    let cb : Option<extern fn(command_handle_: i32, err: ErrorCode, payment_address: *const c_char)> = Some(empty_create_payment_callback);
    let return_error = sovtoken::api::create_payment_address_handler(COMMAND_HANDLE, ptr::null(), cb);
    assert_eq!(return_error, ErrorCode::CommonInvalidParam2, "Expecting Config for 'create_payment_address_handler'");
}

// the create payment method requires a valid JSON format (format is described
// in create_payment_address_handler description).  Expecting error when invalid json is inputted
#[test]
fn errors_with_invalid_config_json() {

    let config_str = CString::new(INVALID_CONFIG_JSON).unwrap();
    let config_str_ptr = config_str.as_ptr();

    let cb : Option<extern fn(command_handle_: i32, err: ErrorCode, payment_address: *const c_char)> = Some(empty_create_payment_callback);
    let return_error = sovtoken::api::create_payment_address_handler(COMMAND_HANDLE, config_str_ptr, cb);

    assert_eq!(return_error, ErrorCode::CommonInvalidStructure, "Expecting Valid JSON for 'create_payment_address_handler'");
}

// this test passes valid parameters.  The callback is invoked and a valid payment address
// is returned in the call back.  The payment address format is described in
// create_payment_address_handler
#[test]
fn successfully_creates_payment_address_with_no_seed() {

    let handle = thread::spawn(||{

        let config_str = CString::new(VALID_CONFIG_EMPTY_SEED_JSON).unwrap();
        let config_str_ptr = config_str.as_ptr();

        let cb : Option<extern fn(command_handle_: i32, err: ErrorCode, payment_address: *const c_char)> = Some(empty_create_payment_callback);

        let return_error = sovtoken::api::create_payment_address_handler(COMMAND_HANDLE, config_str_ptr, cb);
    });


    handle.join().unwrap();

    unimplemented!();
}

// TODO:  the private address needs to be saved in the wallet.  if the wallet id
// is not valid, the private address cannot be saved.  this test passes an invalid
// wallet id and gets an error
#[test]
fn error_when_wallet_cannot_be_accessed() {
    unimplemented!();
}

// TODO: this test passes valid parameters.  the expectation is the callback is invoked
#[test]
fn success_callback_is_called() {
    unimplemented!();
}



