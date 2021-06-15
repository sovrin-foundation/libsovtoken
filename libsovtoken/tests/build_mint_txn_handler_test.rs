extern crate libc;

extern crate sovtoken;
#[macro_use]
extern crate serde_derive;
extern crate indyrs as indy;                     // lib-sdk project
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate indy_sys;

use libc::c_char;
use std::ptr;
use std::ffi::CString;

use indy::future::Future;

use sovtoken::ErrorCode;
use sovtoken::utils::ffi_support::{str_from_char_ptr, c_pointer_from_str};
use sovtoken::utils::constants::txn_types::MINT_PUBLIC;
use sovtoken::utils::constants::txn_fields::OUTPUTS;
use sovtoken::logic::parsers::common::ResponseOperations;
use sovtoken::utils::json_conversion::JsonDeserialize;
use sovtoken::logic::config::output_mint_config::MintRequest;
use sovtoken::logic::request::Request;
use sovtoken::utils::results::ResultHandler;
use sovtoken::utils::test::callbacks;

mod utils;

use utils::wallet::Wallet;
use utils::parse_mint_response::ParseMintResponse;
use utils::setup::{Setup, SetupConfig};

// ***** HELPER METHODS *****
fn build_mint_req(wallet_handle: indy_sys::WalletHandle, did: Option<&str>, outputs: &str, extra: Option<&str>) -> Result<String, ErrorCode> {
    let (receiver, command_handle, cb) = callbacks::cb_ec_string();

    let did = did.map(c_pointer_from_str).unwrap_or(std::ptr::null());
    let extra = extra.map(c_pointer_from_str).unwrap_or(std::ptr::null());

    let error_code = sovtoken::api::build_mint_txn_handler(
        command_handle,
        wallet_handle,
        did,
        c_pointer_from_str(outputs),
        extra,
        cb
    );

    return ResultHandler::one(ErrorCode::from(error_code), receiver);
}

// ***** HELPER TEST DATA  *****

const COMMAND_HANDLE: i32 = 10;
static INVALID_OUTPUT_JSON: &'static str = r#"{"totally" : "Not a Number", "bobby" : "DROP ALL TABLES"}"#;
static VALID_OUTPUT_JSON: &'static str = r#"[{"recipient":"pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", "amount":10}]"#;

// ***** UNIT TESTS ****

// the build_mint_txn_handler requires a callback and this test ensures that we
// receive an error when no callback is provided
#[test]
fn errors_with_no_call_back() {
    let return_error = sovtoken::api::build_mint_txn_handler(COMMAND_HANDLE, indy_sys::WalletHandle(1), ptr::null(), ptr::null(), ptr::null(), None);
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting Callback for 'build_mint_txn_handler'");
}

// the build mint txn handler method requires an outputs_json parameter and this test ensures that 
// a error is returned when no config is provided
#[test]
fn errors_with_no_outputs_json() {
    static mut CALLBACK_CALLED: bool = false;
    extern "C" fn cb_no_json(_: i32, error_code: i32, _: *const c_char) -> i32 {
        unsafe { CALLBACK_CALLED = true; }
        assert_eq!(error_code, ErrorCode::CommonInvalidStructure as i32);
        return ErrorCode::Success as i32;
    }

    let return_error = sovtoken::api::build_mint_txn_handler(COMMAND_HANDLE, indy_sys::WalletHandle(1), ptr::null(), ptr::null(), ptr::null(), Some(cb_no_json));
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting outputs_json for 'build_mint_txn_handler'");
    unsafe { assert!(!CALLBACK_CALLED) }
}

// // the mint txn handler method requires a valid JSON format (format is described
// in build_mint_fees_handler description).  Expecting error when invalid json is inputted
#[test]
fn errors_with_invalid_outputs_json() {
    static mut CALLBACK_CALLED: bool = false;
    extern "C" fn cb_invalid_json(_: i32, error_code: i32, _: *const c_char) -> i32 {
        unsafe { CALLBACK_CALLED = true; }
        assert_eq!(error_code, ErrorCode::CommonInvalidStructure as i32);
        return ErrorCode::Success as i32;
    }

    let outputs_str = CString::new(INVALID_OUTPUT_JSON).unwrap();
    let outputs_str_ptr = outputs_str.as_ptr();
    let return_error = sovtoken::api::build_mint_txn_handler(COMMAND_HANDLE, indy_sys::WalletHandle(1), ptr::null(), outputs_str_ptr, ptr::null(), Some(cb_invalid_json));
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting Valid JSON for 'build_mint_txn_handler'");
    unsafe { assert!(!CALLBACK_CALLED) }
}

#[test]
fn valid_output_json() {
    sovtoken::api::sovtoken_init();
    static mut CALLBACK_CALLED: bool = false;
    extern "C" fn valid_output_json_cb(command_handle: i32, error_code: i32, mint_request: *const c_char) -> i32 {
        unsafe { CALLBACK_CALLED = true; }
        assert_eq!(command_handle, COMMAND_HANDLE);
        assert_eq!(error_code, ErrorCode::Success as i32);
        let mint_request_json_string = str_from_char_ptr(mint_request).unwrap();
        let mint_request_json_value: serde_json::Value = serde_json::from_str(mint_request_json_string).unwrap();
        let mint_operation = mint_request_json_value
            .get("operation")
            .unwrap();

        let expected = json!({
            "type": MINT_PUBLIC,
            OUTPUTS: [
                {
                    "address": "dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
                    "amount": 10
                }
            ]
        });
        assert_eq!(mint_operation, &expected);
        return ErrorCode::Success as i32;
    }

    let did = c_pointer_from_str("Th7MpTaRZVRYnPiabds81Y");
    let outputs_str = CString::new(VALID_OUTPUT_JSON).unwrap();
    let outputs_str_ptr = outputs_str.as_ptr();
    let return_error = sovtoken::api::build_mint_txn_handler(
        COMMAND_HANDLE,
        indy_sys::WalletHandle(1),
        did,
        outputs_str_ptr,
        ptr::null(),
        Some(valid_output_json_cb)
    );

    assert_eq!(return_error, ErrorCode::Success as i32, "Expecting Valid JSON for 'build_mint_txn_handler'");
    unsafe {
        assert!(CALLBACK_CALLED);
    }
}

#[test]
pub fn build_mint_txn_works_for_sov_fully_qualified_did() {
    let wallet = Wallet::new();

    let expected_operation = json!({
        "type": "10000",
        "outputs": [{
            "address": "dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
            "amount":10
        }],
    });

    let mint_req = build_mint_req(wallet.handle, Some("did:sov:VsKV7grR1BUE29mG2Fm2kX"), VALID_OUTPUT_JSON, None, ).unwrap();

    let request_value: serde_json::value::Value = serde_json::from_str(&mint_req).unwrap();
    assert_eq!(&expected_operation, request_value.get("operation").unwrap());
    assert_eq!("VsKV7grR1BUE29mG2Fm2kX", request_value["identifier"].as_str().unwrap());
}

#[test]
pub fn build_mint_txn_works_for_other_fully_qualified_did() {
    let wallet = Wallet::new();

    let expected_operation = json!({
        "type": "10000",
        "outputs": [{
            "address": "dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
            "amount":10
        }],
    });

    let mint_req = build_mint_req(wallet.handle, Some("did:other:VsKV7grR1BUE29mG2Fm2kX"), VALID_OUTPUT_JSON, None, ).unwrap();

    let request_value: serde_json::value::Value = serde_json::from_str(&mint_req).unwrap();
    assert_eq!(&expected_operation, request_value.get("operation").unwrap());
    assert_eq!("did:other:VsKV7grR1BUE29mG2Fm2kX", request_value["identifier"].as_str().unwrap());
}

#[test]
pub fn build_and_submit_mint_txn_works() {
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


    let output_json = json!([
        {
            "recipient": payment_addresses[0],
            "amount": 5,
        },
        {
            "recipient": payment_addresses[1],
            "amount": 10,
        },
        {
            "recipient": payment_addresses[2],
            "amount": 15,
        }
    ]).to_string();

    let mint_req = build_mint_req(
        wallet.handle,
        Some(dids[0]),
        &output_json,
        None,
    ).unwrap();

    trace!("{:?}", &mint_req);

    let mint_req = Request::<MintRequest>::multi_sign_request(
        wallet.handle,
        &mint_req,
        dids.clone()
    ).unwrap();

    trace!("{:?}", &mint_req);

    let result = indy::ledger::submit_request(pool_handle, &mint_req).wait().unwrap();
    let response = ParseMintResponse::from_json(&result).unwrap();
    assert_eq!(response.op, ResponseOperations::REPLY);
    let (utxos, next) = utils::payment::get_utxo::send_get_utxo_request(&wallet, pool_handle, &dids[0], &payment_addresses[0], None);
    assert_eq!(utxos[0].amount, 5);
    assert_eq!(utxos[0].payment_address, payment_addresses[0]);
    assert_eq!(next, None);
}

#[test]
pub fn build_and_submit_mint_txn_works_with_empty_did() {
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


    let output_json = json!([
        {
            "recipient": payment_addresses[0],
            "amount": 5,
        },
        {
            "recipient": payment_addresses[1],
            "amount": 10,
        },
        {
            "recipient": payment_addresses[2],
            "amount": 15,
        }
    ]).to_string();

    let mint_req = build_mint_req(
        wallet.handle,
        Some(&dids[0]),
        &output_json,
        None,
    ).unwrap();

    trace!("{:?}", &mint_req);

    let mint_req = Request::<MintRequest>::multi_sign_request(
        wallet.handle,
        &mint_req,
        dids.clone()
    ).unwrap();

    trace!("{:?}", &mint_req);

    let result = indy::ledger::submit_request(pool_handle, &mint_req).wait().unwrap();
    let response = ParseMintResponse::from_json(&result).unwrap();
    assert_eq!(response.op, ResponseOperations::REPLY);
    let (utxos, next) = utils::payment::get_utxo::send_get_utxo_request(&wallet, pool_handle, &dids[0], &payment_addresses[0], None);
    assert_eq!(utxos[0].amount, 5);
    assert_eq!(utxos[0].payment_address, payment_addresses[0]);
    assert!(next.is_none())
}

#[test]
pub fn build_and_submit_mint_txn_works_for_double_send_mint() {
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


    let output_json = json!([
        {
            "recipient": payment_addresses[0],
            "amount": 5,
        },
        {
            "recipient": payment_addresses[1],
            "amount": 10,
        },
        {
            "recipient": payment_addresses[2],
            "amount": 15,
        }
    ]).to_string();

    let mint_req = build_mint_req(
        wallet.handle,
        Some(dids[0]),
        &output_json,
        None
    ).unwrap();

    trace!("{:?}", &mint_req);

    let mint_req = Request::<MintRequest>::multi_sign_request(
        wallet.handle,
        &mint_req,
        dids.clone()
    ).unwrap();

    trace!("{:?}", &mint_req);

    let result = indy::ledger::submit_request(pool_handle, &mint_req).wait().unwrap();
    let response = ParseMintResponse::from_json(&result).unwrap();
    assert_eq!(response.op, ResponseOperations::REPLY);
    let result = indy::ledger::submit_request(pool_handle, &mint_req).wait().unwrap();
    let response = ParseMintResponse::from_json(&result).unwrap();
    assert_eq!(response.op, ResponseOperations::REPLY);
    let (utxos, next) = utils::payment::get_utxo::send_get_utxo_request(&wallet, pool_handle, &dids[0], &payment_addresses[0], None);
    assert_eq!(utxos.len(), 1);
    assert_eq!(utxos[0].amount, 5);
    assert_eq!(utxos[0].payment_address, payment_addresses[0]);
    assert!(next.is_none());
}

#[test]
/* Confirm 10 billion tokens can be minted */
fn mint_10_billion_tokens() {
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

    // 10 billion tokens
    let tokens = 10u64.pow(18);

    let output_json = json!([{
        "recipient": payment_addresses[0],
        "amount": tokens,
    }]).to_string();

    let mint_req = build_mint_req(
        wallet.handle,
        Some(dids[0]),
        &output_json,
        None
    ).unwrap();

    trace!("{:?}", &mint_req);

    let mint_req = Request::<MintRequest>::multi_sign_request(
        wallet.handle,
        &mint_req,
        dids.clone()
    ).unwrap();

    trace!("{:?}", &mint_req);

    let result = indy::ledger::submit_request(pool_handle, &mint_req).wait().unwrap();
    let response = ParseMintResponse::from_json(&result).unwrap();

    trace!("{:?}", &response);

    assert_eq!(response.op, ResponseOperations::REPLY);
    let (utxos, next) = utils::payment::get_utxo::send_get_utxo_request(&wallet, pool_handle, &dids[0], &payment_addresses[0], None);
    assert_eq!(utxos[0].amount, tokens);
    assert_eq!(utxos[0].payment_address, payment_addresses[0]);
    assert_eq!(next, None);
}
