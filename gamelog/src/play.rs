use crate::{TerrainState, error};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Play {
    down: Down,
    terrain: TerrainState,
}

type Offence = Team;
impl Offence {}

#[derive(Debug, Deserialize, Clone)]
pub enum Event {
    CrackStudentBodyRightTackle(Play),
    Curls(Play),
    FleaFlicker(Play),
    HalfbackSlam(Play),
    HalfbackSlipScreen(Play),
    HalfbackSweep(Play),
    Mesh(Play),
    PlayActionBoot(Play),
    PlayActionComebacks(Play),
    PlayActionPowerZero(Play),
    PowerZero(Play),
    SlantBubble(Play),
    SlotOut(Play),
    SpeedOption(Play),
    StrongFlood(Play),
    Unknown(Play),
    Kickoff { offence: Team },
    Turnover { offence: Team },
    Penalty { terrain: TerrainState },
}

#[derive(Debug, Deserialize, Clone)]
pub enum Down {
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
    #[deprecated(since = "0.2.0", note = "Team left the project.")]
    BoiseState,
    Colorado,
    Iowa,
    Nebraska,
    SouthCarolina,
    Syracuse,
    TexasAnM,
}
