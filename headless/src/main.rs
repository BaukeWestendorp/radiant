use std::path::PathBuf;

use clap::Parser;

mod app;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Args {
    showfile_path: PathBuf,
}

fn main() -> Result<(), eyre::Error> {
    let log_level = if cfg!(debug_assertions) { log::Level::Debug } else { log::Level::Warn };
    simple_logger::init_with_level(log_level)?;

    let args = Args::parse();

    app::run(args.showfile_path)?;

    Ok(())
}
