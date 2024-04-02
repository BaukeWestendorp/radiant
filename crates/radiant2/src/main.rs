use std::path::PathBuf;

use app::run_app;
use gpui::App;

mod app;
mod assets;
mod geometry;
mod layout;
mod output;
mod showfile;
mod window;
mod workspace;

use clap::Parser;

/// Radiant is a lighting design software.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the showfile
    #[arg(short, long)]
    showfile: PathBuf,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    let app = App::new();
    let showfile_path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join(args.showfile)
        .canonicalize()
        .unwrap()
        .to_str()
        .map(|p| p.to_string());

    run_app(app, showfile_path)
}
