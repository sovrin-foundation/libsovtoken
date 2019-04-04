//! Payments module contains functions for working with payments.  :D

use ErrorCode;
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
        To get a sovrin address
        the format of the return is:
            pay:sov:{32 byte address}{4 byte checksum}
    */
    pub fn create_payment_address(&self, wallet_id: i32, config: PaymentAddressConfig) -> Result<String, ErrorCode> {
        trace!("calling self.injected_api.indy_create_key");
        let verkey = self.injected_api.indy_create_key(wallet_id, config)?;

        trace!("got verkey from self.injected_api.indy_create_key {}", verkey);
        return address::qualified_address_from_verkey(&verkey);
    }

    /**
        To get a sovrin address asynchronously.
        the format of the string sent via the callback is:
            pay:sov:{32 byte address}{4 byte checksum}
    */
    pub fn create_payment_address_async<F: 'static>(&self,
                                     wallet_id: i32,
                                     config: PaymentAddressConfig,
                                     mut cb : F) -> ErrorCode where F: FnMut(String, ErrorCode) + Send {

        let cb_closure = move | err: ErrorCode, verkey : String | {
            let res = if ErrorCode::Success == err {
                trace!("got verkey from self.injected_api.indy_create_key_async {}", verkey);
                address::qualified_address_from_verkey(&verkey)
            } else {
                Err(err)
            };

            match res {
                Ok(address) => cb(address, err),
                Err(e) => {
                    error!("got error {:?} from self.injected_api.indy_create_key_async", err);
                    cb("".to_string(), e)
                }
            }
        };

        trace!("calling injected_api.indy_create_key_async");
        let result_code = self.injected_api.indy_create_key_async(wallet_id, config, cb_closure);

        return result_code;
    }
}


// ------------------------------------------------------------------
// unit tests
// ------------------------------------------------------------------

#[cfg(test)]
mod payments_tests {
    extern crate log;

    use std::sync::mpsc::{channel};
    use std::time::Duration;
    use logic::address::*;
    use logic::address::address_tests::gen_random_base58_verkey;
    use utils::random::rand_string;
    use utils::constants::general::PAYMENT_ADDRESS_QUALIFIER;
    use utils::base58::FromBase58;

    use super::*;

    // mock SDK api calls with a call that will generate a random 32 byte string
    struct CreatePaymentSDKMockHandler {}

    impl CryptoAPI for CreatePaymentSDKMockHandler {
        fn indy_create_key(&self, _wallet_id: i32, _config: PaymentAddressConfig) -> Result<String, ErrorCode> {
            return Ok(gen_random_base58_verkey());
        }

        fn indy_crypto_sign<F>(&self, _: i32, _: String, _: String, _: F) -> ErrorCode {
            return ErrorCode::CommonInvalidState;
        }

        fn indy_create_key_async<F: 'static>(&self, _wallet_id: i32, _config: PaymentAddressConfig, mut closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
            closure(ErrorCode::Success, gen_random_base58_verkey());
            return ErrorCode::Success;
        }
    }


    static VALID_SEED_LEN: usize = 32;
    static WALLET_ID: i32 = 10;

    fn validate_address(address : String) {
        // got our result, if its correct, it will look something like this:
        // pay:sov:gzidfrdJtvgUh4jZTtGvTZGU5ebuGMoNCbofXGazFa91234
        // break it up into the individual parts we expect to find and
        // test the validity of the parts
        let qualifier = &address[..ADDRESS_QUAL_LEN];
        let result_address = &address[ADDRESS_QUAL_LEN..];

        assert_eq!(PAYMENT_ADDRESS_QUALIFIER, qualifier, "PAYMENT_ADDRESS_QUALIFIER, not found");
        assert_eq!(VERKEY_LEN + ADDRESS_CHECKSUM_LEN, result_address.from_base58().unwrap().len(), "address is not 36 bytes");
        assert_eq!(VERKEY_LEN, result_address.from_base58_check().unwrap().len(), "verkey is not 32 bytes");
    }

    // This is the happy path test.  Config contains a seed and
    // a fully formatted address is returned.
    #[test]
    fn success_create_payment_with_seed_returns_address() {

        let seed = rand_string(VALID_SEED_LEN);
        let config: PaymentAddressConfig = PaymentAddressConfig { seed };
        let handler = CreatePaymentHandler::new(CreatePaymentSDKMockHandler{});

        let address = match handler.create_payment_address(WALLET_ID, config) {
            Ok(s) => s,
            Err(_) => "".to_string(),
        };

        // got our result, if its correct, it will look something like this:
        // pay:sov:gzidfrdJtvgUh4jZTtGvTZGU5ebuGMoNCbofXGazFa91234
        // break it up into the individual parts we expect to find and
        // test the validity of the parts
        let qualifer = &address[..ADDRESS_QUAL_LEN];
        let result_address = &address[ADDRESS_QUAL_LEN..];

        assert_eq!(PAYMENT_ADDRESS_QUALIFIER, qualifer, "PAYMENT_ADDRESS_QUALIFIER, not found");
        assert_eq!(VERKEY_LEN + ADDRESS_CHECKSUM_LEN, result_address.from_base58().unwrap().len(), "address is not 36 bytes");
        assert_eq!(VERKEY_LEN, result_address.from_base58_check().unwrap().len(), "verkey is not 32 bytes");

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
            Err(_) => "".to_string(),
        };

        // got our result, if its correct, it will look something like this:
        // pay:sov:gzidfrdJtvgUh4jZTtGvTZGU5ebuGMoNCbofXGazFa91234
        // break it up into the individual parts we expect to find and
        // test the validity of the parts
        let qualifer = &address[..ADDRESS_QUAL_LEN];
        let result_address = &address[ADDRESS_QUAL_LEN..];

        assert_eq!(PAYMENT_ADDRESS_QUALIFIER, qualifer, "PAYMENT_ADDRESS_QUALIFIER, not found");
        assert_eq!(VERKEY_LEN + ADDRESS_CHECKSUM_LEN, result_address.from_base58().unwrap().len(), "address is not 36 bytes");
        assert_eq!(VERKEY_LEN, result_address.from_base58_check().unwrap().len(), "verkey is not 32 bytes");

    }

    // Happy path test assumes the CB is valid and it is successfully called
    #[test]
    fn success_create_payment_async() {
        let seed = String::new();
        let config: PaymentAddressConfig = PaymentAddressConfig { seed };

        let handler = CreatePaymentHandler::new(CreatePaymentSDKMockHandler{});

        let (sender, receiver) = channel();

        let cb_closure = move | address : String, err: ErrorCode | {

            if err != ErrorCode::Success {
                sender.send(false).unwrap();
                return;
            }

            validate_address(address.to_string());
            sender.send(true).unwrap();
        };

        let error_code: ErrorCode = handler.create_payment_address_async(WALLET_ID, config, cb_closure);

        let got_good_result = receiver.recv_timeout(Duration::from_secs(10)).unwrap();
        assert_eq!(got_good_result, true);
        assert_eq!(ErrorCode::Success, error_code);
    }
}
