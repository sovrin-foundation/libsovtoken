#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate libc;
extern crate sovtoken;
extern crate indy;                      // lib-sdk project
extern crate bs58;
pub mod utils;

use indy::ErrorCode;
use libc::c_char;
use sovtoken::utils::ffi_support;
use sovtoken::utils::test::callbacks;
use std::ffi::CString;
use std::ptr;
use std::sync::mpsc::{Receiver};
use std::time::Duration;
use utils::payment::fees;
use utils::setup::{Setup, SetupConfig};
use utils::wallet::Wallet;

// ***** HELPER METHODS *****
extern "C" fn empty_create_payment_callback(_command_handle: i32, _err: i32, _mint_req_json: *const c_char) -> i32 {
    return ErrorCode::Success as i32;
}

// ***** HELPER TEST DATA  *****

const COMMAND_HANDLE:i32 = 10;
const WALLET_ID:i32 = 10;
static INVALID_OUTPUT_JSON: &'static str = r#"{"totally" : "Not a Number", "bobby" : "DROP ALL TABLES"}"#;
const CB : Option<extern fn(command_handle_: i32, err: i32, mint_req_json: *const c_char) -> i32 > = Some(empty_create_payment_callback);
static FAKE_DID : &'static str = "Enfru5LNlA2CnA5n4Hfze";


fn call_set_fees(did: &str, fees_json: serde_json::value::Value) -> (ErrorCode, Receiver<(ErrorCode, String)>) {
    let (receiver, command_handle, cb) = callbacks::cb_ec_string();

    let did_pointer = ffi_support::c_pointer_from_str(did);
    let fees_pointer = ffi_support::c_pointer_from_string(fees_json.to_string());

    let ec = sovtoken::api::build_set_txn_fees_handler(command_handle, WALLET_ID, did_pointer, fees_pointer, cb);

    return (ErrorCode::from(ec), receiver);
}

// the build_fees_txn_handler requires a callback and this test ensures that we
// receive an error when no callback is provided
#[test]
fn errors_with_no_call_back() {
    let return_error = sovtoken::api::build_set_txn_fees_handler(COMMAND_HANDLE, WALLET_ID, ptr::null(),ptr::null(), None);
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting Callback for 'build_fees_txn_handler'");
}

// the build fees txn handler method requires an outputs_json parameter and this test ensures that 
// a error is returned when no config is provided
#[test]
fn errors_with_no_fees_json() {
    let return_error = sovtoken::api::build_set_txn_fees_handler(COMMAND_HANDLE, WALLET_ID, ptr::null(),ptr::null(), CB);
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting outputs_json for 'build_fees_txn_handler'");
}

#[test]
fn errors_with_invalid_fees_json() {
    let fees_str = CString::new(INVALID_OUTPUT_JSON).unwrap();
    let fees_str_ptr = fees_str.as_ptr();
    let return_error = sovtoken::api::build_set_txn_fees_handler(COMMAND_HANDLE, WALLET_ID, ptr::null(), fees_str_ptr, CB);
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting Valid JSON for 'build_fees_txn_handler'");
}

#[test]
fn add_fees_invalid_json_key() {
    let fees = json!({
        "1000": 4,
        "XFER_PUBLIC": 13,
    });

    let (ec, receiver) = call_set_fees(FAKE_DID, fees);
    let received = receiver.recv_timeout(Duration::from_millis(300));

    assert_eq!(ErrorCode::CommonInvalidStructure, ec);
    assert!(received.is_err());
}

#[test]
fn add_fees_json() {
    sovtoken::api::sovtoken_init();
    let fees = json!({
        "3": 6,
        "20001": 12
    });
    let expected_operation = json!({
        "type": "20000",
        "fees": fees,
    });

    let did = bs58::encode("1234567890123456").into_string();
    let (ec_initial, receiver) = call_set_fees(&did, fees);
    let (ec_callback, fees_request) = receiver.recv().unwrap();

    let request_value: serde_json::value::Value = serde_json::from_str(&fees_request).unwrap();

    assert_eq!(ErrorCode::Success, ec_initial);
    assert_eq!(ErrorCode::Success, ec_callback);
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
        "202": 0,
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

    assert_eq!(current_fees_value["1"].as_u64().unwrap(), 1);
    assert_eq!(current_fees_value["100"].as_u64().unwrap(), 2);

    let fees = json!({
        "NYM": 0,
        "ATTRIB": 0
    }).to_string();

    fees::set_fees(pool_handle, wallet.handle, &payment_method, &fees, &dids, Some(dids[0]));

}

#[test]
pub fn build_and_submit_set_fees_with_empty_did() {
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

    fees::set_fees(pool_handle, wallet.handle, &payment_method, &fees, &dids, None);
    let current_fees = fees::get_fees(&wallet, pool_handle, None);
    let current_fees_value: serde_json::Value = serde_json::from_str(&current_fees).unwrap();

    assert_eq!(current_fees_value["1"].as_u64().unwrap(), 1);
    assert_eq!(current_fees_value["100"].as_u64().unwrap(), 2);

    let fees = json!({
        "NYM": 0,
        "ATTRIB": 0
    }).to_string();

    fees::set_fees(pool_handle, wallet.handle, &payment_method, &fees, &dids, None);

}