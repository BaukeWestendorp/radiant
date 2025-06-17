use neo_radiant::{error::Result, showfile::Showfile};

/// The [Engine] controls the flow of output data,
/// and is the interface between the user interface
/// (including a headless app, even if it's a CLI) and
/// the show.
pub struct Engine {
    showfile: Showfile,
}

impl Engine {
    pub fn new(showfile: Showfile) -> Self {
        Self { showfile }
    }

    pub fn run(self) -> Result<()> {
        todo!("start engine");
        Ok(())
    }
}
