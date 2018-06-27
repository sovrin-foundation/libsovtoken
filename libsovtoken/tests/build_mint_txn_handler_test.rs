extern crate libc;

extern crate sovtoken;
extern crate rust_indy_sdk as indy;                      // lib-sdk project
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

use indy::ErrorCode;

use libc::c_char;
use std::ptr;
use std::ffi::CString;
use sovtoken::utils::ffi_support::{str_from_char_ptr, c_pointer_from_str};
use std::sync::mpsc::channel;
use indy::utils::results::ResultHandler;
use std::time::Duration;

mod utils;

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
            "type": "10000",
            "outputs": [["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",10]]
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
    let wallet = utils::wallet::Wallet::new();
    let outputs_str = VALID_OUTPUT_JSON;
    let (sender, receiver) = channel();

    let cb = move |ec, req, payment_method| {
        sender.send((ec, req, payment_method));
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
            "type": "10000",
            "outputs": [["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",10]]
        });
    assert_eq!(mint_operation, &expected);
}

