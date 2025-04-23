use crate::{Action, Team, error};
use serde::Deserialize;
use std::{fs::File, path::PathBuf};
use strum::IntoEnumIterator;

#[derive(Debug, Deserialize, Clone)]
pub struct LogFile(pub Vec<super::Game>);

impl LogFile {
    /// Returns the most common action for a given team.
    pub fn most_frequent_action(&self, team: Team) -> Action {
        let mut most_common_action = Action::Unknown;
        let mut frequency = 0;

        for action in Action::iter() {
            if action == Action::Unknown {
                continue;
            }

            let found = self
                .0
                .iter()
                .filter_map(|game| {
                    Some(
                        game.team_plays(team.to_owned())
                            .0
                            .iter()
                            .filter_map(|play| {
                                if play.action == action {
                                    Some(())
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<()>>()
                            .len(),
                    )
                })
                .sum::<usize>();

            if found > frequency {
                frequency = found;
                most_common_action = action.to_owned();
            }
        }

        most_common_action
    }
}

impl TryFrom<File> for LogFile {
    type Error = ron::error::SpannedError;

    fn try_from(file: File) -> Result<Self, Self::Error> {
        ron::Options::default()
            .with_default_extension(ron::extensions::Extensions::EXPLICIT_STRUCT_NAMES)
            .from_reader(file)
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
