use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default, PartialEq)]
pub enum TerrainState {
    Yards(u8),
    GoalLine,
    Inches,
    #[default]
    Unknown,
}
