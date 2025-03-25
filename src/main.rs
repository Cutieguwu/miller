mod calculator;
mod gamelog;

use clap::Parser;
use gamelog::LogFile;
use std::{fs::OpenOptions, path::PathBuf};

#[derive(Debug, Parser)]
struct Args {
    /// Path to source file or block device
    #[arg(
        short,
        long,
        value_hint = clap::ValueHint::DirPath,
        default_value = std::env::current_dir()
            .expect("Failed to get current working dir.")
            .into_os_string()
    )]
    logfile_path: PathBuf,
}

fn main() {
    let config = Args::parse();

    let log: LogFile = LogFile::try_from(
        match OpenOptions::new() // Defaults to setting all options false.
            .read(true)
            .open(&config.logfile_path.as_path())
        {
            Ok(f) => f,
            Err(err) => panic!("Failed to open log file: {:?}", err),
        },
    )
    .expect("Failed to open game log file");
}
