use std::sync::{Arc, Mutex};

use eyre::Context;

use crate::backend::engine::{DMX_OUTPUT_UPDATE_INTERVAL, Engine};
use crate::error::Result;
use crate::showfile::Showfile;

/// Starts the app in headless mode.
pub fn run(showfile: Showfile) -> Result<()> {
    let engine = Arc::new(Mutex::new(Engine::new(showfile).context("Failed to create engine")?));

    let dmx_resolver_handle = std::thread::spawn({
        let engine = engine.clone();
        move || loop {
            engine.lock().unwrap().resolve_dmx();
            spin_sleep::sleep(DMX_OUTPUT_UPDATE_INTERVAL);
        }
    });

    handle_user_interaction();

    dmx_resolver_handle.join().unwrap();

    Ok(())
}

fn handle_user_interaction() {
    todo!("Handle user interaction");
}
