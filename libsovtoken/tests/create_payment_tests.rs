
#[allow(unused_variables)]
#[allow(dead_code)]

extern crate libc;

extern crate sovtoken;
extern crate indy;                      // lib-sdk project

use libc::c_char;
use std::ptr;

use indy::api::ErrorCode;


//____________________HELPER METHODS____________________//
extern "C" fn empty_create_payment_callback(command_handle_: i32, err: ErrorCode, payment_address: *const c_char){

}


//____________________UNIT TESTS____________________//

// the create payment requires a callback and this test ensures we have 
// recieve error when no callback is provided
#[test]
fn errors_with_no_callback () {
    let return_error = sovtoken::api::create_payment_address_handler(10, ptr::null(), None);
    assert!(return_error == ErrorCode::CommonInvalidParam3, "Expecting Callback for 'create_payment_address_handler'"); 
}


// the create payment method requires a config parameter and this test ensures that 
// a error is recieved when no congifg is provided 
#[test]
fn errors_with_no_config() {
    let cb : Option<extern fn(command_handle_: i32, err: ErrorCode, payment_address: *const c_char)> = Some(empty_create_payment_callback);
    let return_error = sovtoken::api::create_payment_address_handler(10, ptr::null(), cb);
    assert!(return_error == ErrorCode::CommonInvalidParam2, "Expecting Config for 'create_payment_address_handler'");

}

// the create payment method requires a valid JSON format described 
// in create_payment_address_handler description
#[test]
fn errors_with_invalid_json() {
    
    let cb : Option<extern fn(command_handle_: i32, err: ErrorCode, payment_address: *const c_char)> = Some(empty_create_payment_callback);
    let return_error = sovtoken::api::create_payment_address_handler(10, ptr::null(), cb);
    assert!(return_error == ErrorCode::CommonInvalidStructure, "Expecting Valid JSON");
}