//!
//! tests for Payment related functions

extern crate env_logger;
extern crate libc;
extern crate rand;

#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate bs58;
extern crate sovtoken;

use libc::c_char;
use rand::Rng;
use std::ptr;
use std::ffi::CString;
use std::time::Duration;

use sovtoken::utils::ErrorCode;
use sovtoken::logic::config::payment_address_config::PaymentAddressConfig;
use sovtoken::logic::address::unqualified_address_from_address;
use sovtoken::utils::test::callbacks;
mod utils;

// ***** HELPER TEST DATA  *****
const WALLET_ID: i32 = 99;
const COMMAND_HANDLE: i32 = 1;
const TIMEOUT_SECONDS: u64 = 20;
static VALID_SEED_LEN: usize = 32;
static INVALID_CONFIG_JSON: &'static str = r#"{ "horrible" : "only on tuedays"}"#;
static VALID_CONFIG_EMPTY_SEED_JSON: &'static str = r#"{}"#;


// ***** HELPER METHODS  *****
// helper methods
fn rand_string(length : usize) -> String {
    let s = rand::thread_rng()
            .gen_ascii_chars()
            .take(length)
            .collect::<String>();

    return s;
}

extern "C" fn empty_create_payment_callback(_command_handle: i32, _err: i32, _payment_address: *const c_char) -> i32 {
    return ErrorCode::Success as i32;
}


// ***** UNIT TESTS *****

// the create payment requires a callback and this test ensures we have
// receive error when no callback is provided
#[test]
fn errors_with_no_callback () {
    let return_error = sovtoken::api::create_payment_address_handler(COMMAND_HANDLE, WALLET_ID, ptr::null(), None);
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting Callback for 'create_payment_address_handler'");
}


// the create payment method requires a config parameter and this test ensures that
// a error is returned when no config is provided
#[test]
fn errors_with_no_config() {
    let cb : Option<extern fn(command_handle_: i32, err: i32, payment_address: *const c_char) -> i32> = Some(empty_create_payment_callback);
    let return_error = sovtoken::api::create_payment_address_handler(COMMAND_HANDLE, WALLET_ID, ptr::null(), cb);
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting Config for 'create_payment_address_handler'");
}


// the create payment method requires a valid JSON format (format is described
// in create_payment_address_handler description).  When invalid json is sent,
// default empty is used instead
#[test]
fn success_with_invalid_config_json() {

    let config_str = CString::new(INVALID_CONFIG_JSON).unwrap();
    let config_str_ptr = config_str.as_ptr();

    let cb : Option<extern fn(command_handle_: i32, err: i32, payment_address: *const c_char) -> i32> = Some(empty_create_payment_callback);
    let return_error = sovtoken::api::create_payment_address_handler(COMMAND_HANDLE, WALLET_ID, config_str_ptr, cb);

    assert_eq!(return_error, ErrorCode::Success as i32, "Expecting Valid JSON for 'create_payment_address_handler'");
}


// this test passes valid parameters.  The callback is invoked and a valid payment address
// is returned in the call back.  The payment address format is described in
// create_payment_address_handler
#[test]
fn successfully_creates_payment_address_with_no_seed() {

    debug!("logging started for successfully_creates_payment_address_with_no_seed");

    let (receiver, command_handle, cb) = callbacks::cb_ec_string();

    let config_str = CString::new(VALID_CONFIG_EMPTY_SEED_JSON).unwrap();
    let config_str_ptr = config_str.as_ptr();

    let wallet = utils::wallet::Wallet::new();

    let return_error = sovtoken::api::create_payment_address_handler(command_handle, wallet.handle, config_str_ptr, cb);

    assert_eq!(ErrorCode::Success, ErrorCode::from(return_error), "api call to create_payment_address_handler failed");

    let (err, payment_address) = receiver.recv_timeout(Duration::from_secs(TIMEOUT_SECONDS)).unwrap();

    debug!("******* got address of {}", payment_address);
    let unqual_address = unqualified_address_from_address(&payment_address).unwrap();
    assert_eq!(bs58::decode(unqual_address).into_vec().unwrap().len(), 36);
    assert_eq!(ErrorCode::Success, err, "Expected Success");
}


// this test passes valid parameters including a seed value for the key.  The callback is invoked and a valid payment address
// is returned in the call back.  The payment address format is described in
// create_payment_address_handler
#[test]
fn success_callback_is_called() {

    trace!("logging started for success_callback_is_called");

    let (receiver, command_handle, cb) = callbacks::cb_ec_string();

    let seed = rand_string(VALID_SEED_LEN);
    let config: PaymentAddressConfig = PaymentAddressConfig { seed, };

    let config_str =  config.serialize_to_cstring().unwrap();
    let config_str_ptr = config_str.as_ptr();

    let wallet = utils::wallet::Wallet::new();

    let return_error = sovtoken::api::create_payment_address_handler(command_handle, wallet.handle, config_str_ptr, cb);
    assert_eq!(ErrorCode::Success, ErrorCode::from(return_error), "api call to create_payment_address_handler failed");

    let (err, payment_address) = receiver.recv_timeout(Duration::from_secs(TIMEOUT_SECONDS)).unwrap();

    debug!("******* got address of {}", payment_address);
    let unqual_address = unqualified_address_from_address(&payment_address).unwrap();
    assert_eq!(bs58::decode(unqual_address).into_vec().unwrap().len(), 36, "callback did not receive valid payment address");
    assert_eq!(ErrorCode::Success, err, "Expected Success");

}

#[test]
pub fn create_address_two_times_with_the_same_seed() {
    sovtoken::api::sovtoken_init();
    let wallet = utils::wallet::Wallet::new();

    let seed = json!({"seed": "00000000000000000000000000000000"}).to_string();

    let _pa1 = indy::payments::Payment::create_payment_address(wallet.handle, "sov", &seed).unwrap();
    let err = indy::payments::Payment::create_payment_address(wallet.handle, "sov", &seed).unwrap_err();

    assert_eq!(err, indy::ErrorCode::WalletItemAlreadyExists);
}