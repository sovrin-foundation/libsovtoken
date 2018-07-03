#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate libc;
extern crate sovtoken;
extern crate rust_indy_sdk as indy;                      // lib-sdk project
extern crate rust_base58;
pub mod utils;

use indy::ErrorCode;
use libc::c_char;
use sovtoken::utils::ffi_support;
use sovtoken::utils::test::callbacks;
use std::ffi::CString;
use std::ptr;
use std::sync::mpsc::{Receiver};
use std::time::Duration;


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

    use rust_base58::ToBase58;
    let (ec_initial, receiver) = call_set_fees(&"1234567890123456".as_bytes().to_base58(), fees);
    let (ec_callback, fees_request) = receiver.recv().unwrap();

    let request_value: serde_json::value::Value = serde_json::from_str(&fees_request).unwrap();

    assert_eq!(ErrorCode::Success, ec_initial);
    assert_eq!(ErrorCode::Success, ec_callback);
    assert_eq!(&expected_operation, request_value.get("operation").unwrap());
}

#[test]
pub fn build_and_submit_set_fees() {
    sovtoken::api::sovtoken_init();
    let payment_method = sovtoken::api::PAYMENT_METHOD_NAME;
    let pc_str = utils::pool::create_pool_config();
    let pool_config = Some(pc_str.as_str());
    indy::pool::Pool::set_protocol_version(2).unwrap();

    let pool_name = utils::pool::create_pool_ledger(pool_config);
    let pool_handle = indy::pool::Pool::open_ledger(&pool_name, None).unwrap();
    let wallet = utils::wallet::Wallet::new(&pool_name);

    let (did_trustee, _) = indy::did::Did::new(wallet.handle, &json!({"seed":"000000000000000000000000Trustee1"}).to_string()).unwrap();

    let (did_1, _) = utils::did::add_new_trustee_did(wallet.handle, &did_trustee, pool_handle).unwrap();
    let (did_2, _) = utils::did::add_new_trustee_did(wallet.handle, &did_trustee, pool_handle).unwrap();
    let (did_3, _) = utils::did::add_new_trustee_did(wallet.handle, &did_trustee, pool_handle).unwrap();

    let fees = json!({
        "1": 1,
        "101": 2
    }).to_string();

    let set_fees_req = indy::payments::Payment::build_set_txn_fees_req(wallet.handle, &did_trustee, payment_method, &fees).unwrap();

    let set_fees_req = indy::ledger::Ledger::multi_sign_request(wallet.handle, &did_trustee, &set_fees_req).unwrap();
    let set_fees_req = indy::ledger::Ledger::multi_sign_request(wallet.handle, &did_1, &set_fees_req).unwrap();
    let set_fees_req = indy::ledger::Ledger::multi_sign_request(wallet.handle, &did_2, &set_fees_req).unwrap();
    let set_fees_req = indy::ledger::Ledger::multi_sign_request(wallet.handle, &did_3, &set_fees_req).unwrap();

    let result = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, &did_trustee, &set_fees_req).unwrap();
    println!("{}", result);
}
