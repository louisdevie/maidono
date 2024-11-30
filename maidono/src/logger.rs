use maidono_core::utils::Error;
use rocket::config::LogLevel;
use rocket::request::{FromRequest, Outcome};
use rocket::{Config, Request};
use std::convert::Infallible;
use std::fmt::{Debug, Display};

pub const LOG_LEVEL_HIGHEST: u8 = 4;
pub const LOG_LEVEL_ERROR: u8 = 3;
pub const LOG_LEVEL_WARNING: u8 = 2;
pub const LOG_LEVEL_INFO: u8 = 1;
pub const LOG_LEVEL_DEBUG: u8 = 0;

#[derive(Clone, Copy)]
pub struct Logger {
    minimum_level: u8,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Logger {
    type Error = Infallible;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        req.guard::<&Config>()
            .await
            .map(|config| config.log_level.into())
    }
}

impl From<LogLevel> for Logger {
    fn from(log_level: LogLevel) -> Self {
        Self {
            minimum_level: match log_level {
                LogLevel::Critical => LOG_LEVEL_WARNING,
                LogLevel::Normal => LOG_LEVEL_INFO,
                LogLevel::Debug => LOG_LEVEL_DEBUG,
                LogLevel::Off => LOG_LEVEL_HIGHEST,
            },
        }
    }
}

impl Logger {
    pub fn error(&self, error: Error) {
        if self.minimum_level <= LOG_LEVEL_ERROR {
            println!("[maidono: error] {:?}", error)
        }
    }

    pub fn error_message<D: Display>(&self, message: D) {
        if self.minimum_level <= LOG_LEVEL_ERROR {
            println!("[maidono: error] {}", message)
        }
    }

    // pub fn warning_message<D: Display>(&self, message: D) {
    //     if self.minimum_level <= LOG_LEVEL_WARNING {
    //         println!("[maidono: warning] {}", message)
    //     }
    // }

    pub fn log<D: Display>(&self, message: D) {
        if self.minimum_level <= LOG_LEVEL_INFO {
            println!("[maidono: info] {}", message)
        }
    }

    pub fn debug<D: Debug>(&self, message: D) {
        if self.minimum_level <= LOG_LEVEL_DEBUG {
            println!("[maidono: debug] {:#?}", message)
        }
    }

    pub fn debug_message<D: Display>(&self, message: D) {
        if self.minimum_level <= LOG_LEVEL_DEBUG {
            println!("[maidono: debug] {}", message)
        }
    }
}
