use crate::{Action, Event, Game, Play, Team, TeamEvents, TerrainState, error};
use serde::Deserialize;
use std::{fs::File, path::PathBuf, usize};
use strum::IntoEnumIterator;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct LogFile(pub Vec<Game>);

impl LogFile {
    /// Returns the most common action for a given team.
    pub fn most_frequent_action(&self, team: Team) -> Action {
        let mut most_freq_action = Action::default();
        let mut frequency = usize::MIN;
        let mut found = usize::MIN;
        let team_actions = self.team_actions(team).into_iter();

        Action::iter()
            .filter(|action| *action != Action::Unknown)
            .for_each(|action| {
                found = team_actions.clone().filter(|a| *a == action).count();

                if found > frequency {
                    frequency = found;
                    most_freq_action = action.to_owned();
                }
            });

        most_freq_action
    }

    /// Returns the least common action for a given team.
    /// This action has to have been played at least once.
    pub fn least_frequent_action(&self, team: Team) -> Action {
        let mut least_freq_action = Action::default();
        let mut frequency = usize::MAX;
        let mut found = usize::MAX;
        let team_actions = self.team_actions(team).into_iter();

        Action::iter()
            .filter(|action| *action != Action::Unknown)
            .for_each(|action| {
                found = team_actions.clone().filter(|a| *a == action).count();

                if (found != 0_usize) && (found < frequency) {
                    dbg!("hit");
                    frequency = found;
                    least_freq_action = action.to_owned();
                }
            });

        least_freq_action
    }

    pub fn most_effective_play(&self, team: Team) -> (Action, TerrainState) {
        let deltas: Vec<Vec<i8>> = self
            .0
            .iter()
            .map(|game| game.deltas(team.to_owned()))
            .collect();

        let team_events: Vec<Vec<Event>> = self
            .0
            .iter()
            .filter_map(|game| game.team_events(team.to_owned()))
            .collect::<Vec<TeamEvents>>()
            .iter()
            .map(|team_events| team_events.0.to_owned())
            .collect::<Vec<Vec<Event>>>();

        let mut action_return = Action::Unknown;
        let mut terrain_delta: u8 = 0;

        let mut action_deltas: Vec<i8>;
        let mut game_idx: usize;
        let mut event_idx: usize;

        for action in Action::iter().filter(|action| *action != Action::Unknown) {
            action_deltas = vec![];
            game_idx = 0;
            event_idx = 0;

            for game in &team_events {
                for _ in game {
                    if let Event::Play(play) = &team_events[game_idx][event_idx] {
                        if play.action == action {
                            action_deltas.push(deltas[game_idx][event_idx]);
                        }
                    }

                    event_idx += 1;
                }

                game_idx += 1;

                if (event_idx + 1) == game.len() {
                    event_idx = 0;
                    continue;
                }
            }

            let sum: u8 = action_deltas.iter().sum::<i8>() as u8;

            if sum > terrain_delta {
                terrain_delta = sum;
                action_return = action.to_owned();
            }
        }

        (action_return, TerrainState::Yards(terrain_delta))
    }

    pub fn check_teams(self) -> Result<LogFile, error::LogFileError> {
        for game in &self.0 {
            if let Err(err) = game.teams() {
                return Err(err);
            }
        }

        Ok(self)
    }

    /// Returns the team actions.
    /// The following code is equivalent to:
    fn team_actions(&self, team: Team) -> Vec<Action> {
        // ```
        // fn foo(&self, team: Team) -> Vec<Action> {
        //     let mut actions = vec![];
        //
        //     for game in &self.0 {
        //         for play in game.team_plays(team.to_owned()).0 {
        //             actions.push(play.action.to_owned())
        //         }
        //     }
        //
        //     actions
        // }
        // ```
        // I just write iterators more naturally for some reason
        // despite them being less readable afterward. I came from
        // loving python's list comprehensions.
        // I suppose I like the lack of nesting.
        //
        // I have no clue if the iterator is actually more efficient.
        self.0
            .iter()
            .filter_map(|game| Some(game.team_plays(team.to_owned()).0))
            .collect::<Vec<Vec<Play>>>()
            .concat()
            .iter()
            .filter_map(|play| Some(play.action.to_owned()))
            .collect::<Vec<Action>>()
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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn most_frequent_action() {
        let a = LogFile(vec![Game {
            events: vec![
                Event::Kickoff(Team::Nebraska),
                Event::Play(Play {
                    action: Action::Mesh,
                    ..Default::default()
                }),
                Event::Play(Play {
                    action: Action::Mesh,
                    ..Default::default()
                }),
                Event::Play(Play {
                    action: Action::Mesh,
                    ..Default::default()
                }),
                Event::Play(Play {
                    action: Action::Curls,
                    ..Default::default()
                }),
                Event::Play(Play {
                    action: Action::Curls,
                    ..Default::default()
                }),
                Event::Play(Play {
                    action: Action::SlotOut,
                    ..Default::default()
                }),
                Event::Kickoff(Team::ArizonaState),
            ],
            ..Default::default()
        }]);

        assert!(a.most_frequent_action(Team::Nebraska) == Action::Mesh)
    }

    #[test]
    fn least_frequent_action() {
        let a = LogFile(vec![Game {
            events: vec![
                Event::Kickoff(Team::Nebraska),
                Event::Play(Play {
                    action: Action::Mesh,
                    ..Default::default()
                }),
                Event::Play(Play {
                    action: Action::Mesh,
                    ..Default::default()
                }),
                Event::Play(Play {
                    action: Action::Mesh,
                    ..Default::default()
                }),
                Event::Play(Play {
                    action: Action::Curls,
                    ..Default::default()
                }),
                Event::Play(Play {
                    action: Action::Curls,
                    ..Default::default()
                }),
                Event::Play(Play {
                    action: Action::SlotOut,
                    ..Default::default()
                }),
                Event::Kickoff(Team::ArizonaState),
            ],
            ..Default::default()
        }]);

        assert!(a.least_frequent_action(Team::Nebraska) == Action::SlotOut)
    }
}
