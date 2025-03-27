use crate::error;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Play {
    action: Option<Action>,
    down: Down,
    terrain: super::TerrainState,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Action {
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
pub enum Down {
    Kickoff { offence: Team },
    First,
    Second,
    Third,
    Fourth,
    PointAfterTouchdown,
}

impl Down {
    fn get_offence(&self) -> Result<&Team, error::DownError> {
        match self {
            Self::Kickoff { offence } => Ok(offence),
            _ => Err(error::DownError::NotKickoff),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum Team {
    ArizonaState,
    Colorado,
    Iowa,
    Nebraska,
    SouthCarolina,
    Syracuse,
    TexasAnM,
}
