//!  what does this module do?

use openssl::hash::{hash2, MessageDigest, Hasher, DigestBytes};
use indy::ErrorCode;

pub const HASH_OUTPUT_LEN: usize = 32;

pub struct Digest {
    data: DigestBytes
}

impl Digest {
    fn new(data: DigestBytes) -> Digest {
        Digest {
            data: data
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}

pub struct Hash {}

impl Hash {
    pub fn new_context() -> Result<Hasher, ErrorCode> {
        Ok(Hasher::new(MessageDigest::sha256()).map_err(|_| ErrorCode::CommonInvalidState)?)
    }

    pub fn hash_empty() -> Result<Digest, ErrorCode> {
        Ok(Digest::new(hash2(MessageDigest::sha256(), &[]).map_err(|_| ErrorCode::CommonInvalidState)?))

    }

    pub fn hash_leaf<T>(leaf: &T) -> Result<Digest, ErrorCode> where T: Hashable {
        let mut ctx = Hash::new_context()?;
        ctx.update(&[0x00]).map_err(|_| ErrorCode::CommonInvalidState)?;
        leaf.update_context(&mut ctx)?;
        Ok(Digest::new(ctx.finish2().map_err(|_| ErrorCode::CommonInvalidState)?))
    }

    pub fn hash_nodes<T>(left: &T, right: &T) -> Result<Digest, ErrorCode> where T: Hashable {
        let mut ctx = Hash::new_context()?;
        ctx.update(&[0x01]).map_err(|_| ErrorCode::CommonInvalidState)?;
        left.update_context(&mut ctx)?;
        right.update_context(&mut ctx)?;
        Ok(Digest::new(ctx.finish2().map_err(|_| ErrorCode::CommonInvalidState)?))
    }

}

/// The type of values stored in a `MerkleTree` must implement
/// this trait, in order for them to be able to be fed
/// to a Ring `Context` when computing the hash of a leaf.
///
/// A default instance for types that already implements
/// `AsRef<[u8]>` is provided.
///
/// ## Example
///
/// Here is an example of how to implement `Hashable` for a type
/// that does not (or cannot) implement `AsRef<[u8]>`:
///
/// ```
/// extern crate openssl;
/// extern crate sovtoken;
/// extern crate rust_libindy_wrapper as indy;
/// use self::openssl::hash::{hash2, MessageDigest, Hasher, DigestBytes};
/// use self::sovtoken::logic::hash::Hashable;
/// use self::indy::ErrorCode;
///
/// struct PublicKey {
///     key: String
/// }
///
/// impl PublicKey {
///     pub fn to_bytes(&self) -> Vec<u8> {
///         self.key.as_bytes().to_vec()
///     }
/// }
///
/// impl Hashable for PublicKey {
///     fn update_context(&self, context: &mut Hasher) -> Result<(), ErrorCode> {
///         let bytes: Vec<u8> = self.to_bytes();
///         Ok(context.update(&bytes).map_err(|_| ErrorCode::CommonInvalidState)?)
///     }
/// }
/// ```
pub trait Hashable {

    /// Update the given `context` with `self`.
    ///
    /// See `openssl::hash::Hasher::update` for more information.
    fn update_context(&self, context: &mut Hasher) -> Result<(), ErrorCode>;

}

impl <T: AsRef<[u8]>> Hashable for T {

    fn update_context(&self, context: &mut Hasher) -> Result<(), ErrorCode> {
        Ok(context.update(self.as_ref()).map_err(|_| ErrorCode::CommonInvalidState)?)
    }
}