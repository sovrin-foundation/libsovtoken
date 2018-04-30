
#[allow(unused_variables)]
#[allow(dead_code)]

extern crate sovtoken;
extern crate indy;                      // lib-sdk project
use std::ptr;
use indy::api::ErrorCode;



// the create payment requires a callback and this test ensures we have 
// recieve error when no callback is provided
#[test]
fn errors_with_no_callback () {
    let return_error = sovtoken::api::create_payment_address_handler(10, ptr::null(), None);
    assert!(return_error == ErrorCode::CommonInvalidParam3, "Expecting Callback for 'create_payment_address_handler'"); 
}

