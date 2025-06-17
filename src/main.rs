use std::path::PathBuf;

use eyre::Context;
use facet::Facet;
use neo_radiant::{app, error::Error, showfile::Showfile};

#[derive(Facet)]
struct Args {
    #[facet(positional, optional)]
    showfile_path: Option<PathBuf>,

    #[facet(named, short = "h")]
    headless: bool,
}

fn main() -> Result<(), Error> {
    let log_level = if cfg!(debug_assertions) { log::Level::Debug } else { log::Level::Warn };
    simple_logger::init_with_level(log_level)?;

    let args: Args = facet_args::from_std_args().context("Failed to parse arguments")?;

    let showfile = match &args.showfile_path {
        Some(path) => Showfile::load(path).context("Failed to load showfile from disk")?,
        None => Showfile::default(),
    };

    app::run(showfile, args.headless)?;

    Ok(())
}
