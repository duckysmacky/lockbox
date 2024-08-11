use std::{
    sync::{Arc, Mutex},
    string::String, cmp::PartialEq,
    fmt
};

use clap::ArgMatches;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref LOGGER: Arc<Mutex<Logger>> = Arc::new(
        Mutex::new(
            Logger::new()
        )
    );
}

#[derive(PartialEq)]
pub enum LogType {
    DEBUG, INFO, WARNING, SUCCESS, ERROR, FATAL
}

enum LoggerMode {
    QUIET, NORMAL, VERBOSE
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
        if log_type == LogType::FATAL {
            println!("[{}] {}", get_icon(log_type), message);
            return;
        }
        if log_type == LogType::DEBUG {
            if !self.debug {
                return;
            }
            println!("[{}] {}", get_icon(log_type), message);
            return;
        }
        match self.mode {
            LoggerMode::QUIET => {
                return;
            },
            LoggerMode::NORMAL => {
                if log_type == LogType::SUCCESS || log_type == LogType::ERROR {
                    println!("[{}] {}", get_icon(log_type), message);
                }
            },
            LoggerMode::VERBOSE => {
                println!("[{}] {}", get_icon(log_type), message);
            },
        }
    }
}

pub fn configure_logger(args: &ArgMatches) {
    let mut logger = LOGGER.lock().unwrap();
    logger.debug = args.get_flag("debug");
    logger.mode = {
        if args.get_flag("quiet") {
            LoggerMode::QUIET
        } else if args.get_flag("verbose") {
            LoggerMode::VERBOSE
        } else {
            LoggerMode::NORMAL
        }
    };
}


#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        {
            use crate::cli::logger::*;
            let logger = LOGGER.lock().unwrap();
            logger.log(LogType::INFO, format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        {
            use crate::cli::logger::*;
            let logger = LOGGER.lock().unwrap();
            logger.log(LogType::WARNING, format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_success {
    ($($arg:tt)*) => {
        {
            use crate::cli::logger::*;
            let logger = LOGGER.lock().unwrap();
            logger.log(LogType::SUCCESS, format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        {
            use crate::cli::logger::*;
            let logger = LOGGER.lock().unwrap();
            logger.log(LogType::ERROR, format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        {
            use crate::cli::logger::*;
            let logger = LOGGER.lock().unwrap();
            logger.log(LogType::DEBUG, format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_fatal {
    ($($arg:tt)*) => {
        {
            use crate::cli::logger::*;
            let logger = LOGGER.lock().unwrap();
            logger.log(LogType::FATAL, format_args!($($arg)*));
            panic!()
        }
    };
}

fn get_icon(log_type: LogType) -> String {
    match log_type {
        LogType::INFO => String::from("i"),
        LogType::WARNING => String::from("*"),
        LogType::SUCCESS => String::from("+"),
        LogType::ERROR => String::from("-"),
        LogType::FATAL => String::from("!"),
        LogType::DEBUG => String::from("d")
    }
}