use neo_radiant::{error::Result, showfile::Showfile};

use crate::app::engine::Engine;

pub fn run(showfile: Showfile) -> Result<()> {
    Engine::new(showfile).run()
}
