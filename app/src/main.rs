use std::path::PathBuf;

use app::error::Result;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    showfile_path: PathBuf,
}

fn main() -> Result<()> {
    let log_level = if cfg!(debug_assertions) { log::Level::Debug } else { log::Level::Info };
    simple_logger::init_with_level(log_level)?;

    let Args { showfile_path } = Args::parse();

    app::app::run(showfile_path);

    Ok(())
}
