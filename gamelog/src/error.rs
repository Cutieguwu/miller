use std::{fmt, io};

#[derive(Debug)]
pub enum LogFileError {
    FailedToOpen(io::Error),
    RonSpannedError(ron::error::SpannedError),
}

impl fmt::Display for LogFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FailedToOpen(err) => write!(f, "{}", err),
            Self::RonSpannedError(err) => write!(f, "{}", err),
        }
    }
}

#[derive(Debug)]
pub enum TeamsError {
    NumberFound(usize),
}

impl fmt::Display for TeamsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NumberFound(err) => write!(f, "Expected two, found: {:?}", err),
        }
    }
}

#[derive(Debug)]
pub struct NoTeamAttribute;

impl fmt::Display for NoTeamAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Object has no team definition.")
    }
}

#[derive(Debug)]
pub struct CannotDetermineTeams;

impl fmt::Display for CannotDetermineTeams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot determine teams present.")
    }
}
