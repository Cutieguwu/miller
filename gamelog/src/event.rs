use crate::{Down, Play, Team, TerrainState, error};
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

            Some((a - b) as i8)
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum ScorePoints {
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
