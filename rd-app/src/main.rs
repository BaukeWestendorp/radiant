use std::path::PathBuf;

mod app;
mod comp;
mod engine;
mod util;

#[derive(clap::Parser)]
struct Args {
    /// Path to the showfile.
    showfile_path: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    init_logger();

    let args = <Args as clap::Parser>::parse();

    app::run(args.showfile_path)?;

    Ok(())
}

#[inline]
fn init_logger() {
    let is_debug_mode = cfg!(debug_assertions);
    let default_level =
        if is_debug_mode { log::LevelFilter::Debug } else { log::LevelFilter::Info };
    pretty_env_logger::formatted_builder().filter_level(default_level).parse_env("RUST_LOG").init();
}
