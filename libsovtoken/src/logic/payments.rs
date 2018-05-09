//! Payments module contains functions for working with payments.  :D
#![allow(unused_variables)]
#![allow(unused_imports)]
#[warn(unused_imports)]

use log::*;
use serde::{Serialize, Deserialize};
use std::{str};
use std::ffi::{CString};

use indy::api::ErrorCode;
use indy::api::crypto::indy_create_key;
use super::payment_address_config::PaymentAddressConfig;
use utils::ffi_support::{string_from_char_ptr, cstring_from_str};
use utils::general::some_or_none_option_u8;
use utils::json_conversion::JsonSerialize;
use utils::callbacks::*;


// ------------------------------------------------------------------
// statics that make up parts of the payment address
// ------------------------------------------------------------------
/// = "pay"
pub static PAY_INDICATOR: &'static str = "pay";
/// = "sov"
pub static SOVRIN_INDICATOR: &'static str = "sov";
/// = ":"
pub static PAYMENT_ADDRESS_FIELD_SEP: &'static str = ":";


/**
    This defines the interfaces which can be replaced with different implementations
    (aka production vs test time)
*/
pub trait CreatePaymentAPI {
    fn new() -> Self;
    fn create_payment_address(&self, command_handle: i32, wallet_id: i32, config: PaymentAddressConfig) -> String;
}

/**
   Implementation of CreatePaymentAPI for use in productions environment
*/
pub struct CreatePaymentHandler{}

impl CreatePaymentAPI for CreatePaymentHandler {

    fn new() -> Self {
        return CreatePaymentHandler{};
    }

    /**
       creates fully formatted address based on inputted seed.  If seed is empty
       then a randomly generated seed is used by libsodium
       the format of the return is:
           pay:sov:{32 byte address}{4 byte checksum}
    */
    fn create_payment_address(&self, command_handle: i32, wallet_id: i32, config: PaymentAddressConfig) -> String {
        let (receiver, sdk_command_handle, cb) = CallbackUtils::closure_to_cb_ec_string();

        let config_cstring: CString = config.serialize_to_cstring().unwrap();
        let config_str_ptr = config_cstring.as_ptr();

        trace!("create_payment_address calling indy_create_key");
        let result: ErrorCode = indy_create_key(command_handle, wallet_id, config_str_ptr, cb);

        // TODO on error this long_timeout also causes a long delay!
        let (err, payment_address) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        return create_formatted_address_with_checksum(payment_address);
    }
}

/** computes a checksum based on an address */
fn compute_address_checksum(address: String) -> String {
    return "1234".to_string();
}

/** creates the fully formatted payment address string */
fn create_formatted_address_with_checksum(address: String) -> String {
    let mut result: String = PAY_INDICATOR.to_owned();

    result.push_str(PAYMENT_ADDRESS_FIELD_SEP);
    result.push_str(SOVRIN_INDICATOR);
    result.push_str(PAYMENT_ADDRESS_FIELD_SEP);
    result.push_str(&address);
    result.push_str(&compute_address_checksum(address));

    return result;
}

pub fn execute_create_payment<T>(injected : T, command_handle: i32, wallet_id: i32, config: PaymentAddressConfig) -> String where T : CreatePaymentAPI {
    return injected.create_payment_address(command_handle, wallet_id, config);
}


// ------------------------------------------------------------------
// unit tests
// ------------------------------------------------------------------

#[cfg(test)]
mod payments_tests {
    extern crate rand;
    extern crate log;
    
    use self::rand::Rng;
    use log::*;
    use std::panic;
    use utils::general::StringUtils;
    use utils::logger::ConsoleLogger;

    use super::*;

    pub struct TestCreatePaymentHandler{}

    impl CreatePaymentAPI for TestCreatePaymentHandler {

        fn new() -> Self {
            return TestCreatePaymentHandler{};
        }

        fn create_payment_address(&self, command_handle: i32, wallet_id: i32, config: PaymentAddressConfig) -> String {
            return create_formatted_address_with_checksum(rand_string(32));
        }
    }


    static VALID_ADDRESS_LEN: usize = 32;
    static VALID_SEED_LEN: usize = 32;
    static INVALID_SEED_LEN: usize = 19;
    static CHECKSUM_LEN: usize = 4;
    static WALLET_ID: i32 = 10;
    static COMMAND_HANDLE: i32 = 10;

    // helper methods
    fn rand_string(length : usize) -> String {
        let s = rand::thread_rng()
                .gen_ascii_chars()
                .take(length)
                .collect::<String>();

        return s;
    }

    // returns the last 4 chars if input
    fn get_address_checksum(address: &str) -> String {
        return address.from_right(CHECKSUM_LEN);
    }

    #[test]
    fn success_validate_create_formatted_address_with_checksum() {
        let address = create_formatted_address_with_checksum(rand_string(32));

        // got our result, if its correct, it will look something like this:
        // pay:sov:gzidfrdJtvgUh4jZTtGvTZGU5ebuGMoNCbofXGazFa91234
        // break it up into the individual parts we expect to find and
        // test the validity of the parts
        let pay_indicator = &address[0..3];
        let first_separator = &address[3..4];
        let sov_indicator = &address[4..7];
        let second_indicator = &address[7..8];
        let result_address = &address[8..40];

        let checksum: String = get_address_checksum( &address[..]);

        assert_eq!(PAY_INDICATOR, pay_indicator, "PAY_INDICATOR not found");
        assert_eq!(PAYMENT_ADDRESS_FIELD_SEP, first_separator, "first PAYMENT_ADDRESS_FIELD_SEP not found");
        assert_eq!(SOVRIN_INDICATOR, sov_indicator, "SOVRIN_INDICATOR not found");
        assert_eq!(PAYMENT_ADDRESS_FIELD_SEP, second_indicator, "second PAYMENT_ADDRESS_FIELD_SEP not found");
        assert_eq!(VALID_ADDRESS_LEN, result_address.chars().count(), "address is not 32 bytes");
        assert_eq!(CHECKSUM_LEN, checksum.len(), "checksum is not 4 bytes");
    }

    // This is the happy path test.  Config contains a seed and
    // a fully formatted address is returned.
    #[test]
    fn success_create_payment_with_seed_returns_address() {

        let seed = rand_string(VALID_SEED_LEN);
        let config: PaymentAddressConfig = PaymentAddressConfig { seed };

        let address = execute_create_payment(TestCreatePaymentHandler{},COMMAND_HANDLE, WALLET_ID, config);

        // got our result, if its correct, it will look something like this:
        // pay:sov:gzidfrdJtvgUh4jZTtGvTZGU5ebuGMoNCbofXGazFa91234
        // break it up into the individual parts we expect to find and
        // test the validity of the parts
        let pay_indicator = &address[0..3];
        let first_separator = &address[3..4];
        let sov_indicator = &address[4..7];
        let second_indicator = &address[7..8];
        let result_address = &address[8..40];

        let checksum: String = get_address_checksum( &address[..]);

        assert_eq!(PAY_INDICATOR, pay_indicator, "PAY_INDICATOR not found");
        assert_eq!(PAYMENT_ADDRESS_FIELD_SEP, first_separator, "first PAYMENT_ADDRESS_FIELD_SEP not found");
        assert_eq!(SOVRIN_INDICATOR, sov_indicator, "SOVRIN_INDICATOR not found");
        assert_eq!(PAYMENT_ADDRESS_FIELD_SEP, second_indicator, "second PAYMENT_ADDRESS_FIELD_SEP not found");
        assert_eq!(VALID_ADDRESS_LEN, result_address.chars().count(), "address is not 32 bytes");
        assert_eq!(CHECKSUM_LEN, checksum.len(), "checksum is not 4 bytes");

    }

    // This is the happy path test when seed provided is empty.  Expectation is a
    // a fully formatted address is returned.
    #[test]
    fn success_create_payment_with_no_seed_returns_address() {

        let seed = String::new();
        let config: PaymentAddressConfig = PaymentAddressConfig { seed };

        let address = execute_create_payment(TestCreatePaymentHandler{}, COMMAND_HANDLE, WALLET_ID, config);

        // got our result, if its correct, it will look something like this:
        // pay:sov:gzidfrdJtvgUh4jZTtGvTZGU5ebuGMoNCbofXGazFa91234
        // break it up into the individual parts we expect to find and
        // test the validity of the parts
        let pay_indicator = &address[0..3];
        let first_separator = &address[3..4];
        let sov_indicator = &address[4..7];
        let second_indicator = &address[7..8];
        let result_address = &address[8..40];

        let checksum: String = get_address_checksum( &address[..]);

        assert_eq!(PAY_INDICATOR, pay_indicator, "PAY_INDICATOR not found");
        assert_eq!(PAYMENT_ADDRESS_FIELD_SEP, first_separator, "first PAYMENT_ADDRESS_FIELD_SEP not found");
        assert_eq!(SOVRIN_INDICATOR, sov_indicator, "SOVRIN_INDICATOR not found");
        assert_eq!(PAYMENT_ADDRESS_FIELD_SEP, second_indicator, "second PAYMENT_ADDRESS_FIELD_SEP not found");
        assert_eq!(VALID_ADDRESS_LEN, result_address.chars().count(), "address is not 32 bytes");
        assert_eq!(CHECKSUM_LEN, checksum.len(), "checksum is not 4 bytes");
    }

    // This test uses an invalid seed length to generate an address. Expectation is an error (panic)
    // is raised.
    #[test]
    fn error_create_payment_with_invalid_seed_len() {

        let result = panic::catch_unwind(|| {

            let seed = rand_string(INVALID_SEED_LEN);
            let config: PaymentAddressConfig = PaymentAddressConfig { seed };

            let address = execute_create_payment(TestCreatePaymentHandler{}, COMMAND_HANDLE, WALLET_ID, config);
        });

        assert!(result.is_err(), "create_payment_address did not throw error on invalid seed length");
    }
}