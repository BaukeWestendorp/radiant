use app::RadiantApp;
use clap::Parser;
use std::path::PathBuf;

mod app;
mod layout;
mod pipeline;
mod processor;
mod protocol;
mod show;
pub mod ui;
mod utils;

#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the showfile. Leave empty to open a new showfile.
    #[arg(short, long)]
    showfile: Option<PathBuf>,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    RadiantApp::new(args.showfile).run();
}
