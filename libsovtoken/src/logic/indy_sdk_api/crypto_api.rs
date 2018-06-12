//! Indy-sdk crypto functions
use indy::{IndyHandle, ErrorCode};
use indy::crypto::{Crypto, Key};
use logic::config::payment_address_config::PaymentAddressConfig;
use utils::base58::serialize_bytes;
use utils::json_conversion::JsonSerialize;

/**
    This defines the interfaces for INDY SDK crypto apis, which can be replaced with different implementations
    (aka production vs test time)

    modeling: master/libindy/src/api/crypto.rs
*/
pub trait CryptoAPI {
    fn indy_create_key(&self, wallet_id: i32, config: PaymentAddressConfig) -> Result<String, ErrorCode>;
    fn indy_crypto_sign<F: FnMut(Result<String, ErrorCode>) + 'static + Send>(&self, wallet_handle: i32, verkey: String, message: String, cb: F) -> ErrorCode;
}

// ------------------------------------------------------------------
// CryptoAPI implementation using INDY SDK
// ------------------------------------------------------------------
/**
   This is the "default" implementation of CryptoAPI for use in productions environment
   This implementation calls Indy SDK indy_create_key(...)
*/
pub struct CryptoSdk{}

impl CryptoAPI for CryptoSdk {

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

        return Key::create(wallet_id, Some(&config_json));
    }

    fn indy_crypto_sign<F: FnMut(Result<String, ErrorCode>) + 'static + Send>(
        &self,
        wallet_handle: IndyHandle,
        verkey: String,
        message: String,
        mut cb: F
    ) -> ErrorCode {
        return Crypto::sign_async(wallet_handle, &verkey, message.as_bytes(), move |error_code, vec| {
            if error_code == ErrorCode::Success {
                cb(Ok(serialize_bytes(&vec)));
            } else {
                cb(Err(error_code));
            }
        });
    }
}