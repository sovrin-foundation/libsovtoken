#![warn(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate ansi_term;
extern crate libc;

#[macro_use] extern crate lazy_static;

mod indy;
mod callbacks;
mod libsovtoken;

use std::ptr::null;
use std::ffi::CString;

use ansi_term::*;
use indy::*;
use libsovtoken::*;
use callbacks::*;


/**
   calls sovtoken to initialize indy-sdk with libsovtoken payment methods
*/
fn initialize_libraries() {
    // unsafe { sovtoken_init(); };
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


fn get_payment_addresses(wallet_handle: i32) -> String {

    let (receiver, command_handle, cb) = CallbackUtils::closure_to_cb_ec_string();

    unsafe {
        let err = indy_list_payment_addresses(command_handle, wallet_handle, cb);
        assert_eq!(ErrorCode::Success, err);
    };

    let (result, addresses_json) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, result);

    return addresses_json;
}

/**
   Entry point for the create payment address demo.  It will setup the environment, create the payment address
   and prove it was created by by calling indysdk::indy_list_payment_addresses.
*/
fn main() {

    println!();
    println!();
    println!("{}", Color::Blue.paint("----------------------------------------------------"));
    println!("{}", Color::Blue.paint("create payment address demo starts"));
    println!();

    let pool_name: String = "pool_1".to_string();
    let wallet_name: String = "payment_demo".to_string();

    let panic_result = std::panic::catch_unwind( ||
    {
        println!();
        println!("{}{}", Color::Cyan.paint("1"), " => initializing libsovtoken -> indy-sdk");
        initialize_libraries();

        println!();
        println!("{}{}'{}'", Color::Cyan.paint("2"), " => Setting up an wallet called ", wallet_name);
        create_wallet(&pool_name, &wallet_name);
        println!("     ....and opening wallet.");
        let wallet_handle: i32 = open_wallet(&wallet_name);

        println!();
        println!("{}{}", Color::Cyan.paint("3"), " => getting payment addresses BEFORE create payment");
        let addresses_json = get_payment_addresses(wallet_handle);
        println!("     ....received list of addresses");
        println!("     {}", Color::Yellow.paint(addresses_json));

        println!();
        println!("{}{}", Color::Cyan.paint("4"), " => creating a payment");
        let payment_address: String = create_payment(wallet_handle);
        println!("     ....received a payment address of '{}'", payment_address);

        println!();
        println!("{}{}", Color::Cyan.paint("5"), " => getting payment addresses");
        let addresses_json = get_payment_addresses(wallet_handle);
        println!("     ....received list of addresses");
        println!("     {}", Color::Yellow.paint(addresses_json));
    });

    println!();
    if false == panic_result.is_err() {
        println!("{}", Color::Green.paint("6 => create payment address success, running cleanup"));
    } else {
        println!("{}", Color::Red.on(Color::White).paint("6 => running cleanup after error"));
    }

    clean_up(&wallet_name);

    println!();
    println!("{}", Color::Blue.paint("demo finished...."));
    println!("{}", Color::Blue.paint("----------------------------------------------------"));
    println!();
}
