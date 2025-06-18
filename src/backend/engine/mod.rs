use std::fs;
use std::path::Path;
use std::time::Duration;

use eyre::{Context, ContextCompat};

use crate::backend::engine::cmd::Cmd;
use crate::backend::object::{AnyPreset, Object};
use crate::backend::patch::fixture::{DmxMode, Fixture, FixtureId};
use crate::backend::pipeline::Pipeline;
use crate::backend::show::Show;
use crate::dmx;
use crate::error::Result;
use crate::showfile::{RELATIVE_GDTF_FILE_FOLDER_PATH, Showfile};

pub mod cmd;
mod dmx_resolver;

pub const DMX_OUTPUT_UPDATE_INTERVAL: Duration = Duration::from_millis(40);

/// The [Engine] controls the flow of output data,
/// and is the interface between the user interface
/// (including a headless app, even if it's a CLI) and
/// the show.
pub struct Engine {
    show: Show,

    /// The final output that will be sent to the DMX sources.
    output_pipeline: Pipeline,
}

impl Engine {
    /// Creates a new [Engine] and internally converts the provided [Showfile] into a [Show].
    pub fn new(showfile: Showfile) -> Result<Self> {
        let show = Show::new(showfile.path().cloned());

        let mut this = Self { show, output_pipeline: Pipeline::new() };

        // Initialize show.
        for fixture in &showfile.patch.fixtures {
            let id = FixtureId(fixture.id);

            let address = dmx::Address::new(
                dmx::UniverseId::new(fixture.universe)?,
                dmx::Channel::new(fixture.channel)?,
            );

            let dmx_mode = DmxMode::new(fixture.dmx_mode.clone());

            let gdtf_file_name = showfile
                .patch
                .gdtf_files
                .get(fixture.gdtf_file_index)
                .context("Failed to generate patch: Tried to reference GDTF file index that is out of bounds")?
                .to_string();

            this.exec_cmd(Cmd::PatchFixture { id, address, dmx_mode, gdtf_file_name })?;
        }

        this.output_pipeline.clear();

        Ok(this)
    }

    pub fn show(&self) -> &Show {
        &self.show
    }

    /// Do a single iteration of DMX resolving. This should be called in a
    /// loop externally, with a delay of [DMX_OUTPUT_UPDATE_INTERVAL].
    pub fn resolve_dmx(&mut self) {
        dmx_resolver::resolve(&mut self.output_pipeline, &mut self.show);
    }

    /// Execute a [Cmd] to interface with the backend.
    pub fn exec_cmd(&mut self, cmd: Cmd) -> Result<()> {
        match cmd {
            Cmd::PatchFixture { id, address, dmx_mode, gdtf_file_name } => {
                let gdtf_file_path = {
                    let showfile_path = match self.show.path() {
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

                self.show.patch.fixtures.push(fixture);
            }
            Cmd::SetDmxValue { address, value } => {
                self.show.programmer.set_dmx_value(address, value);
            }
            Cmd::SetAttributeValue { fixture_id, attribute, value } => {
                self.show.programmer.set_attribute_value(fixture_id, attribute, value);
            }
            Cmd::New(object) => {
                let show = &mut self.show;
                match object {
                    Object::Executor(executor) => {
                        show.executors.insert(executor.id(), executor);
                    }
                    Object::Sequence(sequence) => {
                        show.sequences.insert(sequence.id(), sequence);
                    }
                    Object::FixtureGroup(fixture_group) => {
                        show.fixture_groups.insert(fixture_group.id(), fixture_group);
                    }
                    Object::Preset(any_preset) => match any_preset {
                        AnyPreset::Dimmer(preset) => {
                            show.dimmer_presets.insert(preset.id(), preset);
                        }
                    },
                };
            }
        }

        Ok(())
    }
}
