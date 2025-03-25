mod calculator;
mod gamelog;

use clap::Parser;
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

    let mut log: LogFile = LogFile::try_from(
        match std::fs::OpenOptions::new() // Defaults to setting all options false.
            .read(true) // Only need ensure that reading is possible.
            .open(&config.logfile_path.as_path())
        {
            Ok(f) => f,
            Err(err) => panic!("Failed to open log file: {:?}", err),
        },
    )
    .expect("Failed to open game log file");

    let log_ver = dbg!(log.get_min_ver());

    if log_ver.cmp_precedence(&GAMELOG_MIN_VER).is_lt() {
        panic!(
            "Error: Log file GameRecord version deviates as low as {:?}, while minimum {:?} is required",
            log_ver.to_string(),
            GAMELOG_MIN_VER.to_string()
        )
    }
}
