use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ProtocolError {
    InvalidCommand,
    InvalidArgument,
    InvalidOperator,
    MissingArgument,
    Empty,
    UnexpectedMessage(String),
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidCommand => write!(f, "parsing error: invalid command"),
            Self::MissingArgument => write!(f, "parsing error: missing argument"),
            Self::InvalidArgument => write!(f, "parsing error: invalid argument"),
            Self::InvalidOperator => write!(f, "parsing error: invalid operator"),
            Self::Empty => write!(f, "parsing error: empty command"),
            Self::UnexpectedMessage(msg) => write!(f, "unexpected message: {}", msg),
        }
    }
}

impl Error for ProtocolError {}
