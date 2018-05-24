//! Payments module contains functions for working with payments.  :D
#![allow(unused_variables)]
#![allow(unused_imports)]
#[warn(unused_imports)]

use indy::{ErrorCode, IndyHandle};
use indy::crypto::{Key, Crypto};
use logic::indysdk_api::CryptoAPI;
use super::config::payment_address_config::PaymentAddressConfig;
use utils::ffi_support::{string_from_char_ptr, cstring_from_str, c_pointer_from_string};
use utils::general::some_or_none_option_u8;
use utils::json_conversion::JsonSerialize;
use logic::address;
use logic::address::{PAY_INDICATOR, SOVRIN_INDICATOR, PAYMENT_ADDRESS_FIELD_SEP, CHECKSUM_LEN, VALID_ADDRESS_LEN};
use logic::types::ClosureString;



// ------------------------------------------------------------------
// CryptoAPI implementation using INDY SDK
// ------------------------------------------------------------------
/**
   Implementation of CryptoAPI for use in productions environment
   This implementation calls Indy SDK indy_create_key(...)
*/
pub struct CreatePaymentSDK{}

impl CryptoAPI for CreatePaymentSDK {

    /**
       creates fully formatted address based on inputted seed.  If seed is empty
       then a randomly generated seed is used by libsodium
       the format of the return is:
           pay:sov:{32 byte address}{4 byte checksum}
    */
    fn indy_create_key(&self, wallet_id: IndyHandle, config: PaymentAddressConfig) -> Result<String, ErrorCode> {

        debug!("create_payment_address calling indy_create_key");
        let mut config_json: String = config.to_json().unwrap();

        // indy-sdk expects a valid but empty input to be this below
        // so if no seed was provided, create the json to look like this instead
        if 0 == config.seed.chars().count() {
            config_json = r#"{ }"#.to_string();
        }

        return Key::create(wallet_id, &config_json);
    }

    fn indy_crypto_sign (
        &self,
        wallet_handle: IndyHandle,
        verkey: String,
        message: String,
    ) -> Result<String, ErrorCode>
    {
         return Crypto::sign(wallet_handle, &verkey, message.as_bytes())
             .map(|vec| String::from_utf8(vec).unwrap());

        //return Ok(verkey + "signed");
    }
}

// ------------------------------------------------------------------
// CreatePaymentHandler
// ------------------------------------------------------------------
/**
    CreatePaymentHandler contains methods for creating a fully formatted address based on inputted
    seed.  If seed is empty then a randomly generated seed is used by libsodium

    In production runtime environment, the expectation is T is CreatePaymentSDK
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
    
    use log::*;
    use std::panic;
    use utils::general::StringUtils;
    use utils::logger::ConsoleLogger;
    use utils::random::rand_string;

    use super::*;

    // mock SDK api calls with a call that will generate a random 32 byte string
    struct CreatePaymentSDKMockHandler {}
    impl CryptoAPI for CreatePaymentSDKMockHandler {
        fn indy_create_key(&self, wallet_id: i32, config: PaymentAddressConfig) -> Result<String, ErrorCode> {
            return Ok(rand_string(32));
        }
        fn indy_crypto_sign (
            &self,
            wallet_handle: i32,
            verkey: String,
            message: String,
        ) -> Result<String, ErrorCode> {
            return Err(ErrorCode::CommonInvalidParam2);
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
        let result_address = &address[8..40];

        let checksum: String = address::get_checksum(&address).unwrap();

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
        let result_address = &address[8..40];

        let checksum: String = address::get_checksum(&address).unwrap();

        assert_eq!(PAY_INDICATOR, pay_indicator, "PAY_INDICATOR not found");
        assert_eq!(PAYMENT_ADDRESS_FIELD_SEP, first_separator, "first PAYMENT_ADDRESS_FIELD_SEP not found");
        assert_eq!(SOVRIN_INDICATOR, sov_indicator, "SOVRIN_INDICATOR not found");
        assert_eq!(PAYMENT_ADDRESS_FIELD_SEP, second_indicator, "second PAYMENT_ADDRESS_FIELD_SEP not found");
        assert_eq!(VALID_ADDRESS_LEN, result_address.chars().count(), "address is not 32 bytes");
        assert_eq!(CHECKSUM_LEN, checksum.len(), "checksum is not 4 bytes");

    }
}
