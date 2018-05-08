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
use log::*;
use rand::Rng;
use std::ptr;
use std::ffi::CString;
use std::thread;

use indy::api::ErrorCode;
use sovtoken::utils::ffi_support::str_from_char_ptr;
use sovtoken::logic::payment_address_config::PaymentAddressConfig;
use sovtoken::utils::json_conversion::*;
use sovtoken::utils::logger::*;

// ***** HELPER TEST DATA  *****
const COMMAND_HANDLE: i32 = 10;
static VALID_SEED_LEN: usize = 32;
static INVALID_CONFIG_JSON: &'static str = r#"{ "horrible" : "only on tuedays"}"#;
static VALID_CONFIG_EMPTY_SEED_JSON: &'static str = r#"{"seed":""}"#;
static mut CallBack_Payment_Address: Option<String> = None;
static mut CallBack_Payment_Address2: Option<String> = None;
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

extern "C" fn empty_create_payment_callback(command_handle_: i32, err: ErrorCode, payment_address: *const c_char) { }

extern "C" fn test_create_payment_callback(command_handle_: i32, err: ErrorCode, payment_address: *const c_char) {
    let payment : &str = str_from_char_ptr(payment_address).unwrap();
    unsafe { CallBack_Payment_Address = Some(payment.to_string()) };
}

extern "C" fn test_create_payment_callback2(command_handle_: i32, err: ErrorCode, payment_address: *const c_char) {
    let payment : &str = str_from_char_ptr(payment_address).unwrap();
    unsafe { CallBack_Payment_Address2 = Some(payment.to_string()) };
}

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

    log::set_logger(&MY_LOGGER).unwrap();
    log::set_max_level(LevelFilter::Trace);

    trace!("logging started for successfully_creates_payment_address_with_no_seed");

    unsafe { CallBack_Payment_Address2 = None };

    let handle = thread::Builder::new().name("thread_successfully_creates_payment_address_with_no_seed".to_string()).spawn(||{

        let config_str = CString::new(VALID_CONFIG_EMPTY_SEED_JSON).unwrap();
        let config_str_ptr = config_str.as_ptr();

        let cb : Option<extern fn(command_handle_: i32, err: ErrorCode, payment_address: *const c_char)> = Some(test_create_payment_callback2);

        let return_error = sovtoken::api::create_payment_address_handler(COMMAND_HANDLE, config_str_ptr, cb);
    }).unwrap();

    match handle.join() {
        Err(e) => println!("thread error {:?}", e),
        _ => (),
    };

    unsafe {
        match CallBack_Payment_Address2 {
            Some(ref mut s) => assert!(s.len() >= 32, "callback did not receive valid payment address"),
            None => assert!(false, "callback did not receive any information for a payment address"),
        };
    }

}


// this test passes valid parameters including a seed value for the key.  The callback is invoked and a valid payment address
// is returned in the call back.  The payment address format is described in
// create_payment_address_handler
#[test]
fn success_callback_is_called() {

    log::set_logger(&MY_LOGGER);
    log::set_max_level(LevelFilter::Trace);

    trace!("logging started for success_callback_is_called");


    unsafe { CallBack_Payment_Address = None };

    let handle = thread::Builder::new().name("thread_success_callback_is_called".to_string()).spawn(||{

        let seed = rand_string(VALID_SEED_LEN);
        let config: PaymentAddressConfig = PaymentAddressConfig { seed, };

        let config_str =  CString::new(config.to_json().unwrap()).unwrap();
        let config_str_ptr = config_str.as_ptr();

        let cb : Option<extern fn(command_handle_: i32, err: ErrorCode, payment_address: *const c_char)> = Some(test_create_payment_callback);

        let return_error = sovtoken::api::create_payment_address_handler(COMMAND_HANDLE, config_str_ptr, cb);
    }).unwrap();

    match handle.join() {
        Err(e) => trace!("thread error! {:?}", e),
        _ => (),
    };

    unsafe {
        match CallBack_Payment_Address {
            Some(ref mut s) => assert!(s.len() >= 32, "callback did not receive valid payment address"),
            None => assert!(false, "callback did not receive any information for a payment address"),
        };
    }

}


// TODO:  the private address needs to be saved in the wallet.  if the wallet id
// is not valid, the private address cannot be saved.  this test passes an invalid
// wallet id and gets an error
#[test]
fn error_when_wallet_cannot_be_accessed() {
    unimplemented!();
}





