//!
//! tests for Payment related functions


extern crate bs58;
extern crate libc;

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate indyrs as indy;                      // lib-sdk project
extern crate sovtoken;

use std::ptr;
use std::ffi::CString;
use std::time::Duration;

use sovtoken::logic::config::payment_address_config::PaymentAddressConfig;
use sovtoken::logic::address::unqualified_address_from_address;
use sovtoken::utils::test::callbacks;
use sovtoken::ErrorCode;

mod utils;

// ***** HELPER TEST DATA  *****
const WALLET_ID: i32 = 99;
const COMMAND_HANDLE: i32 = 1;
const TIMEOUT_SECONDS: u64 = 20;
static INVALID_CONFIG_JSON: &'static str = r#"{ "horrible" : "only on tuedays"}"#;


// ***** HELPER METHODS  *****

fn create_payment_address(wallet: &utils::wallet::Wallet, config: PaymentAddressConfig) -> String {
    let (receiver, command_handle, cb) = callbacks::cb_ec_string();

    let config_str = config.serialize_to_cstring().unwrap();
    let config_str_ptr = config_str.as_ptr();

    let return_error = sovtoken::api::create_payment_address_handler(command_handle, wallet.handle, config_str_ptr, cb);

    assert_eq!(ErrorCode::Success, ErrorCode::from(return_error), "api call to create_payment_address_handler failed");

    let (err, payment_address) = receiver.recv_timeout(Duration::from_secs(TIMEOUT_SECONDS)).unwrap();

    assert_eq!(ErrorCode::Success, err, "Expected Success");

    return payment_address;
}


// ***** UNIT TESTS *****

// the create payment requires a callback and this test ensures we have
// receive error when no callback is provided
#[test]
fn errors_with_no_callback() {
    let return_error = sovtoken::api::create_payment_address_handler(COMMAND_HANDLE, WALLET_ID, ptr::null(), None);
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting Callback for 'create_payment_address_handler'");
}


// the create payment method requires a config parameter and this test ensures that
// a error is returned when no config is provided
#[test]
fn errors_with_no_config() {
    let return_error = sovtoken::api::create_payment_address_handler(COMMAND_HANDLE, WALLET_ID, ptr::null(), Some(utils::callbacks::empty_callback));
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting Config for 'create_payment_address_handler'");
}


// the create payment method requires a valid JSON format (format is described
// in create_payment_address_handler description).  When invalid json is sent,
// default empty is used instead
#[test]
fn success_with_invalid_config_json() {
    let config_str = CString::new(INVALID_CONFIG_JSON).unwrap();
    let config_str_ptr = config_str.as_ptr();

    let return_error = sovtoken::api::create_payment_address_handler(COMMAND_HANDLE, WALLET_ID, config_str_ptr, Some(utils::callbacks::empty_callback));

    assert_eq!(return_error, ErrorCode::Success as i32, "Expecting Valid JSON for 'create_payment_address_handler'");
}

// this test passes valid parameters.  The callback is invoked and a valid payment address
// is returned in the call back.  The payment address format is described in
// create_payment_address_handler
#[test]
fn successfully_creates_payment_address_with_no_seed() {
    debug!("logging started for successfully_creates_payment_address_with_no_seed");

    let wallet = utils::wallet::Wallet::new();

    let config: PaymentAddressConfig = PaymentAddressConfig { seed: String::new() };

    let payment_address = create_payment_address(&wallet, config);

    debug!("******* got address of {}", payment_address);
    let unqual_address = unqualified_address_from_address(&payment_address).unwrap();
    assert_eq!(bs58::decode(unqual_address).into_vec().unwrap().len(), 36);
}

// this test passes a valid seed value for the key. The callback is invoked and an expected valid
// payment address is returned in the call back. The payment address format is described in
// create_payment_address_handler
#[test]
fn successfully_creates_payment_address_with_seed() {
    trace!("logging started for successfully_creates_payment_address_with_seed");

    let config: PaymentAddressConfig = PaymentAddressConfig { seed: "00000000000000000000000000000000".to_string() };

    let wallet = utils::wallet::Wallet::new();

    let payment_address = create_payment_address(&wallet, config);

    let expected_payment_address = "pay:sov:DB3eBYTCr9NvNVZNp1GwV12iDfqftoGrDKqBedRZV4SgdeTbi";

    debug!("******* got address of {}", payment_address);

    assert_eq!(expected_payment_address, payment_address, "callback did not receive expected payment address");
}