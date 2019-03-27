//! Logger module contains helper functions for using error!, debug!, trace! etc logging
//! functions and macros in libsovtoken

use std::ffi::CString;
use std::ptr::null;
use libc::c_void;

use indy_sys::logger::{EnabledCB, LogCB, FlushCB};
use log;
use log::{Record, Metadata, LevelFilter};

use logic::indy_sdk_api;
use ErrorCode;

pub struct SovtokenLogger {
    context: *const c_void,
    enabled: Option<EnabledCB>,
    log: LogCB,
    flush: Option<FlushCB>,
}

impl SovtokenLogger {
    fn new(context: *const c_void, enabled: Option<EnabledCB>, log: LogCB, flush: Option<FlushCB>) -> Self {
        SovtokenLogger { context, enabled, log, flush }
    }

    pub fn init() -> Result<(), ErrorCode> {
        // logging, as implemented, crashes with VCX for android and ios, so
        // for this hotfix (IS-1164) simply return OK
        if cfg!(target_os = "android") || cfg!(target_os = "ios") {
            return Ok(());
        }

        let (context, enabled, log, flush) = indy_sdk_api::logger::get_logger()?;

        let log = match log {
            Some(log) => log,
            None => return Err(ErrorCode::CommonInvalidState)
        };

        let logger = SovtokenLogger::new(context, enabled, log, flush);

        log::set_boxed_logger(Box::new(logger)).ok();
        log::set_max_level(LevelFilter::Trace);
        Ok(())
    }
}

impl log::Log for SovtokenLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        if let Some(enabled_cb) = self.enabled {
            let level = metadata.level() as u32;
            let target = c_str!(metadata.target());

            enabled_cb(self.context,
                       level,
                       target.as_ptr(),
            )
        } else { true }
    }

    fn log(&self, record: &Record) {
        let log_cb = self.log;

        let level = record.level() as u32;

        let target = record.target();
        let message = record.args().to_string();
        let module_path = record.module_path();
        let file = record.file();

        let target = c_str!(target);
        let message = c_str!(message);
        let module_path_str = opt_c_str!(module_path);
        let file_str = opt_c_str!(file);

        let line = record.line().unwrap_or(0);

        log_cb(self.context,
               level,
               target.as_ptr(),
               message.as_ptr(),
               opt_c_ptr!(module_path, module_path_str),
               opt_c_ptr!(file, file_str),
               line,
        )
    }

    fn flush(&self) {
        if let Some(flush_cb) = self.flush {
            flush_cb(self.context)
        }
    }
}

unsafe impl Sync for SovtokenLogger {}

unsafe impl Send for SovtokenLogger {}

macro_rules! _map_err {
    ($lvl:expr, $expr:expr) => (
        |err| {
            log!($lvl, "{} - {}", $expr, err);
            err
        }
    );
    ($lvl:expr) => (
        |err| {
            log!($lvl, "{:?}", err);
            err
        }
    )
}

#[macro_export]
macro_rules! map_err_err {
    () => ( _map_err!(::log::Level::Error) );
    ($($arg:tt)*) => ( _map_err!(::log::Level::Error, $($arg)*) )
}

#[macro_export]
macro_rules! map_err_trace {
    () => ( _map_err!(::log::Level::Trace) );
    ($($arg:tt)*) => ( _map_err!(::log::Level::Trace, $($arg)*) )
}