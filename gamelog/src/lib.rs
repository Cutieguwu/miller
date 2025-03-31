#![allow(deprecated)]

mod action;
mod error;
mod event;
mod file;
mod game;
mod period;
mod play;
mod score;
mod terrain;

#[allow(unused)]
pub const MIN_VER: semver::Version = semver::Version::new(0, 3, 0);

pub use action::*;
pub use event::Event;
pub use file::LogFile;
pub use game::{Game, GameRecord};
pub use period::*;
pub use play::*;
pub use score::ScoreChange;
pub use terrain::TerrainState;
