//!
use std::sync::mpsc::Receiver;
use std::time::Duration;

use utils::ErrorCode;

pub struct ResultHandler {}

macro_rules! try_err {
    ($x:ident) => {
        if $x != ErrorCode::Success {
            return Err($x)
        }
    }
}

fn map_recv_timeout_channel_error_error_code(err: ::std::sync::mpsc::RecvTimeoutError) -> ErrorCode {
    match err {
        ::std::sync::mpsc::RecvTimeoutError::Timeout => {
            warn!("Timed out waiting for libindy to call back");
            ErrorCode::CommonIOError
        }
        ::std::sync::mpsc::RecvTimeoutError::Disconnected => {
            warn!("Channel to libindy was disconnected unexpectedly");
            ErrorCode::CommonIOError
        }
    }
}

fn map_recv_channel_error_error_code(err: ::std::sync::mpsc::RecvError) -> ErrorCode {
    warn!("Channel returned an error - {:?}", err);
    ErrorCode::CommonIOError
}

impl ResultHandler {
    pub fn one<T>(err: ErrorCode, receiver: Receiver<(ErrorCode, T)>) -> Result<T, ErrorCode> {
        try_err!(err);

        let (err, val) = receiver.recv().map_err(map_recv_channel_error_error_code)?;

        try_err!(err);

        Ok(val)
    }

    pub fn one_timeout<T>(err: ErrorCode, receiver: Receiver<(ErrorCode, T)>, timeout: Duration) -> Result<T, ErrorCode> {
        try_err!(err);

        let (err, val) = receiver.recv_timeout(timeout).map_err(map_recv_timeout_channel_error_error_code)?;

        try_err!(err);

        Ok(val)
    }
}
