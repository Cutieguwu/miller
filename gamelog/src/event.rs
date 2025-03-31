use crate::{Play, Team, TerrainState};

use serde::Deserialize;

type Offence = Team;

#[derive(Debug, Deserialize, Clone)]
pub enum Event {
    Play(Play),
    Kickoff(Offence),
    Turnover(Offence),
    Penalty(TerrainState),
    Score(u8),
    Pat(PatPoints),
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum PatPoints {
    Fail,
    One,
    Two,
}
