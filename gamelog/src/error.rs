use ron::de::SpannedError;
use std::{fmt, io};

#[derive(Debug)]
pub enum LogFileError {
    IOError(io::Error),
    RonSpanned(ron::error::SpannedError),
    TeamCount(usize),
}

impl fmt::Display for LogFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOError(err) => write!(f, "{}", err),
            Self::RonSpanned(err) => write!(f, "{}", err),
            Self::TeamCount(err) => write!(f, "Expected two, found: {:?}", err),
        }
    }
}

impl From<SpannedError> for LogFileError {
    fn from(value: SpannedError) -> Self {
        Self::RonSpanned(value)
    }
}

impl From<io::Error> for LogFileError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}
