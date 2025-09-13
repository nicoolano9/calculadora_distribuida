use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ClientError {
    NotArgProvided,
    NotEnoughArgs,
    TooManyArgs,
    InvalidAddress,
    MissingArgument,
    ConnectionFailed,
    FileNotFound,
    ReadError,
    WriteError,
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NotArgProvided => write!(f, "argument not provided"),
            Self::NotEnoughArgs => write!(f, "not enough arguments"),
            Self::TooManyArgs => write!(f, "too many arguments"),
            Self::InvalidAddress => write!(f, "invalid address"),
            Self::MissingArgument => write!(f, "missing argument"),
            Self::ConnectionFailed => write!(f, "failed to connect to server"),
            Self::FileNotFound => write!(f, "operations file not found"),
            Self::ReadError => write!(f, "error reading from server"),
            Self::WriteError => write!(f, "error writing to server"),
        }
    }
}

impl Error for ClientError {}