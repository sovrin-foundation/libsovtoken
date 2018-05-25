#![cfg(test)]

/*!
 * A set of test helpers dealing with the wallet.
 */

extern crate rust_indy_sdk as indy;
use self::indy::wallet::Wallet;

use std;

/** Creates a wallet and opens it.
 *  Will delete the current wallet if it exists. It will then create and open the wallet.
 *  
 *  # Errors
 *  May panic on creating the wallet or opening it.
 */
pub fn create_wallet(wallet_name : &str) -> i32 {
    let _ = std::panic::catch_unwind(| | {
        Wallet::delete(wallet_name).unwrap();
    });

    Wallet::create("pool_1", wallet_name, None, Some("{}"), None).unwrap();
    let wallet_id: i32 = Wallet::open(wallet_name, None, None).unwrap();

    return wallet_id;
}