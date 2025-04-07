use crate::{Event, Period, Team, error};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Game {
    pub version: semver::Version,
    pub flags: Vec<Flags>,
    pub periods: Vec<Period>,
}

impl Game {
    /// Returns the teams of this game.
    pub fn teams(&self) -> Result<Vec<Team>, error::TeamsError> {
        let ignore: Vec<Team> = self
            .flags
            .iter()
            .filter_map(|flag| {
                if let Flags::IgnoreTeam(team) = flag {
                    Some(team.to_owned())
                } else {
                    None
                }
            })
            .collect();

        let mut teams = vec![];

        self.periods.iter().for_each(|period| {
            for event in period.events.iter() {
                if let Ok(team) = event.team() {
                    if !ignore.contains(&team) && !teams.contains(&team) {
                        teams.push(team)
                    }
                }
            }
        });

        if teams.len() == 2 || ignore.len() != 0 {
            Ok(teams)
        } else {
            Err(error::TeamsError::NumberFound(teams.len()))
        }
    }

    pub fn deltas(&self, team: Team) -> Vec<i8> {
        let events = self
            .periods
            .iter()
            .filter_map(|period| Some(period.team_events(team.to_owned(), None).ok().unwrap()))
            .collect::<Vec<Vec<Event>>>()
            .concat();
        let len = events.len() - 1;
        let mut idx: usize = 0;
        let mut deltas: Vec<i8> = vec![];

        while idx < len {
            if let Some(value) = events[idx].delta(&events[idx + 1]) {
                deltas.push(value);
            }

            idx += 1
        }

        deltas
    }

    pub fn team_plays(&self, team: Team) -> usize {
        self.periods
            .iter()
            .filter_map(|period| {
                if !period.is_overtime() {
                    let plays = period.team_plays(team.to_owned(), None);
                    Some(plays.unwrap().len())
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>()
            .iter()
            .sum::<usize>()
    }

    /// The average number of plays in a quarter.
    /// Does not include OT plays or quarters where team indeterminate.
    pub fn avg_plays_per_quarter(&self, team: Team) -> f32 {
        // Handle if teams known at start or not override via index calculation of all game events.

        let quarterly_avgs: Vec<f32> = self
            .periods
            .iter()
            .filter_map(|period| {
                if !period.is_overtime() {
                    let plays = period.team_plays(team.to_owned(), None);
                    Some(plays.unwrap().len() as f32 / period.quarters().len() as f32)
                } else {
                    None
                }
            })
            .collect::<Vec<f32>>();

        quarterly_avgs.iter().sum::<f32>() / quarterly_avgs.len() as f32
    }

    pub fn avg_delta(&self, team: Team) -> f32 {
        let deltas = self.deltas(team);

        // Summation doesn't like directly returning f32 from i8.
        deltas.iter().sum::<i8>() as f32 / deltas.len() as f32
    }

    pub fn avg_gain(&self, team: Team) -> f32 {
        let deltas: Vec<u8> = self
            .deltas(team)
            .iter()
            .filter_map(|value| {
                if value.is_positive() {
                    Some(value.to_owned() as u8)
                } else {
                    None
                }
            })
            .collect();

        // Summation doesn't like directly returning f32 from u8.
        deltas.iter().sum::<u8>() as f32 / deltas.len() as f32
    }

    pub fn avg_loss(&self, team: Team) -> f32 {
        let deltas: Vec<i8> = self
            .deltas(team)
            .iter()
            .filter_map(|value| {
                if value.is_negative() {
                    Some(value.to_owned())
                } else {
                    None
                }
            })
            .collect();

        deltas.iter().sum::<i8>() as f32 / deltas.len() as f32
    }

    pub fn penalties(&self, team: Team) -> usize {
        // Knock down nesting?
        self.periods
            .iter()
            .filter_map(|period| {
                Some(
                    period
                        .team_events(team.to_owned(), None)
                        .ok()?
                        .iter()
                        .filter_map(|event| {
                            if let Event::Penalty(_) = event {
                                Some(event.to_owned())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<Event>>(),
                )
            })
            .collect::<Vec<Vec<Event>>>()
            .concat()
            .len()
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Flags {
    IgnoreActions,
    IgnoreTeam(Team),
    IgnoreScore,
    Interval(u8),
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn avg_plays_per_quarter() {
        let a = Game {
            version: crate::MIN_VER,
            flags: vec![],
            periods: vec![
                Period {
                    start: Quarter::First,
                    end: None,
                    events: vec![
                        Event::Kickoff(Team::Nebraska),
                        Event::Play(Play::default()),
                        Event::Turnover(Team::ArizonaState),
                    ],
                },
                Period {
                    start: Quarter::Second,
                    end: Some(Quarter::Fourth),
                    events: vec![
                        Event::Turnover(Team::Nebraska),
                        Event::Play(Play::default()),
                        Event::Play(Play::default()),
                        Event::Play(Play::default()),
                        Event::Play(Play::default()),
                        Event::Play(Play::default()),
                        Event::Play(Play::default()),
                        Event::Turnover(Team::ArizonaState),
                    ],
                },
            ],
        };

        let b = Game {
            version: crate::MIN_VER,
            flags: vec![],
            periods: vec![Period {
                start: Quarter::Second,
                end: Some(Quarter::Fourth),
                events: vec![
                    Event::Turnover(Team::Nebraska),
                    Event::Play(Play::default()),
                    Event::Turnover(Team::ArizonaState),
                ],
            }],
        };

        assert!(a.avg_plays_per_quarter(Team::Nebraska) == ((1_f32 + 2_f32) / 2_f32));
        assert!(b.avg_plays_per_quarter(Team::Nebraska) == (1_f32 / 3_f32))
    }

    #[test]
    fn team_plays() {
        let a = Game {
            version: crate::MIN_VER,
            flags: vec![],
            periods: vec![
                Period {
                    start: Quarter::First,
                    end: None,
                    events: vec![
                        Event::Kickoff(Team::Nebraska),
                        Event::Play(Play::default()),
                        Event::Play(Play::default()),
                        Event::Play(Play::default()),
                        Event::Play(Play::default()),
                        Event::Play(Play::default()),
                        Event::Play(Play::default()),
                    ],
                },
                Period {
                    start: Quarter::Second,
                    end: Some(Quarter::Fourth),
                    events: vec![
                        Event::Turnover(Team::Nebraska),
                        Event::Play(Play::default()),
                        Event::Play(Play::default()),
                        Event::Play(Play::default()),
                        Event::Play(Play::default()),
                        Event::Play(Play::default()),
                        Event::Play(Play::default()),
                    ],
                },
            ],
        };

        assert!(a.team_plays(Team::Nebraska) == 12_usize)
    }

    #[test]
    #[allow(deprecated)]
    fn teams() {
        let a = Game {
            version: crate::MIN_VER,
            flags: vec![],
            periods: vec![
                Period {
                    start: Quarter::First,
                    end: None,
                    events: vec![Event::Kickoff(Team::Nebraska)],
                },
                Period {
                    start: Quarter::Second,
                    end: Some(Quarter::Fourth),
                    events: vec![
                        Event::Turnover(Team::ArizonaState),
                        Event::Kickoff(Team::Nebraska),
                    ],
                },
            ],
        };

        let b = Game {
            version: crate::MIN_VER,
            flags: vec![],
            periods: vec![
                Period {
                    start: Quarter::First,
                    end: None,
                    events: vec![Event::Kickoff(Team::Nebraska)],
                },
                Period {
                    start: Quarter::Second,
                    end: Some(Quarter::Fourth),
                    events: vec![
                        Event::Turnover(Team::ArizonaState),
                        Event::Kickoff(Team::BoiseState),
                    ],
                },
            ],
        };

        let c = Game {
            version: crate::MIN_VER,
            flags: vec![Flags::IgnoreTeam(Team::Nebraska)],
            periods: vec![
                Period {
                    start: Quarter::First,
                    end: None,
                    events: vec![Event::Kickoff(Team::Nebraska)],
                },
                Period {
                    start: Quarter::Second,
                    end: Some(Quarter::Fourth),
                    events: vec![
                        Event::Turnover(Team::ArizonaState),
                        Event::Kickoff(Team::Nebraska),
                    ],
                },
            ],
        };

        let d = Game {
            version: crate::MIN_VER,
            flags: vec![Flags::IgnoreTeam(Team::Nebraska)],
            periods: vec![Period {
                start: Quarter::First,
                end: None,
                events: vec![Event::Kickoff(Team::Nebraska)],
            }],
        };

        assert!(a.teams().unwrap() == vec![Team::Nebraska, Team::ArizonaState]);
        assert!(b.teams().is_err() == true);
        assert!(c.teams().unwrap() == vec![Team::ArizonaState]);
        assert!(d.teams().unwrap() == vec![]);
    }

    #[test]
    fn deltas() {
        let game = Game {
            version: crate::MIN_VER,
            flags: vec![],
            periods: vec![
                Period {
                    start: Quarter::First,
                    end: None,
                    events: vec![
                        Event::Kickoff(Team::Nebraska),
                        Event::Play(Play {
                            action: Action::Unknown,
                            down: Some(Down::First),
                            terrain: Some(TerrainState::Yards(10)),
                        }),
                        Event::Play(Play {
                            action: Action::Unknown,
                            down: Some(Down::Second),
                            terrain: Some(TerrainState::Yards(13)),
                        }),
                        Event::Play(Play {
                            action: Action::Unknown,
                            down: Some(Down::Third),
                            terrain: Some(TerrainState::Yards(8)),
                        }),
                        Event::Turnover(Team::ArizonaState),
                        Event::Play(Play {
                            action: Action::Unknown,
                            down: Some(Down::First),
                            terrain: Some(TerrainState::Yards(10)),
                        }),
                        Event::Play(Play {
                            action: Action::Unknown,
                            down: Some(Down::Second),
                            terrain: Some(TerrainState::Yards(10)),
                        }),
                        Event::Turnover(Team::Nebraska),
                        Event::Play(Play {
                            action: Action::Unknown,
                            down: Some(Down::Second),
                            terrain: Some(TerrainState::Yards(12)),
                        }),
                    ],
                },
                Period {
                    start: Quarter::Second,
                    end: None,
                    events: vec![
                        Event::Play(Play {
                            action: Action::Unknown,
                            down: Some(Down::First),
                            terrain: Some(TerrainState::Yards(10)),
                        }),
                        Event::Turnover(Team::ArizonaState),
                    ],
                },
            ],
        };

        assert!(game.deltas(Team::Nebraska) == vec![10_i8, -3_i8, 5_i8, -2_i8, 12_i8]);
        assert!(game.deltas(Team::ArizonaState) == vec![10_i8, 0_i8]);
    }
}
