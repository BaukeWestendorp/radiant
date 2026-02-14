use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use rd_core::object::CueList;

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

    dbg!(engine.objects().get_all::<CueList>());

    std::thread::sleep(std::time::Duration::MAX);

    Ok(())
}
