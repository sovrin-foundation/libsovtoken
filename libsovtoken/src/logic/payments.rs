//! Payments module contains functions for working with payments.  :D
#![allow(unused_variables)]
#[warn(unused_imports)]

use indy::ErrorCode;
use logic::config::payment_address_config::PaymentAddressConfig;
use logic::indy_sdk_api::crypto_api::{CryptoAPI};
use logic::address;


// ------------------------------------------------------------------
// CreatePaymentHandler
// ------------------------------------------------------------------
/**
    CreatePaymentHandler contains methods for creating a fully formatted address based on inputted
    seed.  If seed is empty then a randomly generated seed is used by libsodium

    In production runtime environment, the expectation is T is CryptoSdk
    and in testing environments its anything else as long as it implements CryptoAPI

*/
pub struct CreatePaymentHandler<T> where T : CryptoAPI {
    injected_api : T
}

impl<T: CryptoAPI> CreatePaymentHandler<T> {
    pub fn new(api_handler : T) -> Self {
        CreatePaymentHandler { injected_api : api_handler }
    }

    /**
       the format of the return is:
           pay:sov:{32 byte address}{4 byte checksum}
    */
    pub fn create_payment_address(&self, wallet_id: i32, config: PaymentAddressConfig) -> Result<String, ErrorCode> {
        let address = self.injected_api.indy_create_key(wallet_id, config)?;
        return Ok(address::create_formatted_address_with_checksum(address));
    }
}


// ------------------------------------------------------------------
// unit tests
// ------------------------------------------------------------------

#[cfg(test)]
mod payments_tests {
    extern crate log;

    use utils::random::rand_string;
    use logic::address::*;

    use super::*;

    // mock SDK api calls with a call that will generate a random 32 byte string
    struct CreatePaymentSDKMockHandler {}
    impl CryptoAPI for CreatePaymentSDKMockHandler {
        fn indy_create_key(&self, wallet_id: i32, config: PaymentAddressConfig) -> Result<String, ErrorCode> {
            return Ok(rand_string(VERKEY_LEN));
        }

        fn indy_crypto_sign<F>(&self, _: i32, _: String, _: String, _: F) -> ErrorCode {
            return ErrorCode::CommonInvalidState;
        } 
    }


    static VALID_SEED_LEN: usize = 32;
    static WALLET_ID: i32 = 10;
    
    // This is the happy path test.  Config contains a seed and
    // a fully formatted address is returned.
    #[test]
    fn success_create_payment_with_seed_returns_address() {

        let seed = rand_string(VALID_SEED_LEN);
        let config: PaymentAddressConfig = PaymentAddressConfig { seed };
        let handler = CreatePaymentHandler::new(CreatePaymentSDKMockHandler{});

        let address = match handler.create_payment_address(WALLET_ID, config) {
            Ok(s) => s,
            Err(e) => "".to_string(),
        };

        // got our result, if its correct, it will look something like this:
        // pay:sov:gzidfrdJtvgUh4jZTtGvTZGU5ebuGMoNCbofXGazFa91234
        // break it up into the individual parts we expect to find and
        // test the validity of the parts
        let pay_indicator = &address[0..3];
        let first_separator = &address[3..4];
        let sov_indicator = &address[4..7];
        let second_indicator = &address[7..8];
        let result_address = &address[8..52];

        let checksum: String = address::get_checksum(&address).unwrap();

        assert_eq!(PAY_INDICATOR, pay_indicator, "PAY_INDICATOR not found");
        assert_eq!(PAYMENT_ADDRESS_FIELD_SEP, first_separator, "first PAYMENT_ADDRESS_FIELD_SEP not found");
        assert_eq!(SOVRIN_INDICATOR, sov_indicator, "SOVRIN_INDICATOR not found");
        assert_eq!(PAYMENT_ADDRESS_FIELD_SEP, second_indicator, "second PAYMENT_ADDRESS_FIELD_SEP not found");
        assert_eq!(VERKEY_LEN, result_address.chars().count(), "address is not 44 bytes");
        assert_eq!(CHECKSUM_LEN, checksum.len(), "checksum is not 4 bytes");

    }

    // This is the happy path test when seed provided is empty.  Expectation is a
    // a fully formatted address is returned.
    #[test]
    fn success_create_payment_with_no_seed_returns_address() {

        let seed = String::new();
        let config: PaymentAddressConfig = PaymentAddressConfig { seed };

        let handler = CreatePaymentHandler::new(CreatePaymentSDKMockHandler{});
        let address = match handler.create_payment_address(WALLET_ID, config){
            Ok(s) => s,
            Err(e) => "".to_string(),
        };

        // got our result, if its correct, it will look something like this:
        // pay:sov:gzidfrdJtvgUh4jZTtGvTZGU5ebuGMoNCbofXGazFa91234
        // break it up into the individual parts we expect to find and
        // test the validity of the parts
        let pay_indicator = &address[0..3];
        let first_separator = &address[3..4];
        let sov_indicator = &address[4..7];
        let second_indicator = &address[7..8];
        let result_address = &address[8..52];

        let checksum: String = address::get_checksum(&address).unwrap();

        assert_eq!(PAY_INDICATOR, pay_indicator, "PAY_INDICATOR not found");
        assert_eq!(PAYMENT_ADDRESS_FIELD_SEP, first_separator, "first PAYMENT_ADDRESS_FIELD_SEP not found");
        assert_eq!(SOVRIN_INDICATOR, sov_indicator, "SOVRIN_INDICATOR not found");
        assert_eq!(PAYMENT_ADDRESS_FIELD_SEP, second_indicator, "second PAYMENT_ADDRESS_FIELD_SEP not found");
        assert_eq!(VERKEY_LEN, result_address.chars().count(), "address is not 44 bytes");
        assert_eq!(CHECKSUM_LEN, checksum.len(), "checksum is not 4 bytes");

    }
}
