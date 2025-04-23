use serde::Deserialize;
use strum::EnumIter;

#[derive(Debug, Deserialize, Clone, Default, PartialEq, EnumIter)]
pub enum Action {
    CrackStudentBodyRightTackle,
    Curls,
    FleaFlicker,
    HailMary,
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
    #[default]
    Unknown,
}

impl Action {
    // I'd love a better way of doing these
    // Attributes are probably the way to go,
    // but I'm not about to write procedural macros for this project.

    /// Returns `true` if `self` is a play action.
    pub fn is_play_action(&self) -> bool {
        if let Self::PlayActionBoot | Self::PlayActionComebacks | Self::PlayActionPowerZero = self {
            true
        } else {
            false
        }
    }

    /// Returns `true` if `self` is a halfback.
    pub fn is_halfback(&self) -> bool {
        if let Self::HalfbackSlam | Self::HalfbackSlipScreen | Self::HalfbackSweep = self {
            true
        } else {
            true
        }
    }

    /// Returns `true` if `self` is a running play.
    pub fn is_run(&self) -> bool {
        if let Self::HalfbackSlam
        | Self::SpeedOption
        | Self::HalfbackSweep
        | Self::PowerZero
        | Self::CrackStudentBodyRightTackle = self
        {
            true
        } else {
            false
        }
    }

    /// Returns `true` if `self` is a passing play.
    pub fn is_pass(&self) -> bool {
        !self.is_run()
    }

    /// Returns `true` if `self` is `Event::Unknown`.
    pub fn is_unknown(&self) -> bool {
        if let Self::Unknown = self {
            true
        } else {
            false
        }
    }

    /// Returns the `Playset` that this action belongs to.
    /// Returns `None` if `Event::Unknown`
    pub fn playset(&self) -> Option<Playset> {
        if self.is_unknown() {
            return None;
        }

        Some(match self {
            Self::SlantBubble | Self::HalfbackSlam | Self::PlayActionBoot => Playset::PistolSpread,
            Self::StrongFlood | Self::SpeedOption | Self::HalfbackSlipScreen => {
                Playset::ShotgunTripleWingsOffset
            }
            Self::SlotOut | Self::HalfbackSweep | Self::PlayActionComebacks => {
                Playset::ShotgunDoubleFlex
            }
            Self::Curls | Self::PowerZero | Self::PlayActionPowerZero => Playset::IFormNormal,
            Self::Mesh | Self::CrackStudentBodyRightTackle | Self::FleaFlicker => {
                Playset::IFormTight
            }
            _ => unreachable!(),
        })
    }

    /// Returns the `Key` that this action belongs to.
    /// Returns `None` if `Event::Unknown`
    pub fn key(&self) -> Option<Key> {
        if self.is_unknown() {
            return None;
        }

        // All running plays are on `Key::X`
        if self.is_run() {
            return Some(Key::X);
        }

        Some(match self {
            Self::SlantBubble | Self::StrongFlood | Self::SlotOut | Self::Curls | Self::Mesh => {
                Key::Square
            }
            Self::PlayActionBoot
            | Self::HalfbackSlipScreen
            | Self::PlayActionComebacks
            | Self::PlayActionPowerZero
            | Self::FleaFlicker => Key::Triangle,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Playset {
    PistolSpread,
    ShotgunTripleWingsOffset,
    ShotgunDoubleFlex,
    IFormNormal,
    IFormTight,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Key {
    Square,
    X,
    Triangle,
}
