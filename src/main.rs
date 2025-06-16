use std::path::PathBuf;

use eyre::Context;
use facet::Facet;
use facet_pretty::FacetPretty;
use neo_radiant::{error::Error, showfile::Showfile};

#[derive(Facet)]
struct Args {
    #[facet(positional, optional)]
    showfile_path: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    simple_logger::init_with_level(if cfg!(debug_assertions) {
        log::Level::Debug
    } else {
        log::Level::Warn
    })?;

    let args: Args =
        facet_args::from_std_args().context("Failed to parse command line arguments")?;

    let showfile = match &args.showfile_path {
        Some(path) => Showfile::load(path).context("Failed to load showfile from disk")?,
        None => Showfile::default(),
    };

    eprintln!("{}", showfile.pretty());

    Ok(())
}
