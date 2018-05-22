//! callback helpers (can this go away once libsovtoken is dynamically linked to indy-sdk)


use libc::c_char;
use std::ffi::CStr;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::sync::Mutex;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

use indy::*;

lazy_static! {
    static ref COMMAND_HANDLE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
    static ref CALLBACKS_EC: Mutex<HashMap<i32, Box<FnMut(ErrorCode) + Send>>> = Default::default();
    static ref CALLBACKS_EC_I32: Mutex<HashMap<i32, Box<FnMut(ErrorCode, i32) + Send>>> = Default::default();
    static ref CALLBACKS_EC_STRING: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) + Send > >> = Default::default();
}

pub struct CallbackUtils {}

impl CallbackUtils {
    pub fn closure_to_cb_ec() -> (Receiver<ErrorCode>, i32,
                                   Option<extern fn(command_handle: i32,
                                                    err: ErrorCode)>) {


        let (sender, receiver) = channel();

        let closure = Box::new(move |err| {
            sender.send(err).unwrap();
        });

        extern "C" fn _callback(command_handle: i32, err: ErrorCode) {
            let mut callbacks = CALLBACKS_EC.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err)
        }

        let mut callbacks = CALLBACKS_EC.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))

    }

    pub fn closure_to_cb_ec_i32() -> (Receiver<(ErrorCode, i32)>, i32,
                                       Option<extern fn(command_handle: i32, err: ErrorCode,
                                                        c_i32: i32)>) {
        let (sender, receiver) = channel();

        let closure = Box::new(move |err, val| {
            sender.send((err, val)).unwrap();
        });

        extern "C" fn _callback(command_handle: i32, err: ErrorCode, c_i32: i32) {
            let mut callbacks = CALLBACKS_EC_I32.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err, c_i32)
        }

        let mut callbacks = CALLBACKS_EC_I32.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }

    pub fn closure_to_cb_ec_string() -> (Receiver<(ErrorCode, String)>, i32,
                                          Option<extern fn(command_handle: i32,
                                                           err: ErrorCode,
                                                           c_str: *const c_char)>) {
        let (sender, receiver) = channel();

        let closure = Box::new(move |err, val| {
            sender.send((err, val)).unwrap();
        });

        extern "C" fn _callback(command_handle: i32, err: ErrorCode, c_str: *const c_char) {
            let mut callbacks = CALLBACKS_EC_STRING.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let metadata = unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() };
            cb(err, metadata)
        }

        let mut callbacks = CALLBACKS_EC_STRING.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, command_handle, Some(_callback))
    }
}

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