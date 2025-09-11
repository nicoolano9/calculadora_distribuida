use std::fmt;

#[derive(Debug)]    
pub enum CalcError {
    DivisionByZero,
    UnknownOperator,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DivisionByZero => write!(f, "Division by zero"),
            Self::UnknownOperator => write!(f, "Unknown operator"),
        }
    }
}