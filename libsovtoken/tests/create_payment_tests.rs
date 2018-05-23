//!
//! tests for Payment related functions


#![warn(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#[allow(unused_imports)]


extern crate libc;
extern crate rand;

#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;

extern crate sovtoken;
extern crate rust_indy_sdk as indy;                      // lib-sdk project

use libc::c_char;
use rand::Rng;
use std::ffi::CStr;
use std::collections::HashMap;
use std::ptr;
use std::ffi::CString;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::sync::Mutex;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

use indy::ErrorCode;
use indy::wallet::Wallet;
use sovtoken::logic::config::payment_address_config::PaymentAddressConfig;
use sovtoken::utils::logger::*;

// ***** HELPER TEST DATA  *****
const WALLET_ID: i32 = 99;
const COMMAND_HANDLE: i32 = 1;
const TIMEOUT_SECONDS: u64 = 20;
static VALID_SEED_LEN: usize = 32;
static WALLET_NAME: &'static str = "integration_test_wallet";
static INVALID_CONFIG_JSON: &'static str = r#"{ "horrible" : "only on tuedays"}"#;
static VALID_CONFIG_EMPTY_SEED_JSON: &'static str = r#"{"seed":""}"#;
static VALID_CONFIG_EMPTY_SEED_JSON2: &'static str = r#"{}"#;
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

lazy_static! {
    static ref COMMAND_HANDLE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
    static ref CALLBACKS_EC_STRING: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
}

pub fn closure_to_cb_ec_string() -> (Receiver<(ErrorCode, String)>, i32,
                                      Option<extern fn(command_handle: i32,
                                                       err: ErrorCode,
                                                       c_str: *const c_char) -> ErrorCode>) {
    let (sender, receiver) = channel();

    let closure = Box::new(move |err: ErrorCode, val: String| {
        sender.send((err, val)).unwrap();
    });

    extern "C" fn _callback(command_handle: i32, err: ErrorCode, c_str: *const c_char) -> ErrorCode {
        let mut callbacks = CALLBACKS_EC_STRING.lock().unwrap();
        let mut cb = callbacks.remove(&command_handle).unwrap();
        let metadata = unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() };
        cb(err, metadata);

        return err;
    }

    let mut callbacks = CALLBACKS_EC_STRING.lock().unwrap();
    let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
    callbacks.insert(command_handle, closure);

    (receiver, command_handle, Some(_callback))
}

//
fn safely_create_wallet() -> i32 {
    let panic_result = std::panic::catch_unwind( ||
         {
             Wallet::delete_wallet(WALLET_NAME);
         });

    Wallet::create_wallet("pool_1", WALLET_NAME, None, Some(VALID_CONFIG_EMPTY_SEED_JSON2), None);
    let wallet_id: i32 = Wallet::open_wallet(WALLET_NAME, None, None).unwrap();

    return wallet_id;
}

// ***** UNIT TESTS *****

// the create payment requires a callback and this test ensures we have 
// receive error when no callback is provided
#[test]
fn errors_with_no_callback () {
    let return_error = sovtoken::api::create_payment_address_handler(COMMAND_HANDLE, WALLET_ID, ptr::null(), None);
    assert_eq!(return_error, ErrorCode::CommonInvalidParam4, "Expecting Callback for 'create_payment_address_handler'"); 
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

    let (receiver, command_handle, cb) = closure_to_cb_ec_string();

    let config_str = CString::new(VALID_CONFIG_EMPTY_SEED_JSON).unwrap();
    let config_str_ptr = config_str.as_ptr();

    let wallet_id: i32 = safely_create_wallet();

    let return_error = sovtoken::api::create_payment_address_handler(command_handle, wallet_id, config_str_ptr, cb);

    assert_eq!(ErrorCode::Success, return_error, "api call to create_payment_address_handler failed");

    let (err, payment_address) = receiver.recv_timeout(Duration::from_secs(TIMEOUT_SECONDS)).unwrap();

    debug!("******* got address of {}", payment_address);
    assert!(payment_address.len() >= 32, "callback did not receive valid payment address");
    assert_eq!(ErrorCode::Success, err, "Expected Success");
}


// this test passes valid parameters including a seed value for the key.  The callback is invoked and a valid payment address
// is returned in the call back.  The payment address format is described in
// create_payment_address_handler
#[test]
fn success_callback_is_called() {

    trace!("logging started for success_callback_is_called");

    let (receiver, command_handle, cb) = closure_to_cb_ec_string();

    let seed = rand_string(VALID_SEED_LEN);
    let config: PaymentAddressConfig = PaymentAddressConfig { seed, };

    let config_str =  config.serialize_to_cstring().unwrap();
    let config_str_ptr = config_str.as_ptr();

    let wallet_id: i32 = safely_create_wallet();

    let return_error = sovtoken::api::create_payment_address_handler(command_handle, wallet_id, config_str_ptr, cb);
    assert_eq!(ErrorCode::Success, return_error, "api call to create_payment_address_handler failed");

    let (err, payment_address) = receiver.recv_timeout(Duration::from_secs(TIMEOUT_SECONDS)).unwrap();

    println!("******* got address of {}", payment_address);
    assert!(payment_address.len() >= 32, "callback did not receive valid payment address");
    assert_eq!(ErrorCode::Success, err, "Expected Success");

}






