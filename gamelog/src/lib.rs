mod action;
mod error;
mod event;
mod file;
mod game;
mod period;
#[allow(deprecated)]
mod play;
mod terrain;

#[allow(unused)]
pub const MIN_VER: semver::Version = semver::Version::new(0, 6, 0);

// I'm lazy.
pub use action::*;
pub use event::*;
pub use file::*;
pub use game::*;
pub use period::*;
pub use play::*;
pub use terrain::*;
