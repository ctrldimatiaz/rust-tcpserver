use std::fmt;

pub enum ParseError {
    InvalidCommand,
    MissingArgument,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCommand => write!(f, "Invalid Command"),
            Self::MissingArgument => write!(f, "Missing Argument"),
        }
    }
}
