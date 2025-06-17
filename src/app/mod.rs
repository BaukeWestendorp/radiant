use crate::{backend::show::Show, error::Result};

mod gui;
mod headless;

/// Runs the app, specifying it's behaviour based on
/// the mode it is being run in (Headless or GUI).
pub fn run(show: Show, headless: bool) -> Result<()> {
    if headless {
        headless::run(show)?;
    } else {
        gui::run(show);
    }

    Ok(())
}
