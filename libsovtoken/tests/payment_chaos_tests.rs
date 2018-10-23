extern crate libc;
extern crate sovtoken;
extern crate indy;                      // lib-sdk project

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

pub mod utils;

use indy::ErrorCode;
use sovtoken::logic::config::output_mint_config::MintRequest;
use sovtoken::logic::request::Request;
use sovtoken::utils::base58::IntoBase58;
use sovtoken::utils::random::rand_bytes;
use utils::setup::{Setup, SetupConfig};
use utils::wallet::Wallet;

// ------------------------------------------------------------------------------------------------
const VERKEY_LEN: usize = 32;

// ------------------------------------------------------------------------------------------------
pub fn gen_random_base58_verkey() -> String {
    let vk_bytes = rand_bytes(VERKEY_LEN);
    vk_bytes.into_base58()
}

// ------------------------------------------------------------------------------------------------
pub fn do_minting(pool_handle : i32, wallet : &Wallet, dids : &Vec<&str>, payment_address : &str, amount : i32) {
    let mint_output_json = json!([
        {
            "recipient": payment_address,
            "amount": amount,
        }
    ]).to_string();

    let (mint_req, _) = indy::payments::Payment::build_mint_req(
        wallet.handle,
        Some(&dids[0]),
        &mint_output_json,
        None,
    ).unwrap();

    trace!("{:?}", &mint_req);

    let mint_req = Request::<MintRequest>::multi_sign_request(
        wallet.handle,
        &mint_req,
        dids.clone()
    ).unwrap();

    trace!("{:?}", &mint_req);

    let _ = indy::ledger::Ledger::submit_request(pool_handle, &mint_req).unwrap();
}

// ------------------------------------------------------------------------------------------------
#[test]
pub fn pay_without_outputs_fails() {

    // ---- setup
    sovtoken::api::sovtoken_init();
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();
    let tokens : i32 = 50;

    // ---- create some tokens
    do_minting(pool_handle, &wallet, &dids, &payment_addresses[0], tokens);

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
    sovtoken::api::sovtoken_init();
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();
    let tokens : i32 = 50;

    // ---- create some tokens
    do_minting(pool_handle, &wallet, &dids, &payment_addresses[0], tokens);

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
    sovtoken::api::sovtoken_init();
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    // ---- create some tokens
    do_minting(pool_handle, &wallet, &dids, &payment_addresses[0], 50);


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
    sovtoken::api::sovtoken_init();
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    // ---- create some tokens
    do_minting(pool_handle, &wallet, &dids, &payment_addresses[0], 50);


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
    sovtoken::api::sovtoken_init();
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    // ---- create some tokens
    do_minting(pool_handle, &wallet, &dids, &payment_addresses[0], 50);

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
    sovtoken::api::sovtoken_init();
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 2,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    // ---- create some tokens
    do_minting(pool_handle, &wallet, &dids, &payment_addresses[0], 50);

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
    sovtoken::api::sovtoken_init();
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 3,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    // ---- create some tokens
    do_minting(pool_handle, &wallet, &dids, &payment_addresses[0], 30);
    do_minting(pool_handle, &wallet, &dids, &payment_addresses[1], 10);
    do_minting(pool_handle, &wallet, &dids, &payment_addresses[2], 10);

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
    sovtoken::api::sovtoken_init();
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    // ---- create some tokens
    do_minting(pool_handle, &wallet, &dids, &payment_addresses[0], 50);

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
pub fn pay_with_funds_remaining_with_several_txo_fails() {

    // ---- setup
    sovtoken::api::sovtoken_init();
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 3,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    // ---- create some tokens
    do_minting(pool_handle, &wallet, &dids, &payment_addresses[0], 30);
    do_minting(pool_handle, &wallet, &dids, &payment_addresses[1], 10);
    do_minting(pool_handle, &wallet, &dids, &payment_addresses[2], 10);

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
            "amount": 25
        }
    ]).to_string();

    let (payment_request, _) = indy::payments::Payment::build_payment_req(wallet.handle, Some(dids[0]), &pay_input_json, &pay_output_json, None).unwrap();

    let payment_result = indy::ledger::Ledger::submit_request(pool_handle, &payment_request).unwrap();

    assert!(payment_result.contains("ExtraFundsError"), "Expected ExtraFundsError");
}