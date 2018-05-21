#![warn(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate libc;
extern crate sovtoken;

#[macro_use] extern crate lazy_static;

mod indy;
mod callbacks;

use std::ptr::null;
use std::ffi::CString;

use indy::*;
use callbacks::*;
// use sovtoken::api::*;

static POOL: &str = "pool_1";
static TYPE: &str = "default";
static PAYMENT_METHOD: &str = "sov";
static WALLET: &str = "wallet_b";
static PAYMENT_CONFIG: &str = r#"{}"#;

/**
   calls sovtoken to initialize indy-sdk with libsovtoken payment methods
*/
/*fn initialize_libraries() {
    sovtoken_init();
}*/

/**
   cleans up any
*/
fn clean_up(wallet_name: &String) {

    let (receiver, command_handle, cb) = CallbackUtils::closure_to_cb_ec();

    let wallet = CString::new(wallet_name.to_string()).unwrap();

    let err = unsafe { indy_delete_wallet(command_handle, wallet.as_ptr(), null(), cb); };

    let err = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
}


fn create_wallet(pool_name: &str, wallet_name: &str, xtype: Option<&str>, config: Option<&str>, credentials: Option<&str>) {

    let pool_name = CString::new(pool_name).unwrap();
    let wallet_name = CString::new(wallet_name).unwrap();
    let xtype_str = xtype.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
    let config_str = config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
    let credentials_str = credentials.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());


    let (create_wallet_receiver, create_wallet_command_handle, create_wallet_callback) = CallbackUtils::closure_to_cb_ec();

    unsafe {
        let err =
            indy_create_wallet(create_wallet_command_handle,
                               pool_name.as_ptr(),
                               wallet_name.as_ptr(),
                               if xtype.is_some() { xtype_str.as_ptr() } else { null() },
                               if config.is_some() { config_str.as_ptr() } else { null() },
                               if credentials.is_some() { credentials_str.as_ptr() } else { null() },
                               create_wallet_callback);

        assert_eq!(ErrorCode::Success, err);
    };


    let err = create_wallet_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

}


fn open_wallet(wallet_name: &str, config: Option<&str>, credentials: Option<&str>) -> i32 {

    let wallet_name = CString::new(wallet_name).unwrap();
    let config_str = config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
    let credentials_str = credentials.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

    let (open_wallet_receiver, open_wallet_command_handle, open_wallet_callback) = CallbackUtils::closure_to_cb_ec_i32();

    unsafe {
        let err =
            indy_open_wallet(open_wallet_command_handle,
                             wallet_name.as_ptr(),
                             if config.is_some() { config_str.as_ptr() } else { null() },
                             if credentials.is_some() { credentials_str.as_ptr() } else { null() },
                             open_wallet_callback);

        assert_eq!(ErrorCode::Success, err);
    };

    let (err, wallet_handle) = open_wallet_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    return wallet_handle;
}


fn create_payment(wallet_handle: i32) -> String {

    let (receiver, command_handle, cb) = CallbackUtils::closure_to_cb_ec_string();

    let payment_method = CString::new(PAYMENT_METHOD.to_string()).unwrap();
    let config = CString::new(PAYMENT_CONFIG.to_string()).unwrap();

    unsafe {
        let err = indy_create_payment_address(command_handle, wallet_handle, payment_method.as_ptr(), config.as_ptr(), cb);
        assert_eq!(ErrorCode::Success, err);
    };

    let (result, payment_address) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

    return payment_address;
}


/**
   Entry point for the create payment address demo.  It will setup the environment, create the payment address
   and prove it was created by doing something.  preferably with a wow factor and maybe some cool colors
*/
fn main() {


    println!("initializing libsovtoken -> indy-sdk");
    // initialize_libraries();

    println!("Setting up an wallet called '{}'", WALLET);
    create_wallet(POOL, WALLET, Some(TYPE), None, None);
    println!("opening wallet.");
    let wallet_handle: i32 = open_wallet(WALLET, None, None);


    println!("creating a payment");
    let payment_address:String = create_payment(wallet_handle);

    println!();
    println!("received a payment address of '{}'", payment_address);

    // final step
    println!("demo complete, running cleanup");
    clean_up(&WALLET.to_string());
}
