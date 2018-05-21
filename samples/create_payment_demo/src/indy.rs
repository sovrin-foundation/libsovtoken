//! indy function wrappers

use libc::c_char;

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(i32)]
pub enum ErrorCode {
    Success = 0,
}


extern "C" {
    pub fn indy_delete_wallet(command_handle : i32,
                              wallet_name : *const c_char,
                              credentials: *const c_char,
                              cb: Option<extern fn(xcommand_handle: i32,
                                                      err: ErrorCode)>) -> ErrorCode;


    pub fn indy_create_wallet(command_handle: i32,
                                 pool_name: *const c_char,
                                 name: *const c_char,
                                 xtype: *const c_char,
                                 config: *const c_char,
                                 credentials: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32,
                                                      err: ErrorCode)>) -> ErrorCode;

    pub fn indy_open_wallet(command_handle: i32,
                               name: *const c_char,
                               runtime_config: *const c_char,
                               credentials: *const c_char,
                               cb: Option<extern fn(xcommand_handle: i32,
                                                    err: ErrorCode,
                                                    handle: i32)>) -> ErrorCode;


    pub fn indy_create_payment_address(command_handle: i32,
                                          wallet_handle: i32,
                                          payment_method: *const c_char,
                                          config: *const c_char,
                                          cb: Option<extern fn(xcommand_handle: i32,
                                                               err: ErrorCode,
                                                               payment_address: *const c_char)>) -> ErrorCode;
}