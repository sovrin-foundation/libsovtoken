#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
extern crate libc;
extern crate sovtoken;
extern crate indyrs as indy;                     // lib-sdk project
extern crate bs58;

mod utils;
use indy::future::Future;
use utils::wallet::Wallet;
use utils::setup::{Setup, SetupConfig};
use indy::payments::{sign_with_address, verify_with_address};
use indy::ErrorCode;

#[test]
pub fn sign_with_address_works() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 1,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });

    let addr = setup.addresses.get(0).unwrap();
    let msg = vec![1, 2, 3, 4];
    let sig = sign_with_address(wallet.handle, addr, msg.as_slice()).wait().unwrap();
    assert!(verify_with_address(addr, msg.as_slice(), sig.as_slice()).wait().unwrap());
}

#[test]
pub fn sign_with_address_works_for_incorrect_signature() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 1,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });

    let addr = setup.addresses.get(0).unwrap();
    let msg = vec![1, 2, 3, 4];
    let sig = sign_with_address(wallet.handle, addr, msg.as_slice()).wait().unwrap();
    assert_eq!(verify_with_address(addr, msg.as_slice(), &sig[..sig.len()-1]).wait().unwrap_err().error_code, ErrorCode::CommonInvalidStructure);
}

#[test]
pub fn sign_with_address_fails_for_invalid_addr() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 1,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });

    let addr = setup.addresses.get(0).unwrap();
    let msg = vec![1, 2, 3, 4];
    assert_eq!(sign_with_address(wallet.handle, &addr[..addr.len()-1], msg.as_slice()).wait().unwrap_err().error_code, ErrorCode::CommonInvalidStructure);
}

#[test]
pub fn sign_with_address_works_for_no_such_addr_in_wallet() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 1,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });
    let wallet_2 = Wallet::new();

    let addr = setup.addresses.get(0).unwrap();
    let msg = vec![1, 2, 3, 4];
    //TODO: Should be WalletItemNotFound
    assert_eq!(sign_with_address(wallet_2.handle, &addr[..addr.len()-1], msg.as_slice()).wait().unwrap_err().error_code, ErrorCode::CommonInvalidStructure);
}

#[test]
pub fn verify_with_address_works_for_invalid_address() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 1,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });

    let addr = setup.addresses.get(0).unwrap();
    let msg = vec![1, 2, 3, 4];
    let sig = sign_with_address(wallet.handle, addr, msg.as_slice()).wait().unwrap();
    assert_eq!(verify_with_address(&addr[..addr.len()-1], msg.as_slice(), &sig[..sig.len()-1]).wait().unwrap_err().error_code, ErrorCode::CommonInvalidStructure);
}
