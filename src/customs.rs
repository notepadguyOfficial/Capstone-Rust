use backtrace::Backtrace;
use chrono::Local;
use colored::*;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use std::fs::{File, OpenOptions, create_dir_all, metadata};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime};

pub struct CustomLogs {
    file: Mutex<File>,
}

impl CustomLogs {
    pub fn new(directory: &str) -> std::io::Result<Self> {
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M").to_string();
        let filename = format!("Logs-{}.log", timestamp);
        let mut path = PathBuf::from(directory);
        path.push(filename);

        if let Some(parent) = path.parent() {
            create_dir_all(parent)?;
        }

        if path.exists() {
            if let Ok(file_metadata) = metadata(&path) {
                if let Ok(modified_time) = file_metadata.modified() {
                    let now = SystemTime::now();
                    if let Ok(duration) = now.duration_since(modified_time) {
                        if duration.as_secs() <= 3600 {
                            let file = OpenOptions::new().append(true).open(path)?;
                            return Ok(CustomLogs {
                                file: Mutex::new(file),
                            });
                        }
                    }
                }
            }
        }

        let file = OpenOptions::new().create(true).append(true).open(path)?;

        Ok(CustomLogs {
            file: Mutex::new(file),
        })
    }
}

fn function() -> Option<String> {
    let backtrace = Backtrace::new();
    for frame in backtrace.frames() {
        if let Some(symbol) = frame.symbols().get(0) {
            if let Some(name) = symbol.name() {
                if !name.to_string().contains("log") && !name.to_string().contains("function") {
                    let name = name.to_string().replace("Server::", "");
                    let name = name.to_string().split("::").next().unwrap_or(&name).to_string();
                    return Some(name);
                }
            }
        }
    }
    None
}

impl log::Log for CustomLogs {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let file = record.file().unwrap_or("unknown");
            let line = record.line().unwrap_or(0);
            let func = function().unwrap_or("unknown".to_owned());

            let category = match record.target() {
                target if target.contains("HTTP") => "HTTP",
                target if target.contains("POSTGRES") => "POSTGRES",
                target if target.contains("WEBSOCKET") => "WEBSOCKET",
                target if target.contains("DATABASE") => "DATABASE",
                target if target.contains("DAT") => "DAT",
                _ => "",
            };

            let category_level = if category.is_empty() {
                format!("[{}]", record.level())
            } else {
                format!("[{}:{}]", category, record.level())
            };

            let category_colored = match category {
                "HTTP" => "HTTP".bright_red(),
                "POSTGRES" => "POSTGRES".bright_green(),
                "WEBSOCKET" => "WEBSOCKET".bright_blue(),
                "DATABASE" => "DATABASE".bright_magenta(),
                "DAT" => "DAT".bright_magenta(),
                _ => category.to_string().bright_white(),
            };

            let level_colored = match record.level() {
                Level::Error => record.level().to_string().red(),
                Level::Warn => record.level().to_string().yellow(),
                Level::Info => record.level().to_string().cyan(),
                Level::Debug => record.level().to_string().white(),
                Level::Trace => record.level().to_string().dimmed(),
            };

            let category_level_colored = if category.is_empty() {
                format!("[{}]", level_colored).bright_white()
            } else {
                format!("[{}:{}]", category_colored, level_colored).bright_white()
            };
            
            let msg_colored = match record.level() {
                Level::Error => format!("{}", record.args()).red(),
                Level::Warn => format!("{}", record.args()).yellow(),
                Level::Info => format!("{}", record.args()).cyan(),
                Level::Debug => format!("{}", record.args()).white(),
                Level::Trace => format!("{}", record.args()).dimmed(),
            };

            println!(
                "[{}] [{}] [{}] {} : {}",
                timestamp.dimmed(),
                format!("{}:{}", file, line).blue(),
                func.green(),
                category_level_colored,
                msg_colored
            );

            let log_line = format!(
                "[{}] [{}:{}] [{}] {} : {}\n",
                timestamp,
                file,
                line,
                func,
                category_level,
                record.args()
            );

            if let Ok(mut file) = self.file.lock() {
                let _ = file.write_all(log_line.as_bytes());
                let _ = file.flush();
            }
        }
    }

    fn flush(&self) {}
}

#[macro_export]
macro_rules! http {
    (error, $fmt:expr $(, $arg:tt)*) => {
        log::error!(target: "HTTP", concat!($fmt) $(, $arg)*)
    };
    (warn, $fmt:expr $(, $arg:tt)*) => {
        log::warn!(target: "HTTP", concat!($fmt) $(, $arg)*)
    };
    (info, $fmt:expr $(, $arg:tt)*) => {
        log::info!(target: "HTTP", concat!($fmt) $(, $arg)*)
    };
    (debug, $fmt:expr $(, $arg:tt)*) => {
        log::debug!(target: "HTTP", concat!($fmt) $(, $arg)*)
    };
    (trace, $fmt:expr $(, $arg:tt)*) => {
        log::trace!(target: "HTTP", concat!($fmt) $(, $arg)*)
    };
}

#[macro_export]
macro_rules! postgres {
    (error, $fmt:expr $(, $arg:tt)*) => {
        log::error!(target: "POSTGRES", concat!($fmt) $(, $arg)*)
    };
    (warn, $fmt:expr $(, $arg:tt)*) => {
        log::warn!(target: "POSTGRES", concat!($fmt) $(, $arg)*)
    };
    (info, $fmt:expr $(, $arg:tt)*) => {
        log::info!(target: "POSTGRES", concat!($fmt) $(, $arg)*)
    };
    (debug, $fmt:expr $(, $arg:tt)*) => {
        log::debug!(target: "POSTGRES", concat!($fmt) $(, $arg)*)
    };
    (trace, $fmt:expr $(, $arg:tt)*) => {
        log::trace!(target: "POSTGRES", concat!($fmt) $(, $arg)*)
    };
}

#[macro_export]
macro_rules! ws {
    (error, $fmt:expr $(, $arg:tt)*) => {
        log::error!(target: "WEBSOCKET", concat!($fmt) $(, $arg)*)
    };
    (warn, $fmt:expr $(, $arg:tt)*) => {
        log::warn!(target: "WEBSOCKET", concat!($fmt) $(, $arg)*)
    };
    (info, $fmt:expr $(, $arg:tt)*) => {
        log::info!(target: "WEBSOCKET", concat!($fmt) $(, $arg)*)
    };
    (debug, $fmt:expr $(, $arg:tt)*) => {
        log::debug!(target: "WEBSOCKET", concat!($fmt) $(, $arg)*)
    };
    (trace, $fmt:expr $(, $arg:tt)*) => {
        log::trace!(target: "WEBSOCKET", concat!($fmt) $(, $arg)*)
    };
}

#[macro_export]
macro_rules! wss {
    (error, $fmt:expr $(, $arg:tt)*) => {
        log::error!(target: "WEBSOCKET:SECURE", concat!($fmt) $(, $arg)*)
    };
    (warn, $fmt:expr $(, $arg:tt)*) => {
        log::warn!(target: "WEBSOCKET:SECURE", concat!($fmt) $(, $arg)*)
    };
    (info, $fmt:expr $(, $arg:tt)*) => {
        log::info!(target: "WEBSOCKET:SECURE", concat!($fmt) $(, $arg)*)
    };
    (debug, $fmt:expr $(, $arg:tt)*) => {
        log::debug!(target: "WEBSOCKET:SECURE", concat!($fmt) $(, $arg)*)
    };
    (trace, $fmt:expr $(, $arg:tt)*) => {
        log::trace!(target: "WEBSOCKET:SECURE", concat!($fmt) $(, $arg)*)
    };
}

#[macro_export]
macro_rules! database {
    (error, $fmt:expr $(, $arg:tt)*) => {
        log::error!(target: "DATABASE", concat!($fmt) $(, $arg)*)
    };
    (warn, $fmt:expr $(, $arg:tt)*) => {
        log::warn!(target: "DATABASE", concat!($fmt) $(, $arg)*)
    };
    (info, $fmt:expr $(, $arg:tt)*) => {
        log::info!(target: "DATABASE", concat!($fmt) $(, $arg)*)
    };
    (debug, $fmt:expr $(, $arg:tt)*) => {
        log::debug!(target: "DATABASE", concat!($fmt) $(, $arg)*)
    };
    (trace, $fmt:expr $(, $arg:tt)*) => {
        log::trace!(target: "DATABASE", concat!($fmt) $(, $arg)*)
    };
}

#[macro_export]
macro_rules! datl {
    (error, $fmt:expr $(, $arg:tt)*) => {
        log::error!(target: "DAT", concat!($fmt) $(, $arg)*)
    };
    (warn, $fmt:expr $(, $arg:tt)*) => {
        log::warn!(target: "DAT", concat!($fmt) $(, $arg)*)
    };
    (info, $fmt:expr $(, $arg:tt)*) => {
        log::info!(target: "DAT", concat!($fmt) $(, $arg)*)
    };
    (debug, $fmt:expr $(, $arg:tt)*) => {
        log::debug!(target: "DAT", concat!($fmt) $(, $arg)*)
    };
    (trace, $fmt:expr $(, $arg:tt)*) => {
        log::trace!(target: "DAT", concat!($fmt) $(, $arg)*)
    };
}

pub fn init(directory: &str) -> Result<(), SetLoggerError> {
    let logger = CustomLogs::new(directory).expect("Failed to open log file");
    let logger = Box::leak(Box::new(logger));

    log::set_logger(logger)?;
    log::set_max_level(LevelFilter::Trace);

    Ok(())
}
