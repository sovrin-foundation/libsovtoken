//! this module defines traits which represent INDY SDK API calls
use super::payment_address_config::PaymentAddressConfig;
use logic::types::ClosureString;


/**
    This defines the interfaces for INDY SDK crypto apis, which can be replaced with different implementations
    (aka production vs test time)

    modeling: master/libindy/src/api/crypto.rs
*/
pub trait CryptoAPI {
    fn indy_create_key(&self, command_handle: i32, wallet_id: i32, config: PaymentAddressConfig) -> String;
    fn indy_crypto_sign(&self, wallet_id: i32, verkey: String, message: String, call_back: ClosureString);
}