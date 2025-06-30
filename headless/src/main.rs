use std::path::PathBuf;

use eyre::Context;
use radiant::showfile::Showfile;

mod app;

#[derive(facet::Facet)]
struct Args {
    #[facet(positional, optional)]
    showfile_path: Option<PathBuf>,
}

fn main() -> Result<(), eyre::Error> {
    let log_level = if cfg!(debug_assertions) { log::Level::Debug } else { log::Level::Warn };
    simple_logger::init_with_level(log_level)?;

    let args: Args = facet_args::from_std_args().wrap_err("failed to parse arguments")?;

    let showfile = match &args.showfile_path {
        Some(path) => Showfile::load(path).wrap_err("failed to load showfile from disk")?,
        None => Showfile::default(),
    };

    app::run(showfile)?;

    Ok(())
}
