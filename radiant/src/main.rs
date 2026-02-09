use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

use crate::showfile::Showfile;

mod app;
mod object;
mod show;
mod showfile;

/// The Radiant CLI.
#[derive(Parser)]
#[command(name = "radiant", about = "The Radiant CLI")]
struct Args {
    /// Path to the showfile.
    showfile_path: PathBuf,
}

fn init_logger() {
    let is_debug_mode = cfg!(debug_assertions);
    let default_level =
        if is_debug_mode { log::LevelFilter::Debug } else { log::LevelFilter::Info };
    pretty_env_logger::formatted_builder().filter_level(default_level).parse_env("RUST_LOG").init();
}

fn main() -> Result<()> {
    init_logger();

    let args = Args::parse();

    let showfile =
        Showfile::load_from_folder(&args.showfile_path).context("failed to load showfile")?;

    app::run(showfile)?;

    Ok(())
}
