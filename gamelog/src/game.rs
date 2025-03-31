use crate::{Event, Period, Play, PlayHandle, Team, error};
use serde::Deserialize;

#[deprecated(since = "0.2.0", note = "Migrated to Game")]
pub type GameRecord = Game;

#[derive(Debug, Deserialize, Clone)]
pub struct Game {
    pub version: semver::Version,
    pub periods: Vec<Period>,
}

impl Game {
    /// Returns the teams of this game.
    pub fn teams(&self) -> Result<Vec<Team>, error::TeamsError> {
        let mut teams = vec![];

        self.periods.iter().for_each(|period| {
            period.events.iter().for_each(|event| {
                if let Event::Kickoff(t) | Event::Turnover(t) = event {
                    if teams.contains(t) {
                        teams.push(t.to_owned())
                    }
                }
            })
        });

        if teams.len() == 2 {
            Ok(teams)
        } else {
            Err(error::TeamsError::NumberFound(teams.len()))
        }
    }
}

impl PlayHandle for Game {
    fn plays(&self) -> Vec<Play> {
        self.periods
            .iter()
            .map(|period| period.plays())
            .collect::<Vec<Vec<Play>>>() // Make compiler happy with turbofish.
            .concat()
    }
}
