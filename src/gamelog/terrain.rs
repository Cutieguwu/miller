use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum TerrainState {
    #[deprecated(since = "0.2.0", note = "Replaced in favour of TerrainState::Yards")]
    Distance(u8),
    Yards(u8),
    GoalLine,
    Inches,
}
