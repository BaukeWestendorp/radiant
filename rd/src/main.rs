use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use rd_core::object::{Effect, FixtureCollection, Object, ObjectKind, ObjectReference, SlotId};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the showfile to load.
    showfile_path: Option<PathBuf>,
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

    let engine = rd_core::Engine::new(args.showfile_path)?;

    engine.start();

    std::thread::sleep(std::time::Duration::MAX);

    Ok(())
}
