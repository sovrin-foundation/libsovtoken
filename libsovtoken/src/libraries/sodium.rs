//! our implementation/use for libsodium
//! copied/modeled from master/libindy/src/utils/crypto/box_/sodium.rs
//! uses crate sodiumoxide = {version = "0.0.14", optional = true}
//!


use libc::c_int;
use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::sign;
use sodiumoxide::randombytes;

use indy::api::ErrorCode;

// enumerations/data defining errors CryptoBox can throw
#[derive(Debug)]
pub enum CryptoError {
    InvalidStructure(String),
}

// helper method to help convert encryption data into an array
pub fn clone_into_array<A, T>(slice: &[T]) -> A
    where A: Sized + Default + AsMut<[T]>, T: Clone
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

// CryptoBox type, no members
pub struct CryptoEngine {}

// CryptoEngine provides further encapsulation of sodiumoxide encryption functions
// Modified version of CryptoBox from Indy-SDK
// copied/modeled from master/libindy/src/utils/crypto/box_/sodium.rs
impl CryptoEngine {

    pub fn encrypt(private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if nonce.len() != 24 {
            return Err(CryptoError::InvalidStructure(format!("Invalid nonce")))
        }

        Ok(box_::seal(
            doc,
            &box_::Nonce(clone_into_array(nonce)),
            &box_::PublicKey(clone_into_array(public_key)),
            &box_::SecretKey(clone_into_array(private_key))
        ))
    }

    pub fn decrypt(private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if nonce.len() != 24 {
            return Err(CryptoError::InvalidStructure(format!("Invalid nonce")))
        }

        box_::open(
            doc,
            &box_::Nonce(clone_into_array(nonce)),
            &box_::PublicKey(clone_into_array(public_key)),
            &box_::SecretKey(clone_into_array(private_key))
        )
            .map_err(|err| CryptoError::InvalidStructure(format!("Unable to decrypt data: {:?}", err)))
    }

    pub fn gen_nonce() -> Vec<u8> {
        box_::gen_nonce()[..].to_vec()
    }

    pub fn create_key_pair_for_signature(seed: Option<&[u8]>) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
        if seed.is_some() && seed.unwrap().len() != 32 {
            return Err(CryptoError::InvalidStructure(format!("Invalid seed")));
        }

        let (public_key, private_key) =
            sign::keypair_from_seed(
                &sign::Seed(
                    clone_into_array(
                        seed.unwrap_or(&randombytes::randombytes(32)[..])
                    )
                )
            );

        Ok((public_key[..].to_vec(), private_key[..].to_vec()))
    }

    pub fn sign(private_key: &[u8], doc: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if private_key.len() != 64 {
            return Err(CryptoError::InvalidStructure(format!("Invalid sign key")));
        }

        let mut pr_key: [u8; 64] = [0; 64];
        pr_key.clone_from_slice(private_key);

        Ok(sign::sign_detached(
            doc,
            &sign::SecretKey(pr_key)
        )[..].to_vec())
    }


    pub fn verify(public_key: &[u8], doc: &[u8], sign: &[u8]) -> Result<bool, CryptoError> {
        if sign.len() != 64 {
            return Err(CryptoError::InvalidStructure(format!("Invalid signature")));
        }

        if public_key.len() != 32 {
            return Err(CryptoError::InvalidStructure(format!("Invalid verkey")));
        }

        let mut signature: [u8; 64] = [0; 64];
        signature.clone_from_slice(sign);

        Ok(sign::verify_detached(
            &sign::Signature(signature),
            doc,
            &sign::PublicKey(clone_into_array(public_key))
        ))
    }
}
