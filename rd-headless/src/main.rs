use std::path::PathBuf;

use clap::Parser;
use rd_engine::Engine;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the showfile to load.
    showfile_path: PathBuf,
}

fn init_logger() {
    let is_debug_mode = cfg!(debug_assertions);
    let default_level =
        if is_debug_mode { log::LevelFilter::Debug } else { log::LevelFilter::Info };
    pretty_env_logger::formatted_builder().filter_level(default_level).parse_env("RUST_LOG").init();
}

fn main() -> anyhow::Result<()> {
    init_logger();

    let args = Args::parse();

    let engine = Engine::new(Some(PathBuf::from(args.showfile_path))).unwrap();
    let running = engine.spawn();
    running.join().unwrap();

    Ok(())
}
