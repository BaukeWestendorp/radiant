use neo_radiant::{error::Result, showfile::Showfile};

mod engine;
mod gui;
mod headless;

pub fn run(showfile: Showfile, headless: bool) -> Result<()> {
    if headless {
        headless::run(showfile)?;
    } else {
        gui::run(showfile);
    }

    Ok(())
}
