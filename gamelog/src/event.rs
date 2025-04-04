use crate::{Down, Play, Team, TerrainState};
use serde::Deserialize;

type Offence = Team;

#[derive(Debug, Deserialize, Clone)]
pub enum Event {
    Play(Play),
    Kickoff(Offence),
    Turnover(Offence),
    Penalty(TerrainState),
    Score(ScorePoints),
}

impl Event {
    pub fn delta(&self, following: &Self) -> Option<i8> {
        // Clean this trash spaghetti code up.

        fn make_play(event: &Event) -> Option<Play> {
            match event {
                Event::Kickoff(_) => Some(Play::default()),
                Event::Play(play) => {
                    let p = play.to_owned();

                    if p.down.is_none()
                        || p.terrain.is_none()
                        || p.terrain.as_ref()? == &TerrainState::Unknown
                    {
                        None
                    } else {
                        Some(p)
                    }
                }
                _ => None,
            }
        }

        let preceeding = make_play(self)?;
        let following = make_play(following)?;

        if following.down? == Down::First {
            if let TerrainState::Yards(yrds) = preceeding.terrain? {
                Some(yrds as i8)
            } else {
                None
            }
        } else {
            let a = if let TerrainState::Yards(yrds) = preceeding.terrain? {
                yrds
            } else {
                unreachable!()
            };

            let b = if let TerrainState::Yards(yrds) = following.terrain? {
                yrds
            } else {
                unreachable!()
            };

            Some(a as i8 - b as i8)
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Default)]
pub enum ScorePoints {
    #[default]
    Touchdown,
    FieldGoal,
    Safety,
    PatFail,
    PatTouchdown,
    PatFieldGoal,
    PatSafety,
}

impl ScorePoints {
    pub fn to_points(&self) -> u8 {
        match &self {
            Self::Touchdown => 6,
            Self::FieldGoal => 3,
            Self::Safety => 2,
            Self::PatFail => 0,
            Self::PatTouchdown => 2,
            Self::PatFieldGoal => 1,
            Self::PatSafety => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Action, Down, Team, TerrainState};

    #[test]
    fn delta() {
        let kickoff = Event::Kickoff(Team::Nebraska);
        let first_down = Event::Play(Play {
            action: Action::Unknown,
            down: Some(Down::First),
            terrain: Some(TerrainState::Yards(10)),
        });
        let second_down = Event::Play(Play {
            action: Action::Unknown,
            down: Some(Down::Second),
            terrain: Some(TerrainState::Yards(10)),
        });
        let third_down = Event::Play(Play {
            action: Action::Unknown,
            down: Some(Down::Third),
            terrain: Some(TerrainState::Yards(13)),
        });
        let fourth_down = Event::Play(Play {
            action: Action::Unknown,
            down: Some(Down::Fourth),
            terrain: Some(TerrainState::Yards(5)),
        });
        let penalty = Event::Penalty(TerrainState::Yards(15));
        let turnover = Event::Turnover(Team::Nebraska);
        let noned_down = Event::Play(Play {
            action: Action::Unknown,
            down: None,
            terrain: None,
        });
        let score = Event::Score(ScorePoints::default());
        let goal_line = Event::Play(Play {
            action: Action::Unknown,
            down: Some(Down::First),
            terrain: Some(TerrainState::GoalLine),
        });
        let inches = Event::Play(Play {
            action: Action::Unknown,
            down: Some(Down::First),
            terrain: Some(TerrainState::Inches),
        });

        assert!(10_i8 == kickoff.delta(&first_down).unwrap());
        assert!(0_i8 == first_down.delta(&second_down).unwrap());
        assert!(-3_i8 == second_down.delta(&third_down).unwrap());
        assert!(10_i8 == first_down.delta(&kickoff).unwrap());
        assert!(10_i8 == first_down.delta(&goal_line).unwrap());
        assert!(10_i8 == first_down.delta(&inches).unwrap());
        assert!(13_i8 == third_down.delta(&kickoff).unwrap());
        assert!(8_i8 == third_down.delta(&fourth_down).unwrap());
        assert!(13_i8 == third_down.delta(&goal_line).unwrap());
        assert!(None == fourth_down.delta(&turnover));
        assert!(None == kickoff.delta(&penalty));
        assert!(None == first_down.delta(&penalty));
        assert!(None == noned_down.delta(&kickoff));
        assert!(None == noned_down.delta(&turnover));
        assert!(None == kickoff.delta(&score));
        assert!(None == first_down.delta(&score));
        assert!(None == turnover.delta(&score));
        assert!(None == goal_line.delta(&first_down));
        assert!(None == inches.delta(&first_down));
        assert!(None == goal_line.delta(&inches));
    }
}
