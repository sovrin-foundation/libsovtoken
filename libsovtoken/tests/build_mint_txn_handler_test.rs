extern crate libc;

extern crate sovtoken;
#[macro_use]
extern crate serde_derive;
extern crate rust_indy_sdk as indy;                      // lib-sdk project
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
static VALID_OUTPUT_JSON: &'static str = r#"[{"paymentAddress":"pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", "amount":10}]"#;

// ***** UNIT TESTS ****

// the build_mint_txn_handler requires a callback and this test ensures that we
// receive an error when no callback is provided
#[test]
fn errors_with_no_call_back() {
    let return_error = sovtoken::api::build_mint_txn_handler(COMMAND_HANDLE, 1, ptr::null(), ptr::null(), None);
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

    let return_error = sovtoken::api::build_mint_txn_handler(COMMAND_HANDLE, 1, ptr::null(), ptr::null(), Some(cb_no_json));
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
    let return_error = sovtoken::api::build_mint_txn_handler(COMMAND_HANDLE, 1, ptr::null(), outputs_str_ptr, Some(cb_invalid_json));
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
            OUTPUTS: [["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",10]]
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
        did,
        outputs_str,
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
        OUTPUTS: [["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",10]]
    });


    assert_eq!("sov", payment_method);
    assert_eq!(mint_operation, &expected);
}

#[test]
pub fn build_and_submit_mint_txn_works() {
    sovtoken::api::sovtoken_init();
    let pc_str = utils::pool::create_pool_config();
    let pool_config = Some(pc_str.as_str());
    indy::pool::Pool::set_protocol_version(2).unwrap();

    let pool_name = utils::pool::create_pool_ledger(pool_config);
    let pool_handle = indy::pool::Pool::open_ledger(&pool_name, None).unwrap();
    let wallet = utils::wallet::Wallet::new();

    let trustees = utils::did::initial_trustees(4, wallet.handle, pool_handle).unwrap();
    let dids = utils::did::did_str_from_trustees(&trustees);

    let payment_addresses = utils::payment::address::generate_n(&wallet, 3);

    let output_json = json!([
        {
            "paymentAddress": payment_addresses[0],
            "amount": 5,
            "extra": "pa1",
        },
        {
            "paymentAddress": payment_addresses[1],
            "amount": 10,
            "extra": "pa2",
        },
        {
            "paymentAddress": payment_addresses[2],
            "amount": 15,
            "extra": "pa3",
        }
    ]).to_string();

    let (mint_req, _) = indy::payments::Payment::build_mint_req(
        wallet.handle,
        dids[0],
        &output_json
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
    let (_, amount, payment_address) = utils::get_utxo::get_first_utxo_for_payment_address(wallet.handle, pool_handle, &dids[0], &payment_addresses[0]);
    assert_eq!(amount, 5);
    assert_eq!(payment_address, payment_addresses[0]);
}