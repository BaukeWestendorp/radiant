use std::path::PathBuf;

use clap::Parser;
use eyre::Context;
use radiant::showfile::Showfile;

mod app;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Args {
    showfile_path: Option<PathBuf>,
}

fn main() -> Result<(), eyre::Error> {
    let log_level = if cfg!(debug_assertions) { log::Level::Debug } else { log::Level::Warn };
    simple_logger::init_with_level(log_level)?;

    let args = Args::parse();

    let showfile = match &args.showfile_path {
        Some(path) => Showfile::load(path).wrap_err("failed to load showfile from disk")?,
        None => Showfile::default(),
    };

    app::run(showfile)?;

    Ok(())
}
