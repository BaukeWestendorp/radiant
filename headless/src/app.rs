use std::path::PathBuf;

use radiant::engine::Engine;

/// Starts the app in headless mode.
pub fn run(showfile_path: PathBuf) -> eyre::Result<()> {
    let mut engine = Engine::new(showfile_path);
    engine.start()?;
    loop {}
}
