//! this module defines traits which represent INDY SDK API calls
use indy::ErrorCode;
use super::config::payment_address_config::PaymentAddressConfig;

/**
    This defines the interfaces for INDY SDK crypto apis, which can be replaced with different implementations
    (aka production vs test time)

    modeling: master/libindy/src/api/crypto.rs
*/
pub trait CryptoAPI {
    fn indy_create_key(&self, wallet_id: i32, config: PaymentAddressConfig) -> Result<String, ErrorCode>;
}
