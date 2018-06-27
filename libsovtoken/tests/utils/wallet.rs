/*!
 * A set of test helpers dealing with the wallet.
 */

extern crate rust_indy_sdk as indy;
extern crate sovtoken;

use self::indy::ErrorCode;
use self::indy::wallet::Wallet as IndyWallet;
use self::sovtoken::utils::random::rand_string;

use std::panic;

static USEFUL_CREDENTIALS : &'static str = r#"
   {
       "key": "12345678901234567890123456789012",
       "rekey": null,
       "storage": null
   }
"#;


pub struct Wallet {
    name: String,
    pub handle: i32,
}

impl Wallet {
    pub fn new() -> Wallet {
        let name = rand_string(20);
        Wallet::create(&name).unwrap();
        let handle = Wallet::open(&name).unwrap();

        Wallet { name, handle }
    }

    fn open(name: &str) -> Result<i32, ErrorCode> {
        IndyWallet::open(&name, None, Some(USEFUL_CREDENTIALS))
    }

    fn create(name: &str) -> Result<(), ErrorCode> {
        IndyWallet::create("pool_1", &name, None, None, Some(USEFUL_CREDENTIALS))
    }

    fn close(handle: i32) -> Result<(), ErrorCode> {
        IndyWallet::close(handle)
    }

    fn delete(name: &str) -> Result<(), ErrorCode> {
        IndyWallet::delete(name, Some(USEFUL_CREDENTIALS))
    }
}

impl Drop for Wallet {
    fn drop(&mut self) {
        Wallet::close(self.handle).unwrap();
        Wallet::delete(&self.name).unwrap();
    }
}
