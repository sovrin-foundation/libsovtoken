#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
extern crate sovtoken;
extern crate indyrs as indy;
extern crate libc;
extern crate bs58;

use libc::c_char;
use std::ptr;

use indy::future::Future;

pub mod utils;

use utils::wallet::Wallet;
use utils::setup::{Setup, SetupConfig};
use sovtoken::logic::address::strip_qualifier_from_address;
use sovtoken::logic::address::verkey_from_unqualified_address;
use sovtoken::utils::results::ResultHandler;
use sovtoken::utils::test::callbacks;
use sovtoken::utils::ffi_support::c_pointer_from_str;
use sovtoken::{ErrorCode, IndyHandle};

// ***** HELPER METHODS *****
fn build_get_payment_sources_request(wallet_handle: IndyHandle, did: &str, payment_address: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callbacks::cb_ec_string();

    let error_code = sovtoken::api::build_get_utxo_request_handler(
        command_handle,
        wallet_handle,
        c_pointer_from_str(did),
        c_pointer_from_str(payment_address),
        cb
    );

    return ResultHandler::one(ErrorCode::from(error_code), receiver);
}

fn parse_get_payment_sources_response(res: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callbacks::cb_ec_string();

    let error_code = sovtoken::api::parse_get_utxo_response_handler(
        command_handle,
        c_pointer_from_str(res),
        cb
    );

    return ResultHandler::one(ErrorCode::from(error_code), receiver);
}

// ***** HELPER TEST DATA  *****
const COMMAND_HANDLE: i32 = 10;
const WALLET_ID: i32 = 10;
const CB: Option<extern fn(command_handle_: i32, err: i32, mint_req_json: *const c_char) -> i32> = Some(utils::callbacks::empty_callback);
const ADDRESS: &str = "pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q";


// the build_fees_txn_handler requires a callback and this test ensures that we
// receive an error when no callback is provided
#[test]
fn get_utxo_errors_with_no_call_back() {
    let return_error = sovtoken::api::build_get_utxo_request_handler(COMMAND_HANDLE, WALLET_ID, ptr::null(), ptr::null(), None);
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting Callback for 'build_get_utxo_request_handler'");
}

// the build fees txn handler method requires an outputs_json parameter and this test ensures that
// a error is returned when no config is provided
#[test]
fn get_utxo_errors_with_no_payment_address() {
    let return_error = sovtoken::api::build_get_utxo_request_handler(COMMAND_HANDLE, WALLET_ID, ptr::null(), ptr::null(), CB);
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting outputs_json for 'build_fees_txn_handler'");
}

#[test]
fn build_get_utxo_json() {
    let did = bs58::encode("1234567890123456").into_string();
    let expected_operation = json!({
        "type": "10002",
        "address": "dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q"
    });

    let request = build_get_payment_sources_request(WALLET_ID, &did, &ADDRESS).unwrap();

    let request_value: serde_json::value::Value = serde_json::from_str(&request).unwrap();

    assert_eq!(&expected_operation, request_value.get("operation").unwrap());
}

#[test]
pub fn build_and_submit_get_utxo_request() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![10]),
        fees: None
    });
    let payment_addresses = &setup.addresses;
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    let get_utxo_req = build_get_payment_sources_request(wallet.handle, dids[0], &payment_addresses[0]).unwrap();
    let res = indy::ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &get_utxo_req).wait().unwrap();
    let res = parse_get_payment_sources_response(&res).unwrap();

    let res_parsed: Vec<serde_json::Value> = serde_json::from_str(&res).unwrap();
    assert_eq!(res_parsed.len(), 1);
    let utxo = res_parsed.get(0).unwrap().as_object().unwrap();
    assert_eq!(utxo.get("paymentAddress").unwrap().as_str().unwrap(), payment_addresses[0]);
    assert_eq!(utxo.get("amount").unwrap().as_u64().unwrap(), 10);
}

#[test]
pub fn build_and_submit_get_utxo_request_no_utxo() {
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

    let get_utxo_req = build_get_payment_sources_request(wallet.handle, dids[0], &payment_addresses[0]).unwrap();
    let res = indy::ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &get_utxo_req).wait().unwrap();
    let res = parse_get_payment_sources_response(&res).unwrap();

    let res_parsed: Vec<serde_json::Value> = serde_json::from_str(&res).unwrap();
    assert_eq!(res_parsed.len(), 0);
}

#[test]
pub fn payment_address_is_identifier() {
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 1,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });
    let payment_addresses = &setup.addresses;
    let dids = setup.trustees.dids();

    let get_utxo_req = build_get_payment_sources_request(wallet.handle, dids[0], &payment_addresses[0]).unwrap();
    let req: serde_json::Value = serde_json::from_str(&get_utxo_req).unwrap();
    let identifier = req.as_object().unwrap().get("identifier").unwrap().as_str().unwrap();
    let unqualified_addr = strip_qualifier_from_address(&payment_addresses[0]);
    let unqualified_addr = verkey_from_unqualified_address(&unqualified_addr).unwrap();
    assert_eq!(identifier, unqualified_addr);
    assert_ne!(identifier, dids[0]);
}