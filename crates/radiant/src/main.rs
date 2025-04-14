use std::path::PathBuf;

use clap::Parser;

mod app;
mod layout;
mod showfile;

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

    let showfile = match args.showfile {
        Some(path) => match showfile::open_from_file(&path) {
            Ok(showfile) => showfile,
            Err(err) => {
                log::error!("Error opening showfile: {}", err);
                std::process::exit(1);
            }
        },
        None => showfile::Showfile::default(),
    };

    app::RadiantApp::new(showfile).run();
}
