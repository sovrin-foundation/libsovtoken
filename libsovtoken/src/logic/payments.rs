//! Payments module contains functions for working with payments.  :D
#![allow(unused_variables)]
#![allow(unused_imports)]
#[warn(unused_imports)]

use std::str;
use log;

use super::payment_address_config::PaymentAddressConfig;
use libraries::sodium::{CryptoEngine};
use libraries::rust_base58::Base58;
use utils::general::some_or_none_option_u8;

// statics that make up parts of the payment address
pub static PAY_INDICATOR: &'static str = "pay";
pub static SOVRIN_INDICATOR: &'static str = "sov";
pub static PAYMENT_ADDRESS_FIELD_SEP: &'static str = ":";


// computes a check some based on an address
fn compute_address_checksum(address: String) -> String {
    return "1234".to_string();
}

// creates the fully formatted payment address string
fn create_formatted_address_with_checksum(address: String) -> String {

    let mut result: String = PAY_INDICATOR.to_owned();

    result.push_str(PAYMENT_ADDRESS_FIELD_SEP);
    result.push_str(SOVRIN_INDICATOR);
    result.push_str(PAYMENT_ADDRESS_FIELD_SEP);
    result.push_str(&address);
    result.push_str(&compute_address_checksum(address));

    return result;
}

// creates fully formatted address based on inputted seed.  If seed is empty
// then a randomly generated seed is used by libsodium
// the format of the return is:
//     pay:sov:{32 byte address}{4 byte checksum}
pub fn create_payment_address(config: PaymentAddressConfig) -> String {

    // TODO: how should we handle errors other than panic?
    let usable_seed = some_or_none_option_u8(config.seed.as_bytes());
    let (pub_address, _) = match CryptoEngine::create_key_pair_for_signature(usable_seed)
    {
        Ok(r) => r,
        Err(e) =>  {
            error!("error creating keys with seed '{}'", config.seed);
            panic!("crypto error: {:?}", e);
        },
    };

    let pub_address_str = Base58::encode(&pub_address);

    return create_formatted_address_with_checksum(pub_address_str.to_string());
}


#[cfg(test)]
mod payments_tests {
    extern crate rand;
    use self::rand::Rng;
    use std::panic;
    use utils::general::StringUtils;

    use super::*;


    static VALID_ADDRESS_LEN: usize = 32;
    static VALID_SEED_LEN: usize = 32;
    static INVALID_SEED_LEN: usize = 19;
    static CHECKSUM_LEN: usize = 4;

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

    // This is the happy path test.  Config contains a seed and
    // a fully formatted address is returned.
    #[test]
    fn success_create_payment_with_seed_returns_address() {
        let seed = rand_string(VALID_SEED_LEN);
        let config: PaymentAddressConfig = PaymentAddressConfig { seed };

        let address = create_payment_address(config);

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

        let address = create_payment_address(config);

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

            let address = create_payment_address(config);
        });

        assert!(result.is_err(), "create_payment_address did not throw error on invalid seed length");
    }
}