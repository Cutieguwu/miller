use clap::{ArgAction, Parser};
use core::panic;
use gamelog::{Action, Flags, Key, LogFile, Team};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    /// Path to source file or block device
    #[arg(
        short,
        long,
        value_hint = clap::ValueHint::DirPath,
        default_value = format!("{}/../templates/logfile.ron", std::env::current_dir()
            .expect("Failed to get current working dir.")
            .into_os_string()
            .to_str()
            .unwrap())
    )]
    logfile_path: PathBuf,

    // Behaviour is backwards.
    // ArgAction::SetFalse by default evaluates to true,
    // ArgAction::SetTrue by default evaluates to false.
    #[arg(short, long, action=ArgAction::SetFalse)]
    display_results: bool,
}

fn main() {
    let config = Args::parse();

    let log: LogFile = match LogFile::try_from(config.logfile_path) {
        Ok(f) => f,
        Err(err) => panic!("Error: Failed to open logfile: {:?}", err),
    };

    let mut stats = vec![
        TeamStats::new(Team::ArizonaState),
        #[allow(deprecated)]
        TeamStats::new(Team::BoiseState),
        TeamStats::new(Team::Colorado),
        TeamStats::new(Team::Iowa),
        TeamStats::new(Team::Nebraska),
        TeamStats::new(Team::Syracuse),
        TeamStats::new(Team::SouthCarolina),
        TeamStats::new(Team::TexasAnM),
    ];

    // Work on knocking down the nesting here?
    for game in log.0.iter() {
        let teams = match game.teams() {
            Ok(teams) => teams,
            Err(_) => continue,
        };

        for team in teams {
            // Skip team if they are to be ignored this game.
            if game.flags.contains(&Flags::IgnoreTeam(team.to_owned())) {
                continue;
            }

            let team_idx = stats
                .iter()
                .position(|stat| stat.team == team.to_owned())
                .unwrap();

            stats[team_idx]
                .avg_terrain_gain
                .push(game.avg_gain(team.to_owned()));

            stats[team_idx]
                .avg_terrain_loss
                .push(game.avg_loss(team.to_owned()));

            stats[team_idx]
                .avg_terrain_delta
                .push(game.avg_delta(team.to_owned()));

            stats[team_idx]
                .plays_per_quarter
                .push(game.avg_plays_per_quarter(team.to_owned()));

            stats[team_idx]
                .plays_per_game
                .push(game.team_plays(team.to_owned()));

            stats[team_idx]
                .penalties_per_game
                .push(game.penalties(team.to_owned()));
        }
    }

    if config.display_results {
        // :#? for pretty-printing.
        stats.iter().for_each(|team| println!("{:#?}", team));
    }
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
        }
    }
}
