use serde::Deserialize;
use std::{fmt, fs::File};

#[derive(Debug, Deserialize)]
pub struct LogFile(Vec<GameRecord>);

impl TryFrom<File> for LogFile {
    type Error = ron::de::SpannedError;

    fn try_from(file: File) -> Result<Self, Self::Error> {
        ron::de::from_reader(file)
    }
}

#[derive(Debug, Deserialize)]
struct GameRecord {
    version: semver::Version,
    periods: Vec<Option<Period>>,
}

#[derive(Debug, Deserialize)]
struct Period {
    start: Quarter,
    end: Option<Quarter>,
    plays: Vec<Play>,
}

#[derive(Debug, Deserialize)]
enum Quarter {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Debug, Deserialize)]
enum TerrainState {
    Distance(u8),
    GoalLine,
    In,
}

#[derive(Debug, Deserialize)]
struct Play {
    action: Option<Action>,
    down: Down,
    terrain: u8,
}

enum DownError {
    NotKickoff,
}

impl fmt::Display for DownError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NotKickoff => write!(f, "Variant was not Down::Kickoff."),
        }
    }
}

#[derive(Debug, Deserialize)]
enum Down {
    Kickoff { offence: Team },
    First,
    Second,
    Third,
    Fourth,
    PointAfterTouchdown,
}

impl Down {
    fn get_offence(&self) -> Result<&Team, DownError> {
        match self {
            Self::Kickoff { offence } => Ok(offence),
            _ => Err(DownError::NotKickoff),
        }
    }
}

#[derive(Debug, Deserialize)]
enum Action {
    CrackStudentBodyRightTackle,
    Curls,
    FleaFlicker,
    HalfbackSlam,
    HalfbackSlipScreen,
    HalfbackSweep,
    Mesh,
    PlayActionBoot,
    PlayActionComebacks,
    PlayActionPowerZero,
    PowerZero,
    SlantBubble,
    SlotOut,
    SpeedOption,
    StrongFlood,
}

#[derive(Debug, Deserialize, Clone)]
enum Team {
    ArizonaState,
    Colorado,
    Iowa,
    Nebraska,
    SouthCarolina,
    Syracuse,
    TexasAnM,
}
