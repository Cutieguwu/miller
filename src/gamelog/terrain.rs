use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum TerrainState {
    Yards(u8),
    GoalLine,
    Inches,
}
