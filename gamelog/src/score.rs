use crate::Team;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ScoreChange {
    pub team: Team,
    pub score: u8,
}
