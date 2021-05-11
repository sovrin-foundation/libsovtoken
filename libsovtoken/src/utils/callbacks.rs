//!

use std::collections::HashMap;
use std::ffi::CStr;
use std::fmt::Display;
use std::os::raw::c_char;
use std::slice;
use std::sync::Mutex;
use std::sync::mpsc::{channel, Receiver};

use indy_sys::{ResponseEmptyCB,
               ResponseStringCB,
               ResponseSliceCB};

use {ErrorCode, IndyHandle};
use utils::sequence::SequenceUtils;

fn log_error<T: Display>(e: T) {
    warn!("Unable to send through libindy callback: {}", e);
}

pub struct ClosureHandler {}

impl ClosureHandler {
    pub fn cb_ec() -> (Receiver<ErrorCode>, IndyHandle, Option<ResponseEmptyCB>) {
        let (sender, receiver) = channel();

        let closure = Box::new(move |err| {
            sender.send(err).unwrap_or_else(log_error);
        });

        let (command_handle, cb) = ClosureHandler::convert_cb_ec(closure);

        (receiver, command_handle, cb)
    }

    pub fn convert_cb_ec(closure: Box<dyn FnMut(ErrorCode) + Send>) -> (IndyHandle, Option<ResponseEmptyCB>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<dyn FnMut(ErrorCode) + Send>>> = Default::default();
        }
        extern "C" fn _callback(command_handle: IndyHandle, err: i32) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(ErrorCode::from(err))
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (command_handle, Some(_callback))
    }

    pub fn cb_ec_string() -> (Receiver<(ErrorCode, String)>, IndyHandle, Option<ResponseStringCB>) {
        let (sender, receiver) = channel();

        let closure = Box::new(move |err, val| {
            sender.send((err, val)).unwrap_or_else(log_error);
        });

        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(closure);

        (receiver, command_handle, cb)
    }

    pub fn convert_cb_ec_string(closure: Box<dyn FnMut(ErrorCode, String) + Send>) -> (IndyHandle, Option<ResponseStringCB>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<dyn FnMut(ErrorCode, String) + Send>>> = Default::default();
        }

        extern "C" fn _callback(command_handle: IndyHandle, err: i32, c_str: *const c_char) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let metadata = rust_str!(c_str);
            cb(ErrorCode::from(err), metadata)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (command_handle, Some(_callback))
    }

    pub fn cb_ec_slice() -> (Receiver<(ErrorCode, Vec<u8>)>, IndyHandle, Option<ResponseSliceCB>) {
        let (sender, receiver) = channel();

        let closure = Box::new(move |err, sig| {
            sender.send((err, sig)).unwrap_or_else(log_error);
        });

        let (command_handle, cb) = ClosureHandler::convert_cb_ec_slice(closure);

        (receiver, command_handle, cb)
    }

    pub fn convert_cb_ec_slice(closure: Box<dyn FnMut(ErrorCode, Vec<u8>) + Send>) -> (IndyHandle, Option<ResponseSliceCB>) {
        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<dyn FnMut(ErrorCode, Vec<u8>) + Send>>> = Default::default();
        }

        extern "C" fn _callback(command_handle: IndyHandle, err: i32, raw: *const u8, len: u32) {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let sig = rust_slice!(raw, len);
            cb(ErrorCode::from(err), sig.to_vec())
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = SequenceUtils::get_next_id();
        callbacks.insert(command_handle, closure);

        (command_handle, Some(_callback))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cb_ec_slice() {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

        let test_vec: Vec<u8> = vec![250, 251, 252, 253, 254, 255];
        let callback = cb.unwrap();
        callback(command_handle, 0, test_vec.as_ptr(), test_vec.len() as u32);

        let (err, slice1) = receiver.recv().unwrap();
        assert_eq!(err, ErrorCode::Success);
        assert_eq!(test_vec, slice1);
    }
}
