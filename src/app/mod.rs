use neo_radiant::{error::Result, showfile::Showfile};

mod engine;
mod gui;
mod headless;

/// Runs the app, specifying it's behaviour based on
/// the mode it is being run in (Headless or GUI).
pub fn run(showfile: Showfile, headless: bool) -> Result<()> {
    if headless {
        headless::run(showfile)?;
    } else {
        gui::run(showfile);
    }

    Ok(())
}
