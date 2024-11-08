use app::RadiantApp;
use clap::Parser;
use gpui::App;

mod app;
mod assets;
mod io;
mod view;

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
        let app = RadiantApp::new();
        app.run(cx);
        app.open_show_window(args.path, cx);
    })
}
