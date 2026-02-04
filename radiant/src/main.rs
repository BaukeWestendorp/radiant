use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use zeevonk::project::file::ProjectFile;

const ZEEVONK_FOLDER_RELATIVE_PATH: &str = "zv/";

mod app;
mod fixture_table;
mod settings;

#[derive(Parser)]
#[command(name = "radiant", about = "The Radiant CLI")]
struct Args {
    project_path: PathBuf,
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

    let zv_project_file_path = args.project_path.join(ZEEVONK_FOLDER_RELATIVE_PATH);
    let zv_project_file = ProjectFile::load_from_folder(&zv_project_file_path)?;

    app::run(zv_project_file)?;

    Ok(())
}
