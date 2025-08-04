use std::path::PathBuf;

use eyre::Context;

use radiant::engine::Engine;

/// Starts the app in headless mode.
pub fn run(showfile_path: Option<&PathBuf>) -> eyre::Result<()> {
    let mut engine = Engine::new(showfile_path).wrap_err("failed to create engine")?;
    engine.start();
    loop {}
}
