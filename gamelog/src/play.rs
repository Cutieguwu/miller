use crate::{Action, TerrainState};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Play {
    pub action: Action,
    pub down: Option<Down>,
    pub terrain: Option<TerrainState>,
}

impl Default for Play {
    fn default() -> Self {
        Self {
            action: Action::default(),
            down: Some(Down::First),
            terrain: Some(TerrainState::Yards(10)),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default, PartialEq)]
pub enum Down {
    #[default]
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Team {
    ArizonaState,
    #[deprecated(since = "0.2.0", note = "Team left the project.")]
    BoiseState,
    Colorado,
    Iowa,
    Nebraska,
    SouthCarolina,
    Syracuse,
    TexasAnM,
}
