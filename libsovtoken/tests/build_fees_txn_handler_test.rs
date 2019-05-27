#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
extern crate libc;
extern crate sovtoken;
extern crate indyrs as indy;                     // lib-sdk project
extern crate bs58;


use libc::c_char;
use std::ffi::CString;
use std::ptr;

use indy::future::Future;

use sovtoken::ErrorCode;
use sovtoken::logic::request::Request;
use sovtoken::logic::config::set_fees_config::SetFees;
use sovtoken::utils::ffi_support;
use sovtoken::utils::test::callbacks;
use sovtoken::utils::results::ResultHandler;

mod utils;

use utils::setup::{Setup, SetupConfig};
use utils::wallet::Wallet;
use utils::payment::fees;

// ***** HELPER METHODS *****
fn build_set_fees(wallet_handle: i32, did: Option<&str>, fees_json: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callbacks::cb_ec_string();

    let did = did.map(ffi_support::c_pointer_from_str).unwrap_or(std::ptr::null());
    let fees = ffi_support::c_pointer_from_str(fees_json);

    let ec = sovtoken::api::build_set_txn_fees_handler(command_handle, wallet_handle, did, fees, cb);

    return ResultHandler::one(ErrorCode::from(ec), receiver);
}

fn build_get_fees(wallet_handle: i32, did: Option<&str>) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callbacks::cb_ec_string();

    let did = did.map(ffi_support::c_pointer_from_str).unwrap_or(std::ptr::null());

    let ec = sovtoken::api::build_get_txn_fees_handler(command_handle, wallet_handle, did, cb);

    return ResultHandler::one(ErrorCode::from(ec), receiver);
}

fn parse_get_txn_fees_response(response: &str) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callbacks::cb_ec_string();

    let response_pointer = ffi_support::c_pointer_from_str(response);

    let ec = sovtoken::api::parse_get_txn_fees_response_handler(command_handle, response_pointer, cb);

    return ResultHandler::one(ErrorCode::from(ec), receiver);
}

// ***** HELPER TEST DATA  *****
const COMMAND_HANDLE: i32 = 10;
const WALLET_ID: i32 = 10;
static INVALID_OUTPUT_JSON: &'static str = r#"{"totally" : "Not a Number", "bobby" : "DROP ALL TABLES"}"#;
const CB: Option<extern fn(command_handle_: i32, err: i32, mint_req_json: *const c_char) -> i32> = Some(utils::callbacks::empty_callback);
static FAKE_DID: &'static str = "Enfru5LNlA2CnA5n4Hfze";


// the build_fees_txn_handler requires a callback and this test ensures that we
// receive an error when no callback is provided
#[test]
fn add_fees_errors_with_no_call_back() {
    let return_error = sovtoken::api::build_set_txn_fees_handler(COMMAND_HANDLE, WALLET_ID, ptr::null(), ptr::null(), None);
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting Callback for 'build_fees_txn_handler'");
}

// the build fees txn handler method requires an outputs_json parameter and this test ensures that 
// a error is returned when no config is provided
#[test]
fn add_fees_errors_with_no_fees_json() {
    let return_error = sovtoken::api::build_set_txn_fees_handler(COMMAND_HANDLE, WALLET_ID, ptr::null(), ptr::null(), CB);
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting outputs_json for 'build_fees_txn_handler'");
}

#[test]
fn add_fees_errors_with_invalid_fees_json() {
    let fees_str = CString::new(INVALID_OUTPUT_JSON).unwrap();
    let fees_str_ptr = fees_str.as_ptr();
    let return_error = sovtoken::api::build_set_txn_fees_handler(COMMAND_HANDLE, WALLET_ID, ptr::null(), fees_str_ptr, CB);
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting Valid JSON for 'build_fees_txn_handler'");
}

#[test]
fn add_fees_invalid_did() {
    let fees = json!({
        "1000": 4,
        "20001": 13,
    }).to_string();

    let err = build_set_fees(WALLET_ID, Some(FAKE_DID), &fees).unwrap_err();
    assert_eq!(ErrorCode::CommonInvalidStructure, err);
}

#[test]
fn add_fees_invalid_fees() {
    let fees = "1000";
    let did = bs58::encode("1234567890123456").into_string();
    let err = build_set_fees(WALLET_ID, Some(&did), fees).unwrap_err();
    assert_eq!(ErrorCode::CommonInvalidStructure, err);
}

#[test]
fn add_fees_json() {
    let fees = json!({
        "3": 6,
        "20001": 12
    });
    let expected_operation = json!({
        "type": "20000",
        "fees": fees,
    });

    let did = bs58::encode("1234567890123456").into_string();
    let fees_request = build_set_fees(WALLET_ID, Some(&did), &fees.to_string()).unwrap();

    let request_value: serde_json::value::Value = serde_json::from_str(&fees_request).unwrap();

    assert_eq!(&expected_operation, request_value.get("operation").unwrap());
}

#[test]
fn build_get_fees_req() {
    let expected_operation = json!({
        "type": "20001",
    });

    let did = bs58::encode("1234567890123456").into_string();
    let get_fees_request = build_get_fees(WALLET_ID, Some(&did)).unwrap();

    let request_value: serde_json::value::Value = serde_json::from_str(&get_fees_request).unwrap();

    assert_eq!(&expected_operation, request_value.get("operation").unwrap());
}

#[test]
fn build_get_fees_error_with_invalid_did() {
    let err = build_get_fees(WALLET_ID, Some(FAKE_DID)).unwrap_err();
    assert_eq!(ErrorCode::CommonInvalidStructure, err);
}

#[test]
fn add_fees_json_for_any_key() {
    sovtoken::api::sovtoken_init();
    let wallet = Wallet::new();
    let fees = json!({
        "3": 6,
        "TXN_ALIAS": 12,
        "TXN ALIAS WITH SPACE": 12,
    });
    let expected_operation = json!({
        "type": "20000",
        "fees": fees,
    });

    let did = bs58::encode("1234567890123456").into_string();
    let fees_request = build_set_fees(wallet.handle, Some(&did), &fees.to_string()).unwrap();

    let request_value: serde_json::value::Value = serde_json::from_str(&fees_request).unwrap();
    assert_eq!(&expected_operation, request_value.get("operation").unwrap());
}

#[test]
pub fn build_and_submit_set_fees() {
    let payment_method = sovtoken::utils::constants::general::PAYMENT_METHOD_NAME;
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 0,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    let fees = json!({
        "100": 1,
        "101": 2
    }).to_string();

    fees::set_fees(pool_handle, wallet.handle, &payment_method, &fees, &dids, Some(dids[0]));
    let current_fees = fees::get_fees(&wallet, pool_handle, Some(dids[0]));
    let current_fees_value: serde_json::Value = serde_json::from_str(&current_fees).unwrap();

    assert_eq!(current_fees_value["101"].as_u64().unwrap(), 2);
    assert_eq!(current_fees_value["100"].as_u64().unwrap(), 1);

    let fees = json!({
        "100": 0,
        "101": 0
    }).to_string();

    fees::set_fees(pool_handle, wallet.handle, &payment_method, &fees, &dids, Some(dids[0]));
}

#[test]
pub fn build_and_submit_set_fees_with_names() {
    let payment_method = sovtoken::utils::constants::general::PAYMENT_METHOD_NAME;
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 0,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    let fees = json!({
        "NYM": 1,
        "ATTRIB": 2
    }).to_string();

    fees::set_fees(pool_handle, wallet.handle, &payment_method, &fees, &dids, Some(dids[0]));
    let current_fees = fees::get_fees(&wallet, pool_handle, Some(dids[0]));
    let current_fees_value: serde_json::Value = serde_json::from_str(&current_fees).unwrap();

    assert_eq!(current_fees_value["NYM"].as_u64().unwrap(), 1);
    assert_eq!(current_fees_value["ATTRIB"].as_u64().unwrap(), 2);

    let fees = json!({
        "NYM": 0,
        "ATTRIB": 0
    }).to_string();

    fees::set_fees(pool_handle, wallet.handle, &payment_method, &fees, &dids, Some(dids[0]));
}

#[test]
pub fn build_and_submit_get_fees_with_empty_did() {
    let payment_method = sovtoken::utils::constants::general::PAYMENT_METHOD_NAME;
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 0,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: None,
        fees: None,
    });
    let pool_handle = setup.pool_handle;
    let dids = setup.trustees.dids();

    let fees = json!({
        "NYM": 1,
        "ATTRIB": 2
    }).to_string();

    fees::set_fees(pool_handle, wallet.handle, &payment_method, &fees, &dids, Some(dids[0]));
    let current_fees = fees::get_fees(&wallet, pool_handle, None);
    let current_fees_value: serde_json::Value = serde_json::from_str(&current_fees).unwrap();

    assert_eq!(current_fees_value["NYM"].as_u64().unwrap(), 1);
    assert_eq!(current_fees_value["ATTRIB"].as_u64().unwrap(), 2);

    let fees = json!({
        "NYM": 0,
        "ATTRIB": 0
    }).to_string();

    fees::set_fees(pool_handle, wallet.handle, &payment_method, &fees, &dids, Some(dids[0]));
}