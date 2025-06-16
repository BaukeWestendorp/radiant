use neo_radiant::{error::Result, showfile::Showfile};

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
