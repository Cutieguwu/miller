use semver;
use serde::Deserialize;
use std::{fmt, fs::File, io, path::PathBuf, u64};

pub const GAMELOG_MIN_VER: semver::Version = semver::Version::new(0, 2, 0);

#[derive(Debug)]
pub enum LogFileError {
    FailedToOpen(io::Error),
    RonSpannedError(ron::error::SpannedError),
    CompatibilityCheck,
}

impl fmt::Display for LogFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FailedToOpen(err) => write!(f, "{}", err),
            Self::CompatibilityCheck => write!(f, "Variant was not Down::Kickoff."),
            Self::RonSpannedError(err) => write!(f, "{}", err),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LogFile(Vec<GameRecord>);

impl TryFrom<File> for LogFile {
    type Error = ron::error::SpannedError;

    fn try_from(file: File) -> Result<Self, Self::Error> {
        ron::de::from_reader(file)
    }
}

impl TryFrom<PathBuf> for LogFile {
    type Error = LogFileError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        match Self::try_from(
            match std::fs::OpenOptions::new() // Defaults to setting all options false.
                .read(true) // Only need ensure that reading is possible.
                .open(path.as_path())
            {
                Ok(f) => f,
                Err(err) => return Err(LogFileError::FailedToOpen(err)),
            },
        ) {
            Ok(f) => Ok(f),
            Err(err) => Err(LogFileError::RonSpannedError(err)),
        }
    }
}

impl LogFile {
    pub fn get_min_ver(&mut self) -> semver::Version {
        let mut lowest = semver::Version::new(u64::MAX, u64::MAX, u64::MAX);

        self.0.iter().for_each(|x| {
            if x.version.cmp_precedence(&lowest).is_lt() {
                lowest = x.version.clone()
            }
        });

        lowest
    }

    /// Returns if the LogFile min version is compatible.
    fn is_compatible(&mut self) -> bool {
        self.get_min_ver().cmp_precedence(&GAMELOG_MIN_VER).is_lt()
    }

    /// Attempts to make a gamefile compatible.
    pub fn make_compatible(&mut self) -> Result<&mut Self, LogFileError> {
        todo!()
    }

    /// Ensures that the returned gamefile is compatible, else returns Error.
    pub fn ensure_compatible(&mut self) -> Result<&mut Self, LogFileError> {
        if self.is_compatible() {
            Ok(self)
        } else {
            Err(LogFileError::CompatibilityCheck)
        }
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
    Yards(u8),
    GoalLine,
    Inches,
}

#[derive(Debug, Deserialize)]
struct Play {
    action: Option<Action>,
    down: Down,
    terrain: TerrainState,
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
