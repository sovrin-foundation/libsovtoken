/*!
    Set of callback utils for testing.

    **These should only be used for testing**
*/

use utils::ErrorCode;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::sync::Mutex;
use std::sync::mpsc::{channel, Receiver};
use std::collections::HashMap;
use std::os::raw::c_char;
use std::ffi::CStr;

// use utils::ErrorCode;

type Callbacks<F> = Mutex<HashMap<i32, Box<F>>>;

macro_rules! closure_cb {
    ($closure:ident, $($name:ident : $ntype:ty),*) => {{
        lazy_static! {
            static ref COMMAND_HANDLE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
            static ref CALLBACKS: Callbacks<FnMut(i32, $($ntype),*) + Send> = Default::default();
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, $closure);

        extern "C" fn _callback(command_handle: i32, err: i32, $($name : $ntype),*) -> i32 {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err, $($name),*);

            return err;
        }

        (command_handle, _callback)
    }}
}


pub fn cb_ec_string() -> (
    Receiver<(ErrorCode, String)>,
    i32,
    Option<extern fn(command_handle: i32, err: i32, c_str: *const c_char) -> i32>) {
    let (sender, receiver) = channel();

    let closure = Box::new(move|error_code, c_str| {
        let string = unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() };
        sender.send((ErrorCode::from(error_code), string)).unwrap();
    });

    let (command_handle, callback) = closure_cb!(closure, char_value: *const c_char);

    (receiver, command_handle, Some(callback))
}

pub fn cb_ec_i32() -> (
    Receiver<(ErrorCode, i32)>,
    i32,
    Option<extern fn(command_handle: i32, err: i32, c_i32: i32) -> i32>) {
    let (sender, receiver) = channel();

    let closure = Box::new(move|error_code, c_i32| {
        sender.send((ErrorCode::from(error_code), c_i32)).unwrap();
    });

    let (command_handle, callback) = closure_cb!(closure, i32_value: i32);

    (receiver, command_handle, Some(callback))
}