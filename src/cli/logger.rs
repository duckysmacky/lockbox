//! Logging macros for CLI mode

use std::fmt;
use std::sync::{Arc, Mutex};
use clap::ArgMatches;
use lazy_static::lazy_static;
use crate::core::logs::LogType;

lazy_static! {
    pub static ref LOGGER: Arc<Mutex<Logger>> = Arc::new(Mutex::new(Logger::new()));
}

enum LoggerMode {
    QUIET,
    NORMAL,
    VERBOSE,
}

pub struct Logger {
    debug: bool,
    mode: LoggerMode
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            debug: false,
            mode: LoggerMode::NORMAL,
        }
    }

    pub fn log(&self, log_type: LogType, message: fmt::Arguments<'_>) {
        if log_type == LogType::DEBUG {
            if !self.debug {
                return;
            }
            println!("[{}] {}", log_type.icon(), message);
            return;
        }

        match self.mode {
            LoggerMode::QUIET => {
                return;
            },
            LoggerMode::NORMAL => {
                if log_type == LogType::ERROR || log_type == LogType::WARN {
                    eprintln!("[{}] {}", log_type.icon(), message);
                } else if log_type == LogType::SUCCESS || log_type == LogType::STATUS {
                    println!("[{}] {}", log_type.icon(), message);
                }
            },
            LoggerMode::VERBOSE => {
                if log_type == LogType::ERROR || log_type == LogType::WARN {
                    eprintln!("[{}] {}", log_type.icon(), message);
                } else {
                    println!("[{}] {}", log_type.icon(), message);
                }
            },
        }
    }
}

pub fn configure_logger(args: &ArgMatches) {
    let mut logger = LOGGER.lock().unwrap();
    logger.debug = args.get_flag("DEBUG");
    logger.mode = {
        if args.get_flag("QUIET") {
            LoggerMode::QUIET
        } else if args.get_flag("VERBOSE") {
            LoggerMode::VERBOSE
        } else {
            LoggerMode::NORMAL
        }
    };
}