use std::sync::{Arc, Mutex};
use std::thread;

use eyre::Context;

use radiant::showfile::Showfile;
use radiant::{DMX_OUTPUT_UPDATE_INTERVAL, Engine};

/// Starts the app in headless mode.
pub fn run(showfile: Showfile) -> eyre::Result<()> {
    let engine = Arc::new(Mutex::new(Engine::new(showfile).wrap_err("failed to create engine")?));

    let dmx_resolver_handle = thread::spawn({
        let engine = engine.clone();
        move || loop {
            engine.lock().unwrap().resolve_dmx();
            spin_sleep::sleep(DMX_OUTPUT_UPDATE_INTERVAL);
        }
    });

    handle_user_interaction(engine.clone());

    dmx_resolver_handle.join().unwrap();

    Ok(())
}

fn handle_user_interaction(_engine: Arc<Mutex<Engine>>) {
    // FIXME: Implement
}
