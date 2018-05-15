#![warn(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


extern crate indy;
extern crate sovtoken;

use std::ptr::null;
use std::ffi::CString;

use indy::api::ErrorCode;
use indy::api::wallet::*;
use sovtoken::api::*;
use sovtoken::utils::callbacks::{CallbackUtils, TimeoutUtils};


static POOL_NAME: &str = "pool_1";
static XTYPE: &str = "default";

fn initialize_libraries() {
    sovtoken_init();
}

fn setup_wallet(wallet_name: &String) {

    let (create_wallet_receiver, create_wallet_command_handle, create_wallet_callback) = CallbackUtils::closure_to_cb_ec();
    let err =
        indy_create_wallet(create_wallet_command_handle,
                           CString::new(POOL_NAME.to_string()).unwrap().as_ptr(),
                           CString::new(wallet_name.to_string()).unwrap().as_ptr(),
                           CString::new(XTYPE.to_string()).unwrap().as_ptr(),
                           null(),
                           null(),
                           create_wallet_callback);


    assert_eq!(ErrorCode::Success, err);
    let err = create_wallet_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

}

fn open_wallet(wallet_name: &String) -> i32 {

    let (open_wallet_receiver, open_wallet_command_handle, open_wallet_callback) = CallbackUtils::closure_to_cb_ec_i32();

    let err =
        indy_open_wallet(open_wallet_command_handle,
                         CString::new(wallet_name.to_string()).unwrap().as_ptr(),
                         null(),
                         null(),
                         open_wallet_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, wallet_handle) = open_wallet_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    return wallet_handle;
}


fn main() {

    println!("initializing libraries");
    initialize_libraries();

    let wallet_name = "payment_test_wallet_123".to_string();

    println!("Setting up an wallet called {}....", wallet_name);
    setup_wallet(&wallet_name);
    let wallet_handle: i32 = open_wallet(&wallet_name);

}
