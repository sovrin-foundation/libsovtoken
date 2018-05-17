#![warn(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#[allow(unused_imports)]

extern crate libc;

extern crate sovtoken;
extern crate indy;                      // lib-sdk project

use indy::api::ErrorCode;

use libc::c_char;
use std::ptr;
use std::ffi::CString;

// ***** HELPER METHODS *****
extern "C" fn empty_create_payment_callback(command_handle_: i32, err: ErrorCode, mint_req_json: *const c_char) -> ErrorCode {
    return ErrorCode::Success;
}

// ***** HELPER TEST DATA  *****

const COMMAND_HANDLE:i32 = 10;
const WALLET_ID:i32 = 10;
static INVALID_OUTPUT_JSON: &'static str = r#"{"totally" : "Not a Number", "bobby" : "DROP ALL TABLES"}"#;
const cb : Option<extern fn(command_handle_: i32, err: ErrorCode, mint_req_json: *const c_char) -> ErrorCode > = Some(empty_create_payment_callback);

// ***** UNIT TESTS ****

// the build_fees_txn_handler requires a callback and this test ensures that we
// receive an error when no callback is provided
#[test]
fn errors_with_no_call_back() {
    let return_error = sovtoken::api::build_get_utxo_request_handler(COMMAND_HANDLE,
                                                                     WALLET_ID,
                                                                     ptr::null(),
                                                                     ptr::null(),
                                                                     None);

    assert_eq!(return_error, ErrorCode::CommonInvalidParam3, "Expecting Callback for 'build_fees_txn_handler'");
}
