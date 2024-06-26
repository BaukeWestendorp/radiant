use clap::Parser;
use gpui::App;

mod app;
mod geo;
mod layout;
mod output;
mod showfile;
mod theme;
mod ui;
mod window;
mod workspace;

/// Radiant is a lighting design software.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the showfile
    #[arg(short, long)]
    showfile: Option<std::path::PathBuf>,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    let app = App::new();
    let showfile_path = args.showfile.map(|showfile| {
        std::env::current_dir()
            .expect("Failed to get current directory")
            .join(showfile)
            .canonicalize()
            .expect("Failed to canonicalize showfile path")
    });

    app::run_app(app, showfile_path);
}
