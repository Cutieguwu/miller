use crate::{Action, Play, Team, error};
use serde::Deserialize;
use std::{fs::File, path::PathBuf};
use strum::IntoEnumIterator;

#[derive(Debug, Deserialize, Clone)]
pub struct LogFile(pub Vec<super::Game>);

impl LogFile {
    /// Returns the most common action for a given team.
    pub fn most_frequent_action(&self, team: Team) -> Action {
        let mut most_freq_action = Action::Unknown;
        let mut frequency = 0;

        // The following let statement is equivalent to:
        //
        //  let team_actions = {
        //      let mut actions = vec![];
        //
        //      for game in &self.0 {
        //          for play in game.team_plays(team.to_owned()).0 {
        //              actions.push(play.action.to_owned())
        //          }
        //      }
        //
        //      actions
        //  }
        //  .into_iter();
        //
        // I just write iterators more naturally for some reason
        // despite them being less readable afterward.
        // I suppose I like the lack of nesting.

        let team_actions = self
            .0
            .iter()
            .filter_map(|game| Some(game.team_plays(team.to_owned()).0))
            .collect::<Vec<Vec<Play>>>()
            .concat()
            .iter()
            .filter_map(|play| Some(play.action.to_owned()))
            .collect::<Vec<Action>>()
            .into_iter();

        for action in Action::iter() {
            if action == Action::Unknown {
                continue;
            }

            let found: usize = team_actions.clone().filter(|a| *a == action).count();

            if found > frequency {
                frequency = found;
                most_freq_action = action.to_owned();
            }
        }

        most_freq_action
    }

    pub fn check_teams(self) -> Result<LogFile, error::LogFileError> {
        for game in &self.0 {
            if let Err(err) = game.teams() {
                return Err(err);
            }
        }

        Ok(self)
    }
}

impl TryFrom<File> for LogFile {
    type Error = error::LogFileError;

    fn try_from(file: File) -> Result<LogFile, Self::Error> {
        let file: LogFile = ron::Options::default()
            .with_default_extension(ron::extensions::Extensions::EXPLICIT_STRUCT_NAMES)
            .from_reader(file)?;

        file.check_teams()
    }
}

impl TryFrom<PathBuf> for LogFile {
    type Error = error::LogFileError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(
            std::fs::OpenOptions::new() // Defaults to setting all options false.
                .read(true) // Only need ensure that reading is possible.
                .open(path.as_path())?,
        )
    }
}
