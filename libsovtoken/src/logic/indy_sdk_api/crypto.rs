use utils::{ErrorCode, IndyHandle};

use std::ffi::CString;
use std::time::Duration;

use indy_sys::crypto;
use indy_sys::{ResponseStringCB,
               ResponseSliceCB};

use utils::results::ResultHandler;
use utils::callbacks::ClosureHandler;

pub struct Key {}

impl Key {
    /// Creates key pair in wallet
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open)
    /// * `my_key_json` - Optional key information as json. If none then defaults are used.
    ///
    /// # Example
    /// my_key_json
    /// {
    ///     "seed": string, (optional) Seed that allows deterministic key creation (if not set random one will be created).
    ///                                Can be UTF-8, base64 or hex string.
    ///     "crypto_type": string, // Optional (if not set then ed25519 curve is used); Currently only 'ed25519' value is supported for this field.
    /// }
    /// # Returns
    /// verkey of generated key pair, also used as key identifier
    pub fn create(wallet_handle: IndyHandle, my_key_json: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Key::_create(command_handle, wallet_handle, my_key_json, cb);

        ResultHandler::one(err, receiver)
    }

    /// Creates key pair in wallet
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open)
    /// * `my_key_json` - Optional key information as json. If none then defaults are used.
    /// * `closure` - The closure that is called when finished
    ///
    /// # Example
    /// my_key_json
    /// {
    ///     "seed": string, (optional) Seed that allows deterministic key creation (if not set random one will be created).
    ///                                Can be UTF-8, base64 or hex string.
    ///     "crypto_type": string, // Optional (if not set then ed25519 curve is used); Currently only 'ed25519' value is supported for this field.
    /// }
    /// # Returns
    /// errorcode from calling ffi function. The closure receives the return result
    pub fn create_async<F: 'static>(wallet_handle: IndyHandle, my_key_json: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Key::_create(command_handle, wallet_handle, my_key_json, cb)
    }

    fn _create(command_handle: IndyHandle, wallet_handle: IndyHandle, my_key_json: Option<&str>, cb: Option<ResponseStringCB>) -> ErrorCode {
        let my_key_json = opt_c_str_json!(my_key_json);

        ErrorCode::from(unsafe { crypto::indy_create_key(command_handle, wallet_handle, my_key_json.as_ptr(), cb) })
    }
}

pub struct Crypto {}

impl Crypto {
    /// Signs a message with a key
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open)
    /// * `signer_vk` - key id or verkey of my key. The key must be created by calling Key::create or Did::new
    /// * `message` - the data to be signed
    /// # Returns
    /// the signature
    pub fn sign(wallet_handle: IndyHandle, signer_vk: &str, message: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

        let err = Crypto::_sign(command_handle, wallet_handle, signer_vk, message, cb);

        ResultHandler::one(err, receiver)
    }

    /// Signs a message with a key
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open)
    /// * `signer_vk` - key id or verkey of my key. The key must be created by calling Key::create or Did::new
    /// * `message` - the data to be signed
    /// * `closure` - The closure that is called when finished
    /// # Returns
    /// errorcode from calling ffi function
    pub fn sign_async<F: 'static>(wallet_handle: IndyHandle, signer_vk: &str, message: &[u8], closure: F) -> ErrorCode where F: FnMut(ErrorCode, Vec<u8>) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_slice(Box::new(closure));

        Crypto::_sign(command_handle, wallet_handle, signer_vk, message, cb)
    }

    fn _sign(command_handle: IndyHandle, wallet_handle: IndyHandle, signer_vk: &str, message: &[u8], cb: Option<ResponseSliceCB>) -> ErrorCode {
        let signer_vk = c_str!(signer_vk);
        ErrorCode::from(unsafe {
            crypto::indy_crypto_sign(command_handle, wallet_handle, signer_vk.as_ptr(),
                                     message.as_ptr() as *const u8,
                                     message.len() as u32,
                                     cb)
        })
    }
}

