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
    pub fn empty(err: ErrorCode, receiver: Receiver<ErrorCode>) -> Result<(), ErrorCode> {
        try_err!(err);

        let err = receiver.recv().map_err(map_recv_channel_error_error_code)?;

        try_err!(err);

        Ok(())
    }

    pub fn empty_timeout(err: ErrorCode, receiver: Receiver<ErrorCode>, timeout: Duration) -> Result<(), ErrorCode> {
        try_err!(err);

        let err = receiver.recv_timeout(timeout).map_err(map_recv_timeout_channel_error_error_code)?;

        try_err!(err);

        Ok(())
    }

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

    pub fn two<T1, T2>(err: ErrorCode, receiver: Receiver<(ErrorCode, T1, T2)>) -> Result<(T1, T2), ErrorCode> {
        try_err!(err);

        let (err, val, val2) = receiver.recv().map_err(map_recv_channel_error_error_code)?;

        try_err!(err);

        Ok((val, val2))
    }

    pub fn two_timeout<T1, T2>(err: ErrorCode, receiver: Receiver<(ErrorCode, T1, T2)>, timeout: Duration) -> Result<(T1, T2), ErrorCode> {
        try_err!(err);

        let (err, val, val2) = receiver.recv_timeout(timeout).map_err(map_recv_timeout_channel_error_error_code)?;

        try_err!(err);

        Ok((val, val2))
    }

    pub fn three<T1, T2, T3>(err: ErrorCode, receiver: Receiver<(ErrorCode, T1, T2, T3)>) -> Result<(T1, T2, T3), ErrorCode> {
        try_err!(err);

        let (err, val, val2, val3) = receiver.recv().map_err(map_recv_channel_error_error_code)?;

        try_err!(err);

        Ok((val, val2, val3))
    }

    pub fn three_timeout<T1, T2, T3>(err: ErrorCode, receiver: Receiver<(ErrorCode, T1, T2, T3)>, timeout: Duration) -> Result<(T1, T2, T3), ErrorCode> {
        try_err!(err);

        let (err, val, val2, val3) = receiver.recv_timeout(timeout).map_err(map_recv_timeout_channel_error_error_code)?;

        try_err!(err);

        Ok((val, val2, val3))
    }
}
