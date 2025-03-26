mod calculator;
mod gamelog;

use clap::Parser;
use core::panic;
use gamelog::{GAMELOG_MIN_VER, LogFile};
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Args {
    /// Path to source file or block device
    #[arg(
        short,
        long,
        value_hint = clap::ValueHint::DirPath,
        default_value = dbg!(format!("{}/templates/logfile.ron", std::env::current_dir()
            .expect("Failed to get current working dir.")
            .into_os_string().to_str().unwrap()))
    )]
    logfile_path: PathBuf,
}

fn main() {
    let config = Args::parse();

    let mut log: LogFile = {
        let file = match LogFile::try_from(config.logfile_path) {
            Ok(f) => f,
            Err(err) => panic!("Error: Failed to open logfile: {:?}", err),
        };

        match file.ensure_compatible() {
            Ok(f) => f.try_into().expect(msg),
            Err(err) => panic!("Error: Failed to ensure logfile compatibility: {:?}", err),
        }
    };
}
