use std::fs;
use std::path::Path;
use std::time::Duration;

use eyre::{Context, ContextCompat};

use crate::backend::engine::cmd::{Command, PatchCommand, ProgrammerCommand, ProgrammerSetCommand};
use crate::backend::object::{
    AnyObjectId, AnyPresetId, Cue, DimmerPreset, Executor, FixtureGroup, PresetContent, Sequence,
};
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

            let mode = DmxMode::new(fixture.dmx_mode.clone());

            let gdtf_file_name = showfile
                .patch
                .gdtf_files
                .get(fixture.gdtf_file_index)
                .wrap_err("Failed to generate patch: Tried to reference GDTF file index that is out of bounds")?
                .to_string();

            this.exec_cmd(Command::Patch(PatchCommand::Add { id, address, mode, gdtf_file_name }))?;
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

    /// Execute a [Command] to interface with the backend.
    pub fn exec_cmd(&mut self, cmd: Command) -> Result<()> {
        match cmd {
            Command::Patch(PatchCommand::Add { id, address, mode, gdtf_file_name }) => {
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
                    fs::File::open(gdtf_file_path).wrap_err("Failed to open GDTF file")?;
                let fixture_type = &gdtf::GdtfFile::new(gdtf_file)
                    .wrap_err("Failed to read GDTF file")?
                    .description
                    .fixture_types[0];

                let fixture = Fixture::new(id, address, mode, gdtf_file_name, fixture_type)?;

                self.show.patch.fixtures.push(fixture);
            }
            Command::Patch(PatchCommand::SetAddress { id, address }) => todo!(),
            Command::Patch(PatchCommand::SetMode { id, mode }) => todo!(),
            Command::Patch(PatchCommand::SetGdtfFileName { id, name }) => todo!(),
            Command::Patch(PatchCommand::Remove { id }) => todo!(),
            Command::Programmer(ProgrammerCommand::Set(ProgrammerSetCommand::Direct {
                address,
                value,
            })) => {
                self.show.programmer.set_dmx_value(address, value);
            }
            Command::Programmer(ProgrammerCommand::Set(ProgrammerSetCommand::Attribute {
                id,
                attribute,
                value,
            })) => {
                self.show.programmer.set_attribute_value(id, attribute, value);
            }
            Command::Programmer(ProgrammerCommand::Clear) => {
                self.show.programmer.clear();
            }
            Command::Create { id, name } => {
                let show = &mut self.show;
                match id {
                    AnyObjectId::Executor(id) => {
                        let mut executor = Executor::new(id);
                        if let Some(name) = name {
                            executor.name = name;
                        }

                        show.executors.insert(executor.id(), executor);
                    }
                    AnyObjectId::Sequence(id) => {
                        let mut sequence = Sequence::new(id);
                        if let Some(name) = name {
                            sequence.name = name;
                        }

                        show.sequences.insert(sequence.id(), sequence);
                    }
                    AnyObjectId::FixtureGroup(id) => {
                        let mut fixture_group = FixtureGroup::new(id);
                        if let Some(name) = name {
                            fixture_group.name = name;
                        }

                        show.fixture_groups.insert(fixture_group.id(), fixture_group);
                    }
                    AnyObjectId::Cue(id) => {
                        let mut cue = Cue::new(id);
                        if let Some(name) = name {
                            cue.name = name;
                        }

                        show.cues.insert(cue.id(), cue);
                    }
                    AnyObjectId::Preset(id) => match id {
                        AnyPresetId::Dimmer(id) => {
                            let mut dimmer_preset = DimmerPreset::new(id, PresetContent::default());
                            if let Some(name) = name {
                                dimmer_preset.name = name;
                            }

                            show.dimmer_presets.insert(dimmer_preset.id(), dimmer_preset);
                        }
                    },
                };
            }
            Command::Remove { id } => todo!(),
            Command::Rename { id, name } => todo!(),
            Command::FixtureGroup(id, fixture_group_command) => todo!(),
            Command::Executor(id, executor_command) => todo!(),
            Command::Sequence(id, sequence_command) => todo!(),
            Command::Cue(id, cue_command) => todo!(),
            Command::Preset(id, preset_command) => todo!(),
        }

        Ok(())
    }
}
