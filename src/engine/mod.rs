use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::backend::pipeline::Pipeline;
use crate::backend::show::Show;
use crate::engine::cmd::Command;
use crate::error::Result;

pub mod cmd;

const DMX_OUTPUT_INTERVAL: Duration = Duration::from_millis(40);

/// The [Engine] controls the flow of output data,
/// and is the interface between the user interface
/// (including a headless app, even if it's a CLI) and
/// the show.
pub struct Engine {
    showfile: Arc<Mutex<Show>>,
    pipeline: Arc<Mutex<Pipeline>>,
    dmx_output_thread: Option<JoinHandle<()>>,
}

impl Engine {
    pub fn new(show: Show) -> Self {
        Self {
            showfile: Arc::new(Mutex::new(show)),
            pipeline: Arc::new(Mutex::new(Pipeline::new())),
            dmx_output_thread: None,
        }
    }

    /// Starts all threads.
    pub fn run(&mut self) -> Result<()> {
        self.start_dmx_output_thread();
        Ok(())
    }

    fn start_dmx_output_thread(&mut self) {
        let handle = thread::spawn({
            let pipeline = self.pipeline.clone();
            let showfile = self.showfile.clone();
            move || loop {
                {
                    let mut pipeline = pipeline.lock().unwrap();

                    pipeline.resolve(&showfile.lock().unwrap().patch);

                    let multiverse = pipeline.output_multiverse();

                    eprintln!("{multiverse:?}");
                }

                thread::yield_now();
                thread::sleep(DMX_OUTPUT_INTERVAL);
            }
        });
        self.dmx_output_thread = Some(handle);
        log::info!("Started DMX Output thread");
    }

    /// Execute a [Command] to interface with the backend.
    pub fn execute_command(&mut self, command: Command) {
        match command {
            Command::SetDmxValue(address, value) => {
                self.pipeline.lock().unwrap().set_dmx_value(address, value);
            }
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        if let Some(handle) = self.dmx_output_thread.take() {
            handle.join().unwrap();
        }
    }
}
