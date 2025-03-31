use crate::{Action, TerrainState};
use serde::Deserialize;

pub trait PlayHandle {
    /// Returns all plays within object's scope.
    fn plays(&self) -> Vec<Play>;
}

#[derive(Debug, Deserialize, Clone, Default, PartialEq)]
pub struct Play {
    pub action: Action,
    pub down: Down,
    pub terrain: TerrainState,
}

impl PlayHandle for Play {
    fn plays(&self) -> Vec<Play> {
        vec![self.to_owned()]
    }
}

#[derive(Debug, Deserialize, Clone, Default, PartialEq)]
pub enum Down {
    #[default]
    First,
    Second,
    Third,
    Fourth,
    PointAfterTouchdown(Option<u8>),
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
