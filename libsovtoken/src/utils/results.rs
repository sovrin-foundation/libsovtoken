//!
use std::sync::mpsc::Receiver;

use utils::ErrorCode;

pub struct ResultHandler {}

macro_rules! try_err {
    ($x:ident) => {
        if $x != ErrorCode::Success {
            return Err($x)
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

        match receiver.recv().map_err(map_recv_channel_error_error_code) {
            Ok(err) => {
                try_err!(err);
                Ok(())
            }
            Err(e) => Err(e)
        }
    }

    pub fn one<T>(err: ErrorCode, receiver: Receiver<(ErrorCode, T)>) -> Result<T, ErrorCode> {
        try_err!(err);

        let (err, val) = receiver.recv().map_err(map_recv_channel_error_error_code)?;

        try_err!(err);

        Ok(val)
    }
}
