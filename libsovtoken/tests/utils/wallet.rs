/*!
 * A set of test helpers dealing with the wallet.
 */

extern crate rust_indy_sdk as indy;
use self::indy::wallet::Wallet;

use std::panic;

static USEFUL_CREDENTIALS : &'static str = r#"
   {
       "key": "12345678901234567890123456789012",
       "rekey": null,
       "storage": null
   }
"#;

/** Creates a wallet and opens it.
 *  Will delete the current wallet if it exists. It will then create and open the wallet.
 *  
 *  # Errors
 *  May panic on creating the wallet or opening it.
 */
pub fn create_wallet(wallet_name : &str) -> i32 {
   let _ = panic::catch_unwind(| | {
       Wallet::delete(wallet_name, Some(USEFUL_CREDENTIALS)).unwrap();
   });

    Wallet::create("pool_1", wallet_name, None, None, Some(USEFUL_CREDENTIALS)).unwrap();
    let wallet_id: i32 = Wallet::open(wallet_name, None, Some(USEFUL_CREDENTIALS)).unwrap();

    return wallet_id;
}

pub fn close_wallet(wallet_handle: i32) {
    let _ = Wallet::close(wallet_handle);
}