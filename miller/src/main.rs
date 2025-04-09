mod tui;

use clap::{ArgAction, Parser};
use core::panic;
use gamelog::{Action, Down, Key, LogFile, Team};
use std::{io, path::PathBuf, sync::mpsc, thread};
use tui::App;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    /// Path to source file or block device
    #[arg(
        short,
        long,
        value_hint = clap::ValueHint::DirPath,
        default_value = format!("../templates/logfile.ron")
    )]
    logfile_path: PathBuf,

    // Behaviour is backwards.
    // ArgAction::SetFalse by default evaluates to true,
    // ArgAction::SetTrue by default evaluates to false.
    #[arg(short, long, action=ArgAction::SetFalse)]
    display_results: bool,

    #[arg(short, long, action=ArgAction::SetTrue)]
    no_tui: bool,
}

fn main() -> io::Result<()> {
    let config = Args::parse();

    let log: LogFile = match LogFile::try_from(config.logfile_path) {
        Ok(f) => f,
        Err(err) => panic!("Error: Failed to open logfile: {:?}", err),
    };

    if config.no_tui {
        return;
    }

    let mut app = App { exit: false };

    // Enter Raw terminal mode.
    let mut terminal = ratatui::init();

    let (tx, rx) = mpsc::channel::<tui::Event>();

    let tx_input_fetcher = tx.clone();
    thread::spawn(move || tui::input_fetcher(tx_input_fetcher));

    let app_result = app.run(&mut terminal, rx);

    // Exit Raw terminal mode.
    ratatui::restore();

    app_result
}

#[derive(Debug)]
struct TeamStats {
    team: gamelog::Team,
    // Terrain
    avg_terrain_gain: Vec<f32>,
    avg_terrain_loss: Vec<f32>,
    avg_terrain_delta: Vec<f32>,
    // Play rate
    plays_per_quarter: Vec<f32>,
    plays_per_game: Vec<usize>,
    // Penalties
    penalties_per_game: Vec<usize>,
    // Score
    points_per_quarter: Vec<u8>,
    points_per_game: Vec<u8>,
    // Biases
    most_common_play: Option<Action>,
    least_common_play: Option<Action>,
    most_common_key: Option<Key>,
    least_common_key: Option<Key>,
    // Traits
    // Typical number of downs to achieve 10 yards.
    time_to_first_down: Option<Down>,
}

impl TeamStats {
    fn new(team: Team) -> Self {
        TeamStats {
            team,
            avg_terrain_gain: vec![],
            avg_terrain_loss: vec![],
            avg_terrain_delta: vec![],
            plays_per_quarter: vec![],
            plays_per_game: vec![],
            penalties_per_game: vec![],
            points_per_quarter: vec![],
            points_per_game: vec![],
            most_common_play: None,
            least_common_play: None,
            most_common_key: None,
            least_common_key: None,
            time_to_first_down: None,
        }
    }
}
