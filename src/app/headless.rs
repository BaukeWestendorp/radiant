use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use eyre::Context;

use crate::backend::{DMX_OUTPUT_UPDATE_INTERVAL, Engine};
use crate::error::Result;
use crate::showfile::Showfile;

/// Starts the app in headless mode.
pub fn run(showfile: Showfile) -> Result<()> {
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

    _engine.lock().unwrap().exec_cmd(crate::cmd!("executor 1 button press")).unwrap();
    _engine.lock().unwrap().exec_cmd(crate::cmd!("executor 1 button release")).unwrap();
    thread::sleep(Duration::from_millis(200));
    _engine.lock().unwrap().exec_cmd(crate::cmd!("executor 1 button press")).unwrap();
    _engine.lock().unwrap().exec_cmd(crate::cmd!("executor 1 button release")).unwrap();

    thread::sleep(Duration::from_millis(500));
    _engine.lock().unwrap().exec_cmd(crate::cmd!("executor 1 fader level 0.25")).unwrap();
    thread::sleep(Duration::from_millis(500));
    _engine.lock().unwrap().exec_cmd(crate::cmd!("executor 1 fader level 0.50")).unwrap();
    thread::sleep(Duration::from_millis(500));
    _engine.lock().unwrap().exec_cmd(crate::cmd!("executor 1 fader level 0.75")).unwrap();
}
