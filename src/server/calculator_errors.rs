use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CalcError {
    DivisionByZero,
    UnknownOperator,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DivisionByZero => write!(f, "division by zero"),
            Self::UnknownOperator => write!(f, "parsing error: unknown operator"),
        }
    }
}

impl Error for CalcError {}
