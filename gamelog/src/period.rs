use crate::{Event, Play, Team, error};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Period {
    pub start: Quarter,
    pub end: Option<Quarter>,
    pub events: Vec<Event>,
}

impl Period {
    pub fn team_events(
        &self,
        team: Team,
        assume_team_known: Option<bool>,
    ) -> Result<Vec<Event>, error::CannotDetermineTeams> {
        let mut events: Vec<Event> = vec![];
        let mut first = true;
        let mut record: bool = true;
        let assume_team_known = assume_team_known.unwrap_or(false);

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
        if !first || (first && assume_team_known) {
            Ok(events)
        } else {
            Err(error::CannotDetermineTeams)
        }
    }

    pub fn team_plays(
        &self,
        team: Team,
        assume_team_known: Option<bool>,
    ) -> Result<Vec<Play>, error::CannotDetermineTeams> {
        Ok(self
            .team_events(team, assume_team_known)?
            .iter()
            .filter_map(|event| {
                if let Event::Play(play) = event {
                    Some(play.to_owned())
                } else {
                    None
                }
            })
            .collect())
    }

    pub fn quarters(&self) -> Vec<Quarter> {
        let mut quarters: Vec<Quarter> = vec![self.start.to_owned()];

        if self.end.is_none() {
            return quarters;
        }

        let order = vec![
            Quarter::First,
            Quarter::Second,
            Quarter::Third,
            Quarter::Fourth,
        ];

        let start = if let Quarter::Overtime(x) = self.start {
            (3 + x) as usize
        } else {
            order.iter().position(|q| q == &self.start).unwrap()
        };

        let end = if let Quarter::Overtime(x) = self.end.as_ref().unwrap() {
            (3 + x) as usize
        } else {
            order
                .iter()
                .position(|q| q == self.end.as_ref().unwrap())
                .unwrap()
        };

        let range: Vec<usize> = ((start + 1)..=end).collect();

        for i in range {
            quarters.push(match i {
                0 => Quarter::First,
                1 => Quarter::Second,
                2 => Quarter::Third,
                3 => Quarter::Fourth,
                _ => Quarter::Overtime((i - 3) as u8),
            });
        }

        quarters
    }

    pub fn is_overtime(&self) -> bool {
        self.start.is_overtime() || self.end.as_ref().is_some_and(|some| some.is_overtime())
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Quarter {
    First,
    Second,
    Third,
    Fourth,
    Overtime(u8),
}

impl Quarter {
    pub fn is_overtime(&self) -> bool {
        if let Self::Overtime(_) = self {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

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
