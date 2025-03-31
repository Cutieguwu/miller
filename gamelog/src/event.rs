use crate::{Play, ScoreChange, Team, TerrainState};

use serde::Deserialize;

type Offence = Team;

#[derive(Debug, Deserialize, Clone)]
pub enum Event {
    Play(Play),
    Kickoff(Offence),
    Turnover(Offence),
    Penalty(TerrainState),
    ScoreChange(ScoreChange),
}
