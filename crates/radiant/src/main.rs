use std::path::PathBuf;

use clap::Parser;
use radiant::app::RadiantApp;
use radiant::error::Result;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    showfile_path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let log_level = if cfg!(debug_assertions) { log::Level::Debug } else { log::Level::Info };
    simple_logger::init_with_level(log_level)?;

    let Args { showfile_path } = Args::parse();

    RadiantApp::new().run(showfile_path);

    Ok(())
}
