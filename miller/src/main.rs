use clap::Parser;
use core::panic;
use gamelog::{Action, Flags, LogFile, Team};
use std::path::PathBuf;

#[derive(Debug, Parser)]
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

    for game in log.0.iter() {
        if let Ok(teams) = game.teams() {
            for team in teams {
                if !game.flags.contains(&Flags::IgnoreTeam(team.to_owned())) {
                    // Team is to have their stats recorded this game of file.
                    let team_idx = stats
                        .iter()
                        .position(|stat| {
                            if stat.team == team.to_owned() {
                                true
                            } else {
                                false
                            }
                        })
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
                }
            }
        }
    }

    for team in stats {
        dbg!(team);
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
    penalties_per_game: Vec<u8>,
    // Score
    points_per_quarter: Vec<u8>,
    points_per_game: Vec<u8>,
    // Biases
    most_common_play: Option<Action>,
    least_common_play: Option<Action>,
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
        }
    }
}
