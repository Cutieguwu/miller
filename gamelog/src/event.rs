use crate::{Play, Team, TerrainState};

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
