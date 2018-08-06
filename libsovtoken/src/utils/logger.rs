//! Logger module contains helper functions for using error!, debug!, trace! etc logging
//! functions and macros in libsovtoken

#[cfg(target_os = "android")]
extern crate android_logger;

use std::env;
use std::io::Write;

use env_logger::{Builder, fmt};
use log::{Record, Level, Metadata, Log, LevelFilter};
#[cfg(target_os = "android")]
use android_logger;
#[cfg(target_os = "android")]
use android_logger::Filter;


/**
    Routes logging to console all of the time regardless of RUST_LOG setting.  helpful for unit tests
*/
pub struct ConsoleLogger;

impl Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("\r\n{:>5}|{:<30}|{:>35}:{:<4}| {}",
                record.level(),
                record.target(),
                record.file().unwrap(),
                record.line().unwrap(),
                record.args()
            );
        }
    }

    fn flush(&self) {

    }
}

/**
    Required call to get logging in libsovtoken to appear, depending on call (debug! vs error! etc)
    and RUST_LOG env setting.
*/
pub fn init_log() {
    if cfg!(target_os = "android") {
        #[cfg(target_os = "android")]
        let log_filter = match env::var("RUST_LOG") {
            Ok(val) => match val.to_lowercase().as_ref(){
                "error" => Filter::default().with_min_level(Level::Error),
                "warn" => Filter::default().with_min_level(Level::Warn),
                "info" => Filter::default().with_min_level(Level::Info),
                "debug" => Filter::default().with_min_level(Level::Debug),
                "trace" => Filter::default().with_min_level(Level::Trace),
                _ => Filter::default().with_min_level(Level::Error),
            },
            Err(..) => Filter::default().with_min_level(Level::Error)
        };

        //Set logging to off when deploying production android app.
        #[cfg(target_os = "android")]
        android_logger::init_once(log_filter);

        info!("Logging for Android");
    } else{
        Builder::new()
            .format(|buf: &mut fmt::Formatter, record: &Record| {
                writeln!(
                    buf,
                    "{:>5}|{:<30}|{:>35}:{:<4}| {}",
                    record.level(),
                    record.target(),
                    record.file().unwrap(),
                    record.line().unwrap(),
                    record.args()
                )
            })
            .filter(None, LevelFilter::Off)
            .parse(env::var("RUST_LOG").as_ref().map(String::as_str).unwrap_or(""))
            .try_init()
            .ok();
    }

}

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