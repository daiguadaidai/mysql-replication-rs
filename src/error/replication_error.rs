use std::char::TryFromCharError;
use std::fmt::{Display, Formatter, Pointer, Result as FmtResult};

use crate::error::{EventError, MysqlError};
use hex::FromHexError;
use std::io::Error as IoError;
use uuid::Error as UuidError;

#[derive(Debug)]
pub enum ReplicationError {
    NoError,
    NormalError(String),
    ParseIntError(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
    IoError(IoError),
    FromHexError(FromHexError),
    UuidError(UuidError),
    TryFromCharError(TryFromCharError),
    ConfigErrors(log4rs::config::runtime::ConfigErrors),
    SetLoggerError(log::SetLoggerError),
    DecimalError(bigdecimal::ParseBigDecimalError),
    SerdeJsonError(serde_json::Error),
    AsyncChannelRecvError(async_channel::RecvError),
    EventError(EventError),
    MysqlError(MysqlError),
    ErrorStack(openssl::error::ErrorStack),
}

impl std::error::Error for ReplicationError {}

impl ReplicationError {
    pub fn new(s: String) -> ReplicationError {
        ReplicationError::NormalError(s)
    }
}

impl Display for ReplicationError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            ReplicationError::NormalError(ref e) => e.fmt(f),
            ReplicationError::ParseIntError(ref e) => e.fmt(f),
            ReplicationError::IoError(ref e) => e.fmt(f),
            ReplicationError::FromHexError(ref e) => e.fmt(f),
            ReplicationError::UuidError(ref e) => e.fmt(f),
            ReplicationError::TryFromCharError(ref e) => e.fmt(f),
            ReplicationError::ConfigErrors(ref e) => e.fmt(f),
            ReplicationError::SetLoggerError(ref e) => e.fmt(f),
            ReplicationError::DecimalError(ref e) => e.fmt(f),
            ReplicationError::SerdeJsonError(ref e) => e.fmt(f),
            ReplicationError::EventError(ref e) => e.fmt(f),
            ReplicationError::NoError => {
                write!(f, "<NoError>")
            }
            ReplicationError::AsyncChannelRecvError(ref e) => e.fmt(f),
            ReplicationError::MysqlError(ref e) => e.fmt(f),
            ReplicationError::ParseFloatError(ref e) => e.fmt(f),
            ReplicationError::ErrorStack(ref e) => e.fmt(f),
        }
    }
}

//将IoError转为ReplicationError
impl From<IoError> for ReplicationError {
    fn from(error: IoError) -> ReplicationError {
        ReplicationError::IoError(error)
    }
}

//将ParseIntError转为ReplicationError
impl From<std::num::ParseIntError> for ReplicationError {
    fn from(error: std::num::ParseIntError) -> ReplicationError {
        ReplicationError::ParseIntError(error)
    }
}

//将ParseFloatError转为ReplicationError
impl From<std::num::ParseFloatError> for ReplicationError {
    fn from(error: std::num::ParseFloatError) -> ReplicationError {
        ReplicationError::ParseFloatError(error)
    }
}

//将ParseIntError转为ReplicationError
impl From<FromHexError> for ReplicationError {
    fn from(error: FromHexError) -> ReplicationError {
        ReplicationError::FromHexError(error)
    }
}

//将UuidError转为ReplicationError
impl From<UuidError> for ReplicationError {
    fn from(error: UuidError) -> ReplicationError {
        ReplicationError::UuidError(error)
    }
}

//将TryFromCharError转为ReplicationError
impl From<TryFromCharError> for ReplicationError {
    fn from(error: TryFromCharError) -> ReplicationError {
        ReplicationError::TryFromCharError(error)
    }
}

//将ConfigErrors转为 ReplicationError
impl From<log4rs::config::runtime::ConfigErrors> for ReplicationError {
    fn from(error: log4rs::config::runtime::ConfigErrors) -> ReplicationError {
        ReplicationError::ConfigErrors(error)
    }
}

//将SetLoggerError转为 ReplicationError
impl From<log::SetLoggerError> for ReplicationError {
    fn from(error: log::SetLoggerError) -> ReplicationError {
        ReplicationError::SetLoggerError(error)
    }
}

//将DecimalError转为 ReplicationError
impl From<bigdecimal::ParseBigDecimalError> for ReplicationError {
    fn from(error: bigdecimal::ParseBigDecimalError) -> ReplicationError {
        ReplicationError::DecimalError(error)
    }
}

//将SerdeJsonError转为 ReplicationError
impl From<serde_json::Error> for ReplicationError {
    fn from(error: serde_json::Error) -> ReplicationError {
        ReplicationError::SerdeJsonError(error)
    }
}

//将SerdeJsonError转为 ReplicationError
impl From<EventError> for ReplicationError {
    fn from(error: EventError) -> ReplicationError {
        ReplicationError::EventError(error)
    }
}

//将RecvError转为 ReplicationError
impl From<async_channel::RecvError> for ReplicationError {
    fn from(error: async_channel::RecvError) -> ReplicationError {
        ReplicationError::AsyncChannelRecvError(error)
    }
}

//将MysqlError转为 ReplicationError
impl From<MysqlError> for ReplicationError {
    fn from(error: MysqlError) -> ReplicationError {
        ReplicationError::MysqlError(error)
    }
}

//将ErrorStack转为 ReplicationError
impl From<openssl::error::ErrorStack> for ReplicationError {
    fn from(error: openssl::error::ErrorStack) -> ReplicationError {
        ReplicationError::ErrorStack(error)
    }
}
