use std::fmt;

#[derive(Debug)]
pub enum ProtocolError {
    InvalidCommand,
    InvalidArgument,
    InvalidOperator,
    MissingArgument,
    Empty,
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidCommand => write!(f, "Invalid command"),
            Self::MissingArgument => write!(f, "Missing argument"),
            Self::InvalidArgument => write!(f, "Invalid argument"),
            Self::InvalidOperator => write!(f, "Invalid operator"),
            Self::Empty => write!(f, "Empty command"),
        }
    }
}
