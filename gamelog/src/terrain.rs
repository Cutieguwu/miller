use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum TerrainState {
    Yards(u8),
    GoalLine,
    Inches,
}

impl Default for TerrainState {
    fn default() -> Self {
        TerrainState::Yards(10)
    }
}
