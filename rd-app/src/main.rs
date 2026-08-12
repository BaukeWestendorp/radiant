use std::path::PathBuf;

mod app;
mod comp;

#[inline]
fn init_logger() {
    let is_debug_mode = cfg!(debug_assertions);
    let default_level =
        if is_debug_mode { log::LevelFilter::Debug } else { log::LevelFilter::Info };
    pretty_env_logger::formatted_builder().filter_level(default_level).parse_env("RUST_LOG").init();
}

#[derive(Debug)]
#[derive(facet::Facet)]
struct Cli {
    /// Path to the showfile to load.
    #[facet(figue::positional)]
    showfile_path: Option<PathBuf>,

    /// Adds --help, --version, and --completions
    #[facet(flatten)]
    builtins: figue::FigueBuiltins,
}

fn main() -> anyhow::Result<()> {
    init_logger();

    let cli: Cli = figue::from_std_args().unwrap();

    app::run(cli.showfile_path)?;

    Ok(())
}
