use app::RadiantApp;
use clap::Parser;
use gpui::App;

mod app;
mod assets;
mod dmx_io;
mod ui;
mod workspace;

/// Radiant
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The show file to open.
    #[arg(short = 'f', long = "file")]
    path: Option<std::path::PathBuf>,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    App::new().with_assets(assets::Assets).run(|cx| {
        RadiantApp::new(args.path, cx).unwrap();
    })
}
