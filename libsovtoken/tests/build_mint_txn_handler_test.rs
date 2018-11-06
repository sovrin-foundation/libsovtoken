extern crate libc;

extern crate sovtoken;
#[macro_use]
extern crate serde_derive;
extern crate indy;                      // lib-sdk project
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;

mod utils;

use libc::c_char;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::ptr;
use std::ffi::CString;

use indy::utils::results::ResultHandler;
use indy::ErrorCode;

use utils::wallet::Wallet;
use utils::parse_mint_response::ParseMintResponse;
use utils::setup::{Setup, SetupConfig};

use sovtoken::utils::ffi_support::{str_from_char_ptr, c_pointer_from_str};
use sovtoken::utils::constants::txn_types::MINT_PUBLIC;
use sovtoken::utils::constants::txn_fields::OUTPUTS;
use sovtoken::logic::parsers::common::ResponseOperations;
use sovtoken::utils::json_conversion::JsonDeserialize;
use sovtoken::logic::config::output_mint_config::MintRequest;
use sovtoken::logic::request::Request;



// ***** HELPER METHODS *****

// ***** HELPER TEST DATA  *****

const COMMAND_HANDLE:i32 = 10;
static INVALID_OUTPUT_JSON: &'static str = r#"{"totally" : "Not a Number", "bobby" : "DROP ALL TABLES"}"#;
static VALID_OUTPUT_JSON: &'static str = r#"[{"recipient":"pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", "amount":10}]"#;

// ***** UNIT TESTS ****

// the build_mint_txn_handler requires a callback and this test ensures that we
// receive an error when no callback is provided
#[test]
fn errors_with_no_call_back() {
    let return_error = sovtoken::api::build_mint_txn_handler(COMMAND_HANDLE, 1, ptr::null(), ptr::null(), ptr::null(), None);
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

    let return_error = sovtoken::api::build_mint_txn_handler(COMMAND_HANDLE, 1, ptr::null(), ptr::null(), ptr::null(), Some(cb_no_json));
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting outputs_json for 'build_mint_txn_handler'");
    unsafe { assert!(! CALLBACK_CALLED) }
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
    let return_error = sovtoken::api::build_mint_txn_handler(COMMAND_HANDLE, 1, ptr::null(), outputs_str_ptr, ptr::null(), Some(cb_invalid_json));
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure as i32, "Expecting Valid JSON for 'build_mint_txn_handler'");
    unsafe { assert!(! CALLBACK_CALLED) }
}

#[test]
fn  valid_output_json() {
    sovtoken::api::sovtoken_init();
    static mut CALLBACK_CALLED: bool = false;
    extern "C" fn valid_output_json_cb(command_handle: i32, error_code: i32, mint_request: *const c_char) -> i32 {
        unsafe { CALLBACK_CALLED = true; }
        assert_eq!(command_handle, COMMAND_HANDLE);
        assert_eq!(error_code, ErrorCode::Success as i32);
        let mint_request_json_string = str_from_char_ptr(mint_request).unwrap();
        let mint_request_json_value : serde_json::Value = serde_json::from_str(mint_request_json_string).unwrap();
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
        1,
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
fn valid_output_json_from_libindy() {
    sovtoken::api::sovtoken_init();
    let did = "Th7MpTaRZVRYnPiabds81Y";
    let wallet = Wallet::new();
    let outputs_str = VALID_OUTPUT_JSON;
    let (sender, receiver) = channel();

    let cb = move |ec, req, payment_method| {
        sender.send((ec, req, payment_method)).unwrap();
    };

    let return_error = indy::payments::Payment::build_mint_req_async(
        wallet.handle,
        Some(did),
        outputs_str,
        None,
        cb
    );

    assert_eq!(return_error, ErrorCode::Success, "Expecting Valid JSON for 'build_mint_txn_handler'");

    let (req, payment_method) = ResultHandler::two_timeout(return_error, receiver, Duration::from_secs(5)).unwrap();

    let mint_request_json_value : serde_json::Value = serde_json::from_str(&req).unwrap();
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


    assert_eq!("sov", payment_method);
    assert_eq!(mint_operation, &expected);
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

    let (mint_req, _) = indy::payments::Payment::build_mint_req(
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

    let result = indy::ledger::Ledger::submit_request(pool_handle, &mint_req).unwrap();
    let response = ParseMintResponse::from_json(&result).unwrap();
    assert_eq!(response.op, ResponseOperations::REPLY);
    let utxos = utils::payment::get_utxo::send_get_utxo_request(&wallet, pool_handle, &dids[0], &payment_addresses[0]);
    assert_eq!(utxos[0].amount, 5);
    assert_eq!(utxos[0].payment_address, payment_addresses[0]);
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

    let (mint_req, _) = indy::payments::Payment::build_mint_req(
        wallet.handle,
        None,
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

    let result = indy::ledger::Ledger::submit_request(pool_handle, &mint_req).unwrap();
    let response = ParseMintResponse::from_json(&result).unwrap();
    assert_eq!(response.op, ResponseOperations::REPLY);
    let utxos = utils::payment::get_utxo::send_get_utxo_request(&wallet, pool_handle, &dids[0], &payment_addresses[0]);
    assert_eq!(utxos[0].amount, 5);
    assert_eq!(utxos[0].payment_address, payment_addresses[0]);
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

    let (mint_req, _) = indy::payments::Payment::build_mint_req(
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

    let result = indy::ledger::Ledger::submit_request(pool_handle, &mint_req).unwrap();
    let response = ParseMintResponse::from_json(&result).unwrap();
    assert_eq!(response.op, ResponseOperations::REPLY);
    let result = indy::ledger::Ledger::submit_request(pool_handle, &mint_req).unwrap();
    let response = ParseMintResponse::from_json(&result).unwrap();
    assert_eq!(response.op, ResponseOperations::REPLY);
    let utxos = utils::payment::get_utxo::send_get_utxo_request(&wallet, pool_handle, &dids[0], &payment_addresses[0]);
    assert_eq!(utxos.len(), 1);
    assert_eq!(utxos[0].amount, 5);
    assert_eq!(utxos[0].payment_address, payment_addresses[0]);
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

    let (mint_req, _) = indy::payments::Payment::build_mint_req(
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

    let result = indy::ledger::Ledger::submit_request(pool_handle, &mint_req).unwrap();
    let response = ParseMintResponse::from_json(&result).unwrap();

    trace!("{:?}", &response);

    assert_eq!(response.op, ResponseOperations::REPLY);
    let utxos = utils::payment::get_utxo::send_get_utxo_request(&wallet, pool_handle, &dids[0], &payment_addresses[0]);
    assert_eq!(utxos[0].amount, tokens);
    assert_eq!(utxos[0].payment_address, payment_addresses[0]);
}
