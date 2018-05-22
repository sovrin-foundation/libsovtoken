#![warn(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate libc;

#[macro_use] extern crate lazy_static;

mod indy;
mod callbacks;

use std::ptr::null;
use std::ffi::CString;

use indy::*;
use callbacks::*;


/**
   calls sovtoken to initialize indy-sdk with libsovtoken payment methods
*/
fn initialize_libraries() {
    // sovtoken_init();
}

/**
   cleans up any
*/
fn clean_up(wallet_name: &String) {

    let (receiver, command_handle, cb) = CallbackUtils::closure_to_cb_ec();

    let wallet = CString::new(wallet_name.to_string()).unwrap();

    let err = unsafe { indy_delete_wallet(command_handle, wallet.as_ptr(), null(), cb); };

    let err = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
}

/**
    creates wallet for test
*/
fn create_wallet(pool_name: &String, wallet_name: &String) {

    let pool_name = CString::new(pool_name.to_string()).unwrap();
    let wallet_name = CString::new(wallet_name.to_string()).unwrap();

    let (create_wallet_receiver, create_wallet_command_handle, create_wallet_callback) = CallbackUtils::closure_to_cb_ec();

    unsafe {
        let err =
            indy_create_wallet(create_wallet_command_handle,
                               pool_name.as_ptr(),
                               wallet_name.as_ptr(),
                               null(),
                               null(),
                               null(),
                               create_wallet_callback);

        assert_eq!(ErrorCode::Success, err);
    };

    let err = create_wallet_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

}

/**
   opens wallet
*/
fn open_wallet(wallet_name: &String) -> i32 {

    let wallet_name = CString::new(wallet_name.to_string()).unwrap();

    let (open_wallet_receiver, open_wallet_command_handle, open_wallet_callback) = CallbackUtils::closure_to_cb_ec_i32();

    unsafe {
        let err =
            indy_open_wallet(open_wallet_command_handle,
                             wallet_name.as_ptr(),
                             null(),
                             null(),
                             open_wallet_callback);

        assert_eq!(ErrorCode::Success, err);
    };

    let (err, wallet_handle) = open_wallet_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    return wallet_handle;
}


/**
   calls indy_create_payment_address which is expected to call libsovtoken::create_payment_address_handler and return
   a payment address looking like pay:sov:{address}{checksum}
*/
fn create_payment(wallet_handle: i32) -> String {

    let (receiver, command_handle, cb) = CallbackUtils::closure_to_cb_ec_string();

    let payment_method = CString::new("pay::sov".to_string()).unwrap();
    let config = CString::new(r#"{}"#.to_string()).unwrap();

    unsafe {
        let err = indy_create_payment_address(command_handle, wallet_handle, payment_method.as_ptr(), config.as_ptr(), cb);
        assert_eq!(ErrorCode::Success, err);
    };

    let (result, payment_address) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, result);

    return payment_address;
}


/**
   Entry point for the create payment address demo.  It will setup the environment, create the payment address
   and prove it was created by doing something.  preferably with a wow factor and maybe some cool colors
*/
fn main() {

    println!();
    println!();
    println!("----------------------------------------------------");
    println!("create payment address demo starts");
    println!();

    let POOL: String = "pool_1".to_string();
    let WALLET: String = "payment_demo".to_string();

    let panic_result = std::panic::catch_unwind( ||
    {
        println!("1 => initializing libsovtoken -> indy-sdk");
        initialize_libraries();

        println!("2 => Setting up an wallet called '{}'", WALLET);
        create_wallet(&POOL, &WALLET);
        println!("     ....opening wallet.");
        let wallet_handle: i32 = open_wallet(&WALLET);


        println!("3 => creating a payment");
        let payment_address: String = create_payment(wallet_handle);

        println!();
        println!("     received a payment address of '{}'", payment_address);
    });

    if false == panic_result.is_err() {
        println!("4 => payment complete, running cleanup");
    } else {
        println!("4 => running cleanup after error");
    }

    clean_up(&WALLET);

    println!();
    println!("demo finished....");
    println!("----------------------------------------------------");
}
