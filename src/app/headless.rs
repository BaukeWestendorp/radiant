use neo_radiant::{error::Result, showfile::Showfile};

use crate::app::engine::Engine;

/// Starts the app in headless mode.
pub fn run(showfile: Showfile) -> Result<()> {
    Engine::new(showfile).run()
}
