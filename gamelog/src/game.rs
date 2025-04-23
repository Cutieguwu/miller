use crate::{Event, Play, Quarter, Team, error};
use serde::Deserialize;
use strum::IntoEnumIterator;

#[derive(Debug, Deserialize, Clone)]
pub struct Game {
    pub version: semver::Version,
    pub flags: Vec<Flags>,
    pub events: Vec<Event>,
}

impl Game {
    /// Returns the teams that played.
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

        self.events.iter().for_each(|event| {
            if let Ok(team) = event.team() {
                if !ignore.contains(&team) && !teams.contains(&team) {
                    teams.push(team)
                }
            }
        });

        if teams.len() == 2 || ignore.len() != 0 {
            Ok(teams)
        } else {
            Err(error::TeamsError::NumberFound(teams.len()))
        }
    }

    /// Returns all of the terrain deltas of a team.
    pub fn deltas(&self, team: Team) -> Vec<i8> {
        let events: Vec<Event> = self
            .team_events(team)
            .0
            .iter()
            .filter_map(|event| {
                if let Event::Quarter(_) = event {
                    None
                } else {
                    Some(event.to_owned())
                }
            })
            .collect();
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

    /// Returns all of the plays of a team.
    pub fn team_plays(&self, team: Team) -> TeamPlays {
        TeamPlays(
            self.team_events(team)
                .0
                .iter()
                .filter_map(|event| {
                    if let Event::Play(play) = event {
                        Some(play.to_owned())
                    } else {
                        None
                    }
                })
                .collect::<Vec<Play>>(),
        )
    }

    /// The average number of plays in a quarter.
    pub fn avg_plays_per_quarter(&self, team: Team) -> f32 {
        let periods: Vec<Period> = Quarter::iter()
            .filter_map(|quarter| Some(self.get_period(quarter.to_owned())).to_owned())
            .collect();

        let quarterly_avgs: Vec<f32> = periods
            .iter()
            .filter_map(|period| {
                if !period.is_overtime() {
                    Some(period.team_plays(team.to_owned()) as f32)
                } else {
                    None
                }
            })
            .collect::<Vec<f32>>();

        quarterly_avgs.iter().sum::<f32>() / quarterly_avgs.len() as f32
    }

    /// Returns the average delta of a team.
    pub fn avg_delta(&self, team: Team) -> f32 {
        let deltas = self.deltas(team);

        // Summation doesn't like directly returning f32 from i8.
        deltas.iter().sum::<i8>() as f32 / deltas.len() as f32
    }

    /// Returns the average delta for a team's positive deltas.
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

    /// Returns the average delta for a team's negative deltas.
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

    /// Returns the number of penalties that a team experienced.
    pub fn penalties(&self, team: Team) -> usize {
        self.team_events(team)
            .0
            .iter()
            .filter_map(|event| {
                if let Event::Penalty(_) = event {
                    Some(event.to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<Event>>()
            .len()
    }

    /// Returns the requested quarter.
    pub fn get_period(&self, quarter: Quarter) -> Period {
        let mut record = false;

        Period {
            period: quarter.to_owned(),
            events: self
                .events
                .iter()
                .filter_map(|event| {
                    if let Event::Quarter(_) = event {
                        record = Event::Quarter(quarter.to_owned()) == *event;
                    }

                    if record {
                        return Some(event.to_owned());
                    }

                    None
                })
                .collect::<Vec<Event>>(),
        }
    }

    /// Returns all events relevent to a team's deltas and score.
    pub fn team_events(&self, team: Team) -> TeamEvents {
        let mut events: Vec<Event> = vec![];
        let mut first = true;
        let mut record: bool = true;

        self.events.iter().for_each(|event| {
            if let Event::Kickoff(_) | Event::Turnover(_) = event {
                record = {
                    if team == event.team().unwrap() {
                        // Wipe events vec if the start of quarter was opposition
                        // on offence.
                        if first {
                            events = vec![];
                        }

                        true
                    } else {
                        events.push(event.to_owned());
                        false
                    }
                };

                first = false;
            }

            if record {
                events.push(event.to_owned());
            }
        });

        // If already handled or assumption override applicable
        TeamEvents(events)
    }
}

#[derive(Debug)]
pub struct TeamEvents(pub Vec<Event>);

#[derive(Debug)]
pub struct TeamPlays(pub Vec<Play>);

#[derive(Debug, Clone)]
pub struct Period {
    period: Quarter,
    events: Vec<Event>,
}

impl Period {
    /// Returns all events relevent to a team's deltas and score.
    pub fn team_events(&self, team: Team) -> Vec<Event> {
        let mut events: Vec<Event> = vec![];
        let mut first = true;
        let mut record: bool = true;

        self.events.iter().for_each(|event| {
            if let Event::Kickoff(_) | Event::Turnover(_) = event {
                record = {
                    if team == event.team().unwrap() {
                        // Wipe events vec if the start of quarter was opposition
                        // on offence.
                        if first {
                            events = vec![];
                        }

                        true
                    } else {
                        events.push(event.to_owned());
                        false
                    }
                };

                first = false;
            }

            if record {
                events.push(event.to_owned());
            }
        });

        events
    }

    /// Returns all of the plays of a team.
    pub fn team_plays(&self, team: Team) -> usize {
        self.team_events(team)
            .iter()
            .filter_map(|event| {
                if let Event::Play(_) = event {
                    Some(event)
                } else {
                    None
                }
            })
            .collect::<Vec<&Event>>()
            .len()
    }

    /// Returns true if the current period is overtime.
    pub fn is_overtime(&self) -> bool {
        if let Quarter::Overtime(_) = self.period {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Flags {
    IgnoreActions,
    IgnoreTeam(Team),
    IgnoreScore,
    Interval(u8),
    SheerDumbFuckingLuck,
}

/*
#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn avg_plays_per_quarter() {
        let a = Game {
            version: crate::MIN_VER,
            flags: vec![],
            events: vec![
                Event::Quarter(Quarter::First),
                Event::Kickoff(Team::Nebraska),
                Event::Play(Play::default()),
                Event::Turnover(Team::ArizonaState),
                Event::Quarter(Quarter::Second),
                Event::Turnover(Team::Nebraska),
                Event::Play(Play::default()),
                Event::Play(Play::default()),
                Event::Play(Play::default()),
                Event::Play(Play::default()),
                Event::Play(Play::default()),
                Event::Play(Play::default()),
                Event::Turnover(Team::ArizonaState),
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

    #[test]
    fn team_events() {
        let a = Period {
            start: Quarter::First,
            end: None,
            events: vec![
                Event::Kickoff(Team::Nebraska),
                Event::Play(Play::default()),
                Event::Turnover(Team::ArizonaState),
                Event::Play(Play::default()),
                Event::Play(Play::default()),
                Event::Kickoff(Team::Nebraska),
                Event::Score(ScorePoints::Touchdown),
                Event::Kickoff(Team::SouthCarolina),
            ],
        };

        let b = Period {
            start: Quarter::Second,
            end: None,
            events: vec![
                Event::Play(Play::default()),
                Event::Turnover(Team::SouthCarolina),
            ],
        };

        let c = Period {
            start: Quarter::Second,
            end: None,
            events: vec![
                Event::Play(Play::default()),
                Event::Turnover(Team::Nebraska),
            ],
        };

        let d = Period {
            start: Quarter::Second,
            end: None,
            events: vec![Event::Play(Play::default())],
        };

        assert!(
            a.team_events(Team::Nebraska, None).unwrap()
                == vec![
                    Event::Kickoff(Team::Nebraska),
                    Event::Play(Play::default()),
                    Event::Turnover(Team::ArizonaState),
                    Event::Kickoff(Team::Nebraska),
                    Event::Score(ScorePoints::Touchdown),
                    Event::Kickoff(Team::SouthCarolina),
                ]
        );
        assert!(
            b.team_events(Team::Nebraska, None).unwrap()
                == vec![
                    Event::Play(Play::default()),
                    Event::Turnover(Team::SouthCarolina)
                ]
        );
        assert!(
            c.team_events(Team::Nebraska, None).unwrap() == vec![Event::Turnover(Team::Nebraska)]
        );
        assert!(true == d.team_events(Team::Nebraska, None).is_err());
        assert!(false == d.team_events(Team::Nebraska, Some(true)).is_err())
    }

    #[test]
    fn team_plays() {
        let period = Period {
            start: Quarter::First,
            end: None,
            events: vec![
                Event::Kickoff(Team::Nebraska),
                Event::Play(Play::default()),
                Event::Turnover(Team::ArizonaState),
                Event::Play(Play::default()),
                Event::Play(Play::default()),
                Event::Kickoff(Team::Nebraska),
                Event::Play(Play::default()),
                Event::Score(ScorePoints::default()),
                Event::Kickoff(Team::SouthCarolina),
                Event::Play(Play::default()),
                Event::Turnover(Team::Nebraska),
                Event::Play(Play::default()),
            ],
        };

        assert!(
            period.team_plays(Team::Nebraska, None).unwrap()
                == vec![Play::default(), Play::default(), Play::default()]
        );
    }

    #[test]
    fn quarters() {
        let first = Period {
            start: Quarter::First,
            end: None,
            events: vec![],
        };

        let second_fourth = Period {
            start: Quarter::Second,
            end: Some(Quarter::Fourth),
            events: vec![],
        };

        let third_ot_three = Period {
            start: Quarter::Third,
            end: Some(Quarter::Overtime(3)),
            events: vec![],
        };

        let ot_one_three = Period {
            start: Quarter::Overtime(1),
            end: Some(Quarter::Overtime(3)),
            events: vec![],
        };

        assert!(first.quarters() == vec![Quarter::First]);
        assert!(second_fourth.quarters() == vec![Quarter::Second, Quarter::Third, Quarter::Fourth]);
        assert!(
            third_ot_three.quarters()
                == vec![
                    Quarter::Third,
                    Quarter::Fourth,
                    Quarter::Overtime(1),
                    Quarter::Overtime(2),
                    Quarter::Overtime(3)
                ]
        );
        assert!(
            ot_one_three.quarters()
                == vec![
                    Quarter::Overtime(1),
                    Quarter::Overtime(2),
                    Quarter::Overtime(3)
                ]
        )
    }
}
*/
