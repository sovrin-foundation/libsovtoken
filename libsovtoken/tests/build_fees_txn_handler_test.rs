#[allow(unused_variables)]
#[allow(dead_code)]

extern crate libc;

extern crate sovtoken;
extern crate indy;                      // lib-sdk project

use libc::c_char;
use std::ptr;
use std::ffi::CString;

use indy::api::ErrorCode;

// ***** HELPER METHODS *****
extern "C" fn empty_create_payment_callback(command_handle_: i32, err: ErrorCode, mint_req_json: *const c_char) { }

// ***** HELPER TEST DATA  *****

const COMMAND_HANDLE:i32 = 10;
static INVALID_OUTPUT_JSON: &'static str = r#"{"totally" : "Not a Number", "bobby" : "DROP ALL TABLES"}"#;
const cb : Option<extern fn(command_handle_: i32, err: ErrorCode, mint_req_json: *const c_char)> = Some(empty_create_payment_callback);

// ***** UNIT TESTS ****

// the build_fees_txn_handler requires a callback and this test ensures that we
// receive an error when no callback is provided
#[test]
fn errors_with_no_call_back() {
    let return_error = sovtoken::api::build_fees_txn_handler(COMMAND_HANDLE, ptr::null(), None);
    assert_eq!(return_error, ErrorCode::CommonInvalidParam3, "Expecting Callback for 'build_fees_txn_handler'"); 
}

// the build fees txn handler method requires an outputs_json parameter and this test ensures that 
// a error is returned when no config is provided
#[test]
fn errors_with_no_fees_json() {
    let return_error = sovtoken::api::build_fees_txn_handler(COMMAND_HANDLE, ptr::null(), cb);
    assert_eq!(return_error, ErrorCode::CommonInvalidParam2, "Expecting outputs_json for 'build_fees_txn_handler'");
}

#[test]
fn errors_with_invalid_fees_json() {
    let fees_str = CString::new(INVALID_OUTPUT_JSON).unwrap();
    let fees_str_ptr = fees_str.as_ptr();
    let return_error = sovtoken::api::build_fees_txn_handler(COMMAND_HANDLE, fees_str_ptr, cb);
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure, "Expecting Valid JSON for 'build_fees_txn_handler'");
}