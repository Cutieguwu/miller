use serde::Deserialize;

#[deprecated(since = "0.2.0", note = "Migrated to Game")]
type GameRecord = Game;

#[derive(Debug, Deserialize, Clone)]
pub struct Game {
    pub version: semver::Version,
    periods: Vec<Option<Period>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Period {
    start: Quarter,
    end: Option<Quarter>,
    plays: Vec<super::Play>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Quarter {
    First,
    Second,
    Third,
    Fourth,
    Overtime(u8),
}
