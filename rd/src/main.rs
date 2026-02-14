use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the showfile to load.
    showfile_path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let engine = rd_core::Engine::new(args.showfile_path)?;

    engine.start();

    std::thread::sleep(std::time::Duration::MAX);

    Ok(())
}
