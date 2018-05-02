//! our implementation/use for rust_base58
//! copied/modeled from indy-sdk/master/libindy/src/utils/crypto/base58/rust_base58.rs
//! uses crate rust-base58 = {version = "0.0.4"}
//!

use rust_base58::{ToBase58, FromBase58};

// enumerations/data defining errors Base58 can throw
#[derive(Debug)]
pub enum Base58Error {
    InvalidStructure(String),
}

// Defines type Base58 which wraps the rust_base58 crate
pub struct Base58 {}

/*
   Base58 adds further encapsulation from rust_base58 functions.
   Its a modified version of Base58 from Indy-SDK
*/
impl Base58 {
    pub fn encode(doc: &[u8]) -> String {
        doc.to_base58()
    }

    pub fn decode(doc: &str) -> Result<Vec<u8>, Base58Error> {
        doc.from_base58()
            .map_err(|err| Base58Error::InvalidStructure(format!("{}", err)))
    }
}
