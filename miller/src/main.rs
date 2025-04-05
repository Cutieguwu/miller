use clap::Parser;
use core::panic;
use gamelog::{Flags, LogFile};
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

    for game in log.0.iter() {
        if let Ok(teams) = game.teams() {
            for team in teams {
                if !game.flags.contains(&Flags::IgnoreTeam(team.to_owned())) {
                    println!(
                        "{:?}: {:?}",
                        &team,
                        game.avg_plays_per_quarter(team.to_owned())
                    )
                }
            }
        }
    }
}
