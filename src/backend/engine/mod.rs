use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use eyre::{Context, ContextCompat};

use crate::backend::engine::cmd::Command;
use crate::backend::patch::fixture::{DmxMode, Fixture, FixtureId};
use crate::backend::pipeline::Pipeline;
use crate::backend::show::Show;
use crate::dmx;
use crate::error::Result;
use crate::showfile::{RELATIVE_GDTF_FILE_FOLDER_PATH, Showfile};

pub mod cmd;

const DMX_OUTPUT_INTERVAL: Duration = Duration::from_millis(40);

/// The [Engine] controls the flow of output data,
/// and is the interface between the user interface
/// (including a headless app, even if it's a CLI) and
/// the show.
pub struct Engine {
    show: Arc<Mutex<Show>>,
    pipeline: Arc<Mutex<Pipeline>>,
    dmx_output_thread: Option<JoinHandle<()>>,
}

impl Engine {
    pub fn new(showfile: Showfile) -> Result<Self> {
        let show = Show::new(showfile.path().cloned());

        let mut this = Self {
            show: Arc::new(Mutex::new(show)),
            pipeline: Arc::new(Mutex::new(Pipeline::new())),
            dmx_output_thread: None,
        };

        // Initialize show.
        for fixture in &showfile.patch.fixtures {
            let id = FixtureId(fixture.id);

            let address = dmx::Address::new(
                dmx::UniverseId::new(fixture.universe)?,
                dmx::Channel::new(fixture.channel)?,
            );

            let dmx_mode = DmxMode::new(fixture.dmx_mode.clone());

            let gdtf_file_name = showfile.patch.gdtf_files.get(fixture.gdtf_file_index).context("Failed to generate patch: Tried to reference GDTF file index that is out of bounds")?.to_string();

            this.execute_command(Command::PatchFixture { id, address, dmx_mode, gdtf_file_name })?;
        }

        this.pipeline.lock().unwrap().clear();

        Ok(this)
    }

    /// Starts all threads.
    pub fn start(&mut self) -> Result<()> {
        self.start_dmx_output_thread();
        Ok(())
    }

    fn start_dmx_output_thread(&mut self) {
        let handle = thread::spawn({
            let pipeline = self.pipeline.clone();
            let showfile = self.show.clone();
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
    pub fn execute_command(&mut self, command: Command) -> Result<()> {
        match command {
            Command::PatchFixture { id, address, dmx_mode, gdtf_file_name } => {
                let gdtf_file_path = {
                    let show = self.show.lock().unwrap();
                    let showfile_path = match show.path() {
                        Some(path) => path,
                        None => {
                            todo!(
                                "Support creating new showfiles and defining their temporary location"
                            )
                        }
                    };

                    Path::new(&showfile_path)
                        .join(RELATIVE_GDTF_FILE_FOLDER_PATH)
                        .join(&gdtf_file_name)
                };

                let gdtf_file =
                    fs::File::open(gdtf_file_path).context("Failed to open GDTF file")?;
                let fixture_type = &gdtf::GdtfFile::new(gdtf_file)
                    .context("Failed to read GDTF file")?
                    .description
                    .fixture_types[0];

                let fixture = Fixture::new(id, address, dmx_mode, gdtf_file_name, fixture_type)?;

                self.show.lock().unwrap().patch.fixtures.push(fixture);
            }
        }

        Ok(())
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        if let Some(handle) = self.dmx_output_thread.take() {
            handle.join().unwrap();
        }
    }
}
