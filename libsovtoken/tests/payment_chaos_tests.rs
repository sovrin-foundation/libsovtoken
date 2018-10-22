extern crate libc;
extern crate sovtoken;
extern crate indy;                      // lib-sdk project

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;

pub mod utils;

use indy::ErrorCode;
use sovtoken::logic::address::qualified_address_from_verkey;
use sovtoken::logic::config::output_mint_config::MintRequest;
use sovtoken::logic::request::Request;
use sovtoken::logic::parsers::common::TXO;
use sovtoken::utils::base58::{IntoBase58, FromBase58};
use sovtoken::utils::random::rand_bytes;
use utils::setup::{Setup, SetupConfig};
use utils::wallet::Wallet;

pub const VERKEY_LEN: usize = 32;
static VALID_CONFIG_EMPTY_SEED_JSON: &'static str = r#"{}"#;

// static INPUTS : &'static str = "[\"pay:sov:1\", \"pay:sov:2\"]";
// static OUTPUTS : &'static str = "[{\"recipient\": \"pay:sov:1\", \"amount\":1}, {\"recipient\": \"pay:sov:2\", \"amount\":2}]";



pub fn gen_random_base58_verkey() -> String {
    let vk_bytes = rand_bytes(VERKEY_LEN);
    vk_bytes.into_base58()
}

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

    let result = indy::ledger::Ledger::submit_request(pool_handle, &mint_req).unwrap();
}

#[test]
pub fn pay_to_non_existent_payment_source_fails() {

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

    // ---- spend tokens to address not created
    let verkey = gen_random_base58_verkey().to_string();
    let address = qualified_address_from_verkey(&verkey).unwrap();

    let txo_1 = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &payment_addresses[0]);

    let pay_input_json = json!([
        txo_1
    ]).to_string();

    let pay_output_json = json!([
        {
            "recipient": address,
            "amount": 50
        }
    ]).to_string();

    let (payment_request, op_code) = indy::payments::Payment::build_payment_req(wallet.handle, Some(dids[0]), &pay_input_json, &pay_output_json, None).unwrap();

    let payment_result = indy::ledger::Ledger::submit_request(pool_handle, &payment_request).unwrap();

    assert!(false, "hey -> {:?}", pay_input_json);
}

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

    match indy::payments::Payment::build_payment_req(wallet.handle, Some(dids[0]), &pay_input_json, &pay_output_json, None) {
        Ok(_) => {
            assert!(false, "expected build_payment_req to fail");
        },
        Err(ec) => {
            assert_eq!(ec, ErrorCode::WalletItemNotFound);
        }

    }
}


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
    let verkey = gen_random_base58_verkey().to_string();
    let address = qualified_address_from_verkey(&verkey).unwrap();

    let txo_1 = utils::payment::get_utxo::get_first_utxo_txo_for_payment_address(&wallet, pool_handle, dids[0], &payment_addresses[0]);

    let pay_input_json = json!([
        txo_1
    ]).to_string();

    let pay_output_json = json!([
        {
            "recipient": address,
            "amount": 90
        }
    ]).to_string();

    let (payment_request, op_code) = indy::payments::Payment::build_payment_req(wallet.handle, Some(dids[0]), &pay_input_json, &pay_output_json, None).unwrap();

    let payment_result = indy::ledger::Ledger::submit_request(pool_handle, &payment_request).unwrap();

    assert!(payment_result.contains("InsufficientFundsError"), "Expected InsufficientFundsError");
}