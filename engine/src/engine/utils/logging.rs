use std::fs::OpenOptions;
use crate::engine::status_codes::StatusCode;
use std::io::prelude::*;
use crate::engine::utils::time;

// This struct is private because other modules don't need to be aware of it, only this one does.
// It is used only to improve the maintainability of the log_levels mod.
pub struct LogLevel
{
    // The static lifetime specifier means that we can only pass string literals or slices with a
    // similar lifetime to the name property of this struct
    name: &'static str,
    // The LogLevels enum stores log level numbers for quick reading during logging. But, enums
    // take only isize values in declaration, so we make the level property of this struct isize.
    // This way it can be stored in the LogLevels enum
    level: isize,
}

// TODO: Should we include a OFF logging type for performance?
// TODO: We should implement some kind of print function to display the log levels and their
// properties, especially the description. This can be done later once the desired interface is
// better defined.
pub mod log_levels
{
    pub const FATAL: super::LogLevel = super::LogLevel{name: "FATAL", level: 10};
    pub const ERROR: super::LogLevel = super::LogLevel{name: "ERROR", level: 20};
    pub const WARN: super::LogLevel = super::LogLevel{name: "WARN", level: 30};
    pub const INFO: super::LogLevel = super::LogLevel{name: "INFO", level: 40};
    pub const DEBUG: super::LogLevel = super::LogLevel{name: "DEBUG", level: 50};
}

enum LogLevelsEnum
{
    FATAL = log_levels::FATAL.level,
    ERROR = log_levels::ERROR.level,
    WARN = log_levels::WARN.level,
    INFO = log_levels::INFO.level,
    DEBUG = log_levels::DEBUG.level,
}

pub struct Logger<'a>
{
    name: &'a str,
    log_level: LogLevel,
    log_file: &'a str,
    // This is to make checking the log level on message logging calls faster.
    quick_log_level: isize,
    quick_log_level_name: &'a str,
}

impl<'a> Logger<'a>
{
    pub const fn new(name: &'a str, log_level: LogLevel, log_file: &'a str) -> Logger<'a>
    {
        return Logger
        {
            name: name,
            quick_log_level: log_level.level,
            quick_log_level_name: log_level.name,
            log_level: log_level,
            log_file: log_file,
        };
    }

    pub fn get_name(&self) -> &str
    {
        return self.name;
    }

    pub fn get_log_file(&self) -> &str
    {
        return self.log_file;
    }

    pub fn get_log_level(&self) -> &LogLevel
    {
        return &(self.log_level);
    }

    pub fn fatal(&self,
                 payload: &str,
                 caller: &str,
                 status_code: StatusCode)
        -> ()
    {
        if self.quick_log_level <= LogLevelsEnum::FATAL as isize
        {
            self.write_to_log_file(payload, caller, status_code);
        }
    }

    pub fn error(&self,
                 payload: &str,
                 caller: &str,
                 status_code: StatusCode)
        -> ()
    {
        if self.quick_log_level <= LogLevelsEnum::ERROR as isize
        {
            self.write_to_log_file(payload, caller, status_code);
        }
    }


    pub fn warn(&self,
                 payload: &str,
                 caller: &str,
                 status_code: StatusCode)
        -> ()
    {
        if self.quick_log_level <= LogLevelsEnum::WARN as isize
        {
            self.write_to_log_file(payload, caller, status_code);
        }
    }


    pub fn info(&self,
            payload: &str,
            caller: &str,
            status_code: StatusCode)
         -> ()
    {
        if self.quick_log_level <= LogLevelsEnum::INFO as isize
        {
            self.write_to_log_file(payload, caller, status_code);
        }
    }

    pub fn debug(&self,
                 payload: &str,
                 caller: &str,
                 status_code: StatusCode)
        -> ()
    {
        if self.quick_log_level <= LogLevelsEnum::DEBUG as isize
        {
            self.write_to_log_file(payload, caller, status_code);
        }
    }

    fn write_to_log_file(&self,
                         payload: &str,
                         caller: &str,
                         status_code: StatusCode)
        -> ()
    {
            let mut file = match OpenOptions::new()
                .append(true)
                .create(true)
                .open(self.log_file)
                {
                    Ok(f) => f,
                    Err(s) => panic!("opening log file '{}' failed with error '{}'", self.log_file, s),
                };

            match write!(file,
                   "{} -- {}: Caller {} logged '{}' and error code '{}'.\n",
                   time::now(),
                   self.quick_log_level_name,
                   caller,
                   payload,
                   status_code)
            {
                Ok(()) => (),
                Err(s) => panic!("writing to log file '{}' failed with error '{}'", self.log_file, s),
            };

    }
}
