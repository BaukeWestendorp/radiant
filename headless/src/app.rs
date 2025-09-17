use std::path::PathBuf;

use radlib::engine::Engine;

/// Starts the app in headless mode.
pub fn run(showfile_path: PathBuf) -> eyre::Result<()> {
    let mut engine = Engine::new(Some(showfile_path));
    engine.start()?;
    loop {}
}
