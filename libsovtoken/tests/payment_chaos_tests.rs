extern crate libc;
extern crate sovtoken;
extern crate indy;                      // lib-sdk project

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

use sovtoken::utils::ErrorCode;

pub mod utils;
use utils::setup::{Setup, SetupConfig};
use utils::wallet::Wallet;

// ------------------------------------------------------------------------------------------------
#[test]
pub fn pay_without_outputs_fails() {

    // ---- setup
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![50]),
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    // ---- create payment request with empty output json
    let txo_1 = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &payment_addresses[0]);

    let pay_input_json = json!([
        txo_1
    ]).to_string();

    let pay_output_json = json!([
    ]).to_string();

    let ec = indy::payments::Payment::build_payment_req(wallet.handle, Some(dids[0]), &pay_input_json, &pay_output_json, None).unwrap_err();
    assert_eq!(ec, ErrorCode::CommonInvalidStructure);

}

// ------------------------------------------------------------------------------------------------
#[test]
pub fn pay_without_inputs_fails() {

    // ---- setup
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![50]),
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let dids = setup.trustees.dids();

    // ---- create payment request with empty input json
    let pay_input_json = json!([

    ]).to_string();

    let pay_output_json = json!([
        {
            "recipient": payment_addresses[0],
            "amount": 50
        }
    ]).to_string();

    let ec = indy::payments::Payment::build_payment_req(wallet.handle, Some(dids[0]), &pay_input_json, &pay_output_json, None).unwrap_err();
    assert_eq!(ec, ErrorCode::CommonInvalidStructure);
}

// ------------------------------------------------------------------------------------------------
#[test]
pub fn pay_from_non_existent_payment_source_fails() {

    // ---- setup
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![50]),
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let dids = setup.trustees.dids();

    // ---- spend tokens from a valid address that is not in the wallet

    let pay_input_json = json!([
        "txo:sov:3x42qH8UkJac1BuorqjSEvuVjvYkXk8sUAqoVPn1fGCwjLPquu4CndzBHBQ5hX6RSmDVnXGdMPrnWDUN5S1ty4YQP87hW8ubMSzu9M56z1FbAQV6aMSX5h"
    ]).to_string();

    let pay_output_json = json!([
        {
            "recipient": payment_addresses[0],
            "amount": 50
        }
    ]).to_string();

    let ec = indy::payments::Payment::build_payment_req(wallet.handle, Some(dids[0]), &pay_input_json, &pay_output_json, None).unwrap_err();
    assert_eq!(ec, ErrorCode::WalletItemNotFound);
}

// ------------------------------------------------------------------------------------------------
#[test]
pub fn pay_from_existent_and_non_existent_payment_source_fails() {

    // ---- setup
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![50]),
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    // ---- spend tokens from inputs with both an address in the wallet and one address that is not in the wallet
    let txo_1 = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &payment_addresses[0]);

    let pay_input_json = json!([
        txo_1, "txo:sov:3x42qH8UkJac1BuorqjSEvuVjvYkXk8sUAqoVPn1fGCwjLPquu4CndzBHBQ5hX6RSmDVnXGdMPrnWDUN5S1ty4YQP87hW8ubMSzu9M56z1FbAQV6aMSX5h"
    ]).to_string();

    let pay_output_json = json!([
        {
            "recipient": payment_addresses[0],
            "amount": 50
        }
    ]).to_string();

    let ec = indy::payments::Payment::build_payment_req(wallet.handle, Some(dids[0]), &pay_input_json, &pay_output_json, None).unwrap_err();
    assert_eq!(ec, ErrorCode::WalletItemNotFound);
}

// ------------------------------------------------------------------------------------------------
#[test]
pub fn pay_with_insufficent_funds_fails() {

    // ---- setup
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![50]),
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    // ---- spend more tokens than we minted
    let txo_1 = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &payment_addresses[0]);

    let pay_input_json = json!([
        txo_1
    ]).to_string();

    let pay_output_json = json!([
        {
            "recipient": payment_addresses[0],
            "amount": 90
        }
    ]).to_string();

    let (payment_request, _) = indy::payments::Payment::build_payment_req(wallet.handle, Some(dids[0]), &pay_input_json, &pay_output_json, None).unwrap();

    let payment_result = indy::ledger::Ledger::submit_request(pool_handle, &payment_request).unwrap();

    assert!(payment_result.contains("InsufficientFundsError"), "Expected InsufficientFundsError");
}

// ------------------------------------------------------------------------------------------------
#[test]
pub fn pay_with_insufficent_funds_with_several_output_addresses_fails() {

    // ---- setup
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 2,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![50, 1]),
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    // ---- spend more tokens than we minted sending the outputs (spending) to several addresses
    let txo_1 = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &payment_addresses[0]);

    let pay_input_json = json!([
        txo_1
    ]).to_string();

    let pay_output_json = json!([
        {
            "recipient": payment_addresses[0],
            "amount": 45
        },
        {
            "recipient": payment_addresses[1],
            "amount": 45
        }
    ]).to_string();

    let (payment_request, _) = indy::payments::Payment::build_payment_req(wallet.handle, Some(dids[0]), &pay_input_json, &pay_output_json, None).unwrap();

    let payment_result = indy::ledger::Ledger::submit_request(pool_handle, &payment_request).unwrap();

    assert!(payment_result.contains("InsufficientFundsError"), "Expected InsufficientFundsError");
}

// ------------------------------------------------------------------------------------------------
#[test]
pub fn pay_with_insufficent_funds_with_several_txo_fails() {

    // ---- setup
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 3,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![30, 10, 10]),
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    // ---- spend more tokens than we minted
    let txo_1 = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &payment_addresses[0]);
    let txo_2 = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &payment_addresses[1]);
    let txo_3 = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &payment_addresses[2]);

    let pay_input_json = json!([
        txo_1, txo_2, txo_3
    ]).to_string();

    let pay_output_json = json!([
        {
            "recipient": payment_addresses[0],
            "amount": 90
        }
    ]).to_string();

    let (payment_request, _) = indy::payments::Payment::build_payment_req(wallet.handle, Some(dids[0]), &pay_input_json, &pay_output_json, None).unwrap();

    let payment_result = indy::ledger::Ledger::submit_request(pool_handle, &payment_request).unwrap();

    assert!(payment_result.contains("InsufficientFundsError"), "Expected InsufficientFundsError");
}

// ------------------------------------------------------------------------------------------------
#[test]
pub fn pay_with_funds_remaining_fails() {

    // ---- setup
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![50]),
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    // ---- spend more tokens than we minted
    let txo_1 = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &payment_addresses[0]);

    let pay_input_json = json!([
        txo_1
    ]).to_string();

    let pay_output_json = json!([
        {
            "recipient": payment_addresses[0],
            "amount": 25
        }
    ]).to_string();

    let (payment_request, _) = indy::payments::Payment::build_payment_req(wallet.handle, Some(dids[0]), &pay_input_json, &pay_output_json, None).unwrap();

    let payment_result = indy::ledger::Ledger::submit_request(pool_handle, &payment_request).unwrap();

    assert!(payment_result.contains("ExtraFundsError"), "Expected ExtraFundsError");
}

// ------------------------------------------------------------------------------------------------
#[test]
pub fn pay_with_funds_remaining_with_several_outputs_fails() {

    // ---- setup
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 2,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![50]),
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    // ---- spend less tokens than we minted
    let txo_1 = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &payment_addresses[0]);

    let pay_input_json = json!([
        txo_1
    ]).to_string();

    let pay_output_json = json!([
        {
            "recipient": payment_addresses[0],
            "amount": 15
        },
        {
            "recipient": payment_addresses[1],
            "amount": 5
        }
    ]).to_string();

    let (payment_request, _) = indy::payments::Payment::build_payment_req(wallet.handle, Some(dids[0]), &pay_input_json, &pay_output_json, None).unwrap();

    let payment_result = indy::ledger::Ledger::submit_request(pool_handle, &payment_request).unwrap();

    assert!(payment_result.contains("ExtraFundsError"), "Expected ExtraFundsError");
}

// ------------------------------------------------------------------------------------------------
#[test]
pub fn pay_with_funds_remaining_with_several_txo_fails() {

    // ---- setup
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 3,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![30, 10, 10]),
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    // ---- spend less tokens than we minted
    let txo_1 = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &payment_addresses[0]);
    let txo_2 = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &payment_addresses[1]);
    let txo_3 = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &payment_addresses[2]);

    let pay_input_json = json!([
        txo_1, txo_2, txo_3
    ]).to_string();

    let pay_output_json = json!([
        {
            "recipient": payment_addresses[0],
            "amount": 25
        }
    ]).to_string();

    let (payment_request, _) = indy::payments::Payment::build_payment_req(wallet.handle, Some(dids[0]), &pay_input_json, &pay_output_json, None).unwrap();

    let payment_result = indy::ledger::Ledger::submit_request(pool_handle, &payment_request).unwrap();

    assert!(payment_result.contains("ExtraFundsError"), "Expected ExtraFundsError");
}