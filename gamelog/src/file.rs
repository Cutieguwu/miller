use crate::error;
use serde::Deserialize;
use std::{fs::File, path::PathBuf};

#[derive(Debug, Deserialize, Clone)]
pub struct LogFile(Vec<super::Game>);

impl LogFile {
    pub fn min_ver(&self) -> semver::Version {
        let mut lowest = semver::Version::new(u64::MAX, u64::MAX, u64::MAX);

        self.0.iter().for_each(|x| {
            if x.version.cmp_precedence(&lowest).is_lt() {
                lowest = x.version.clone()
            }
        });

        lowest
    }

    /// Returns if the LogFile min version is compatible.
    pub fn is_compatible(&self) -> bool {
        self.min_ver().cmp_precedence(&super::MIN_VER).is_lt()
    }
}

impl TryFrom<File> for LogFile {
    type Error = ron::error::SpannedError;

    fn try_from(file: File) -> Result<Self, Self::Error> {
        ron::de::from_reader(file)
    }
}

impl TryFrom<PathBuf> for LogFile {
    type Error = error::LogFileError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        match Self::try_from(
            match std::fs::OpenOptions::new() // Defaults to setting all options false.
                .read(true) // Only need ensure that reading is possible.
                .open(path.as_path())
            {
                Ok(f) => f,
                Err(err) => return Err(error::LogFileError::FailedToOpen(err)),
            },
        ) {
            Ok(f) => Ok(f),
            Err(err) => Err(error::LogFileError::RonSpannedError(err)),
        }
    }
}
