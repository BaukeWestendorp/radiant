use eyre::Context;

use crate::backend::engine::Engine;
use crate::error::Result;
use crate::showfile::Showfile;

/// Starts the app in headless mode.
pub fn run(showfile: Showfile) -> Result<()> {
    let mut engine = Engine::new(showfile).context("Failed to create engine")?;
    engine.start().context("Failed to start engine")?;

    Ok(())
}
