extern crate libc;

extern crate sovtoken;
extern crate indy;                      // lib-sdk project

use libc::c_char;
use std::ptr;
use std::ffi::CString;
use sovtoken::utils::ffi_support::str_from_char_ptr;

use indy::api::ErrorCode;
// ***** HELPER METHODS *****
extern "C" fn empty_mint_request_cb(_command_handle: i32, _err: ErrorCode, _mint_request: *const c_char) { }

// ***** HELPER TEST DATA  *****

const COMMAND_HANDLE:i32 = 10;
static INVALID_OUTPUT_JSON: &'static str = r#"{"totally" : "Not a Number", "bobby" : "DROP ALL TABLES"}"#;
static VALID_OUTPUT_JSON: &'static str = r#"{"outputs":[["AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",10]]}"#;
const EMPTY_CB : Option<extern fn(command_handle: i32, err: ErrorCode, payment_address: *const c_char)> = Some(empty_mint_request_cb);

// ***** UNIT TESTS *****

// the build_mint_txn_handler requires a callback and this test ensures that we
// receive an error when no callback is provided
#[test]
fn errors_with_no_call_back() {
    let return_error = sovtoken::api::build_mint_txn_handler(COMMAND_HANDLE, ptr::null(), None);
    assert_eq!(return_error, ErrorCode::CommonInvalidParam3, "Expecting Callback for 'build_mint_txn_handler'"); 
}

// the build mint txn handler method requires an outputs_json parameter and this test ensures that 
// a error is returned when no config is provided
#[test]
fn errors_with_no_outputs_json() {
    let return_error = sovtoken::api::build_mint_txn_handler(COMMAND_HANDLE, ptr::null(), EMPTY_CB);
    assert_eq!(return_error, ErrorCode::CommonInvalidParam2, "Expecting outputs_json for 'build_mint_txn_handler'");
}

// // the mint txn handler method requires a valid JSON format (format is described
// in build_mint_txn_handler description).  Expecting error when invalid json is inputted
#[test]
fn errors_with_invalid_outputs_json() {
    let outputs_str = CString::new(INVALID_OUTPUT_JSON).unwrap();
    let outputs_str_ptr = outputs_str.as_ptr();
    let return_error = sovtoken::api::build_mint_txn_handler(COMMAND_HANDLE, outputs_str_ptr, EMPTY_CB);
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure, "Expecting Valid JSON for 'build_mint_txn_handler'");
}

#[test]
fn valid_output_json() {
    extern fn valid_output_json_cb(command_handle: i32, error_code: ErrorCode, mint_request: *const c_char) {
        assert_eq!(command_handle, COMMAND_HANDLE);
        assert_eq!(error_code, ErrorCode::Success);
        let mint_request_as_json = str_from_char_ptr(mint_request).unwrap();
        let expected = r#"{"type":"1001","outputs":[["AesjahdahudgaiuNotARealAKeyygigfuigraiudgfasfhja",10]],"signatures":[]}"#;
        assert_eq!(mint_request_as_json, expected);
    }

    let outputs_str = CString::new(VALID_OUTPUT_JSON).unwrap();
    let outputs_str_ptr = outputs_str.as_ptr();
    let return_error = sovtoken::api::build_mint_txn_handler(COMMAND_HANDLE, outputs_str_ptr, Some(valid_output_json_cb));
    assert_eq!(return_error, ErrorCode::Success, "Expecting Valid JSON for 'build_mint_txn_handler'");
}

