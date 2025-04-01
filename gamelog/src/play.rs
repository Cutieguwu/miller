use crate::{Action, TerrainState};
use serde::Deserialize;

pub trait PlayHandle {
    /// Returns all plays within object's scope.
    fn plays(&self) -> Vec<Play>;
}

pub trait Distance {
    fn distance(&self) -> u8;

    fn delta<D: Distance>(&self, d: D);
}

#[derive(Debug, Deserialize, Clone, Default, PartialEq)]
pub struct Play {
    pub action: Action,
    pub down: Option<Down>,
    pub terrain: Option<TerrainState>,
}

impl PlayHandle for Play {
    fn plays(&self) -> Vec<Play> {
        vec![self.to_owned()]
    }
}

impl Distance for Play {
    fn distance(&self) -> u8 {
        todo!()
    }

    fn delta<D: Distance>(&self, d: D) {
        todo!()
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
