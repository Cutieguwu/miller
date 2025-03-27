use std::{fmt, io};

#[derive(Debug)]
pub enum LogFileError {
    FailedToOpen(io::Error),
    RonSpannedError(ron::error::SpannedError),
    CompatibilityCheck(semver::Version),
}

impl fmt::Display for LogFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FailedToOpen(err) => write!(f, "{}", err),
            Self::CompatibilityCheck(ver) => write!(
                f,
                "GameLogs cannot be older than {}, but {} was found in logfile.",
                crate::gamelog::GAMELOG_MIN_VER.to_string(),
                ver.to_string()
            ),
            Self::RonSpannedError(err) => write!(f, "{}", err),
        }
    }
}

pub enum DownError {
    NotKickoff,
}

impl fmt::Display for DownError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NotKickoff => write!(f, "Variant was not Down::Kickoff."),
        }
    }
}
