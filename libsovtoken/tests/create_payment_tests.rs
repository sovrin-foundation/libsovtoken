
#[allow(unused_variables)]
#[allow(dead_code)]

extern crate libc;

extern crate sovtoken;
extern crate indy;                      // lib-sdk project


use libc::c_char;
use std::ptr;
use std::ffi::CString;


use indy::api::ErrorCode;

//____________________HELPER TEST DATA____________________//
const COMMAND_HANDLE: i32 = 10;
static INVALID_CONFIG_JSON: &'static str = r#"{ "horrible" : "only on tuedays"}"#;


//____________________HELPER METHODS____________________//
extern "C" fn empty_create_payment_callback(command_handle_: i32, err: ErrorCode, payment_address: *const c_char) {

}


//____________________UNIT TESTS____________________//

// the create payment requires a callback and this test ensures we have 
// recieve error when no callback is provided
#[test]
fn errors_with_no_callback () {
    let return_error = sovtoken::api::create_payment_address_handler(COMMAND_HANDLE, ptr::null(), None);
    assert!(return_error == ErrorCode::CommonInvalidParam3, "Expecting Callback for 'create_payment_address_handler'"); 
}


// the create payment method requires a config parameter and this test ensures that 
// a error is returned when no config is provided
#[test]
fn errors_with_no_config() {
    let cb : Option<extern fn(command_handle_: i32, err: ErrorCode, payment_address: *const c_char)> = Some(empty_create_payment_callback);
    let return_error = sovtoken::api::create_payment_address_handler(COMMAND_HANDLE, ptr::null(), cb);
    assert!(return_error == ErrorCode::CommonInvalidParam2, "Expecting Config for 'create_payment_address_handler'");

}

// the create payment method requires a valid JSON format (format is described
// in create_payment_address_handler description).  Expecting error when invalid json is inputted
#[test]
fn errors_with_invalid_json() {

    let config_str = CString::new(INVALID_CONFIG_JSON).unwrap();
    let config_str_ptr = config_str.as_ptr();

    let cb : Option<extern fn(command_handle_: i32, err: ErrorCode, payment_address: *const c_char)> = Some(empty_create_payment_callback);
    let return_error = sovtoken::api::create_payment_address_handler(COMMAND_HANDLE, config_str_ptr, cb);

    assert!(return_error == ErrorCode::CommonInvalidStructure, "Expecting Valid JSON");
}