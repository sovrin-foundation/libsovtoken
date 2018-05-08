//! Logger module contains helper functions for using error!, debug!, trace! etc logging
//! functions and macros in libsovtoken

use log::{Record, Level, Metadata, Log};

pub struct ConsoleLogger;

impl Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("\r\nTesting log => {} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {

    }
}

