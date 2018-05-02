//! our implementation/use for libsodium
//! copied/modeled from master/libindy/src/utils/crypto/box_/sodium.rs
//! uses crate sodiumoxide = {version = "0.0.14", optional = true}
//!


use libc::c_int;
use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::sign;
use sodiumoxide::randombytes;
// use utils::byte_array::_clone_into_array;


pub struct CryptoBox {}

impl CryptoBox {
    /*
    pub fn encrypt(private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CommonError> {
        if nonce.len() != 24 {
            return Err(CommonError::InvalidStructure(format!("Invalid nonce")))
        }

        Ok(box_::seal(
            doc,
            &box_::Nonce(_clone_into_array(nonce)),
            &box_::PublicKey(_clone_into_array(public_key)),
            &box_::SecretKey(_clone_into_array(private_key))
        ))
    }

    pub fn decrypt(private_key: &[u8], public_key: &[u8], doc: &[u8], nonce: &[u8]) -> Result<Vec<u8>, CommonError> {
        if nonce.len() != 24 {
            return Err(CommonError::InvalidStructure(format!("Invalid nonce")))
        }

        box_::open(
            doc,
            &box_::Nonce(_clone_into_array(nonce)),
            &box_::PublicKey(_clone_into_array(public_key)),
            &box_::SecretKey(_clone_into_array(private_key))
        )
            .map_err(|err| CommonError::InvalidStructure(format!("Unable to decrypt data: {:?}", err)))
    }

    pub fn gen_nonce() -> Vec<u8> {
        box_::gen_nonce()[..].to_vec()
    }*/

    pub fn create_key_pair_for_signature(seed: Option<&[u8]>) -> Result<(Vec<u8>, Vec<u8>), CommonError> {
        if seed.is_some() && seed.unwrap().len() != 32 {
            return Err(CommonError::InvalidStructure(format!("Invalid seed")));
        }

        let (public_key, private_key) =
            sign::keypair_from_seed(
                &sign::Seed(
                    _clone_into_array(
                        seed.unwrap_or(&randombytes::randombytes(32)[..])
                    )
                )
            );

        Ok((public_key[..].to_vec(), private_key[..].to_vec()))
    }


    pub fn sign(private_key: &[u8], doc: &[u8]) -> Result<Vec<u8>, CommonError> {
        if private_key.len() != 64 {
            return Err(CommonError::InvalidStructure(format!("Invalid sign key")));
        }

        let mut pr_key: [u8; 64] = [0; 64];
        pr_key.clone_from_slice(private_key);

        Ok(sign::sign_detached(
            doc,
            &sign::SecretKey(pr_key)
        )[..].to_vec())
    }

    /*
    pub fn verify(public_key: &[u8], doc: &[u8], sign: &[u8]) -> Result<bool, CommonError> {
        if sign.len() != 64 {
            return Err(CommonError::InvalidStructure(format!("Invalid signature")));
        }

        if public_key.len() != 32 {
            return Err(CommonError::InvalidStructure(format!("Invalid verkey")));
        }

        let mut signature: [u8; 64] = [0; 64];
        signature.clone_from_slice(sign);

        Ok(sign::verify_detached(
            &sign::Signature(signature),
            doc,
            &sign::PublicKey(_clone_into_array(public_key))
        ))
    }

    pub fn sk_to_curve25519(sk: &[u8]) -> Result<Vec<u8>, CommonError> {
        if sk.len() != 64 {
            return Err(CommonError::InvalidStructure(format!("Invalid signkey")));
        }

        let mut from: [u8; 64] = [0; 64];
        from.clone_from_slice(sk);
        let mut to: [u8; 32] = [0; 32];
        unsafe {
            crypto_sign_ed25519_sk_to_curve25519(&mut to, &from);
        }
        Ok(to.iter().cloned().collect())
    }

    pub fn vk_to_curve25519(pk: &[u8]) -> Result<Vec<u8>, CommonError> {
        if pk.len() != 32 {
            return Err(CommonError::InvalidStructure(format!("Invalid verkey")));
        }

        let mut from: [u8; 32] = [0; 32];
        from.clone_from_slice(pk);
        let mut to: [u8; 32] = [0; 32];
        unsafe {
            crypto_sign_ed25519_pk_to_curve25519(&mut to, &from);
        }
        Ok(to.iter().cloned().collect())
    }*/
}
