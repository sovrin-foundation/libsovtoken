//! contains helper functions for creating and executing the callbacks required
//! to use indy-sdk API methods.   Copied from master/libindy/tests/utils/callback.rs

use libc::c_char;
use std::ffi::CStr;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::sync::Mutex;
use std::slice;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

use indy::api::ErrorCode;

lazy_static! {
    static ref COMMAND_HANDLE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
}

lazy_static! {
    static ref CLOSURE_CB_MAP: Mutex<HashMap<i32, i32>> = Default::default();
}

/**
    provides callback wrappers and handling for all of the different types of callback
    signatures used in INDY-SDK API functions.
*/
pub struct CallbackUtils {}

impl CallbackUtils {
    /**
       cb => Option<extern fn(command_handle: i32, err: ErrorCode)>
    */
    pub fn closure_to_cb_ec() -> (Receiver<ErrorCode>, i32,
                                   Option<extern fn(command_handle: i32,
                                                    err: ErrorCode)>) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
        }

        let closure = Box::new(move |err| {
            sender.send(err).unwrap();
        });

        extern "C" fn _callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    /**
       cb => Option<extern fn(command_handle: i32, err: ErrorCode, c_i32: i32)>
    */
    pub fn closure_to_cb_ec_i32() -> (Receiver<(ErrorCode, i32)>, i32,
                                       Option<extern fn(command_handle: i32, err: ErrorCode,
                                                        c_i32: i32)>) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, i32) + Send>>> = Default::default();
        }

        let closure = Box::new(move |err, val| {
            sender.send((err, val)).unwrap();
        });

        extern "C" fn _callback(command_handle: i32, err: ErrorCode, c_i32: i32) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err, c_i32)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    /**
       cb => Option<extern fn(command_handle: i32, err: ErrorCode, valid: bool)>
    */
    pub fn closure_to_cb_ec_bool() -> (Receiver<(ErrorCode, bool)>, i32,
                                        Option<extern fn(command_handle: i32, err: ErrorCode,
                                                         valid: bool)>) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(ErrorCode, bool) + Send>>> = Default::default();
        }

        let closure = Box::new(move |err, val| {
            sender.send((err, val)).unwrap();
        });

        extern "C" fn _callback(command_handle: i32, err: ErrorCode, valid: bool) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err, valid)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    /**
       cb => Option<extern fn(command_handle: i32, err: ErrorCode, c_str: *const c_char)>
    */
    pub fn closure_to_cb_ec_string() -> (Receiver<(ErrorCode, String)>, i32,
                                          Option<extern fn(command_handle: i32,
                                                           err: ErrorCode,
                                                           c_str: *const c_char)>) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
        }

        let closure = Box::new(move |err, val| {
            sender.send((err, val)).unwrap();
        });

        extern "C" fn _callback(command_handle: i32, err: ErrorCode, c_str: *const c_char) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let metadata = unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() };
            cb(err, metadata)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    /**
       cb => Option<extern fn(command_handle: i32, err: ErrorCode, str1: *const c_char, str2: *const c_char)>
    */
    pub fn closure_to_cb_ec_string_string() -> (Receiver<(ErrorCode, String, String)>, i32,
                                                 Option<extern fn(command_handle: i32,
                                                                  err: ErrorCode,
                                                                  str1: *const c_char,
                                                                  str2: *const c_char)>) {
        let (sender, receiver) = channel();

        lazy_static! {
                static ref CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String, String) + Send > >> = Default::default();
        }

        let closure = Box::new(move |err, val1, val2| {
            sender.send((err, val1, val2)).unwrap();
        });

        extern "C" fn _callback(command_handle: i32, err: ErrorCode, str1: *const c_char, str2: *const c_char) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let str1 = unsafe { CStr::from_ptr(str1).to_str().unwrap().to_string() };
            let str2 = unsafe { CStr::from_ptr(str2).to_str().unwrap().to_string() };
            cb(err, str1, str2)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    /**
       cb => Option<extern fn(command_handle: i32, err: ErrorCode, str1: *const c_char, str2: *const c_char)>
    */
    pub fn closure_to_cb_ec_string_opt_string() -> (Receiver<(ErrorCode, String, Option<String>)>, i32,
                                                     Option<extern fn(command_handle: i32,
                                                                      err: ErrorCode,
                                                                      str1: *const c_char,
                                                                      str2: *const c_char)>) {
        let (sender, receiver) = channel();

        lazy_static! {
                static ref CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String, Option<String>) + Send > >> = Default::default();
        }

        let closure = Box::new(move |err, val1, val2| {
            sender.send((err, val1, val2)).unwrap();
        });

        extern "C" fn _callback(command_handle: i32, err: ErrorCode, str1: *const c_char, str2: *const c_char) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let str1 = unsafe { CStr::from_ptr(str1).to_str().unwrap().to_string() };
            let str2 = if !str2.is_null() {
                unsafe { Some(CStr::from_ptr(str2).to_str().unwrap().to_string()) }
            } else { None };
            cb(err, str1, str2)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    /**
       cb => Option<extern fn(command_handle: i32, err: ErrorCode, raw: *const u8, len: u32)>
    */
    pub fn closure_to_cb_ec_vec_u8() -> (Receiver<(ErrorCode, Vec<u8>)>, i32,
                                          Option<extern fn(command_handle: i32,
                                                           err: ErrorCode,
                                                           raw: *const u8,
                                                           len: u32)>) {
        let (sender, receiver) = channel();

        lazy_static! {
                static ref CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, Vec<u8>) + Send > >> = Default::default();
            }

        let closure = Box::new(move |err, val1| {
            sender.send((err, val1)).unwrap();
        });

        extern "C" fn _callback(command_handle: i32, err: ErrorCode, raw: *const u8, len: u32) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let vec = unsafe { slice::from_raw_parts(raw, len as usize) };
            cb(err, vec.to_vec())
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    /**
       cb => Option<extern fn(command_handle: i32, err: ErrorCode, str: *const c_char, raw: *const u8, len: u32)>
    */
    pub fn closure_to_cb_ec_string_vec_u8() -> (Receiver<(ErrorCode, String, Vec<u8>)>, i32,
                                                 Option<extern fn(command_handle: i32,
                                                                  err: ErrorCode,
                                                                  str: *const c_char,
                                                                  raw: *const u8,
                                                                  len: u32)>) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String, Vec<u8>) + Send > >> = Default::default();
        }

        let closure = Box::new(move |err, val1, val2| {
            sender.send((err, val1, val2)).unwrap();
        });

        extern "C" fn _callback(command_handle: i32, err: ErrorCode, str: *const c_char, raw: *const u8, len: u32) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let str = unsafe { CStr::from_ptr(str).to_str().unwrap().to_string() };
            let vec = unsafe { slice::from_raw_parts(raw, len as usize) };
            cb(err, str, vec.to_vec())
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }
}


/**
    helper methods for managing a timespan/delay
    copied from master/libindy/src/utils/timeout.rs
*/
pub struct TimeoutUtils {}

impl TimeoutUtils {
    /**
        5 second delay
    */
    pub fn short_timeout() -> Duration {
        Duration::from_secs(5)
    }

    /**
        10 second delay
    */
    pub fn medium_timeout() -> Duration {
        Duration::from_secs(10)
    }

    /**
        specify the timeout in seconds
    */
    pub fn specific_timeout(seconds: u64) -> Duration {
        Duration::from_secs(seconds)
    }

    /**
        100 second delay
    */
    pub fn long_timeout() -> Duration {
        Duration::from_secs(100)
    }
}