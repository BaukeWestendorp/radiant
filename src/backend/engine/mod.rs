use std::fs;
use std::path::Path;
use std::time::Duration;

use eyre::{Context, ContextCompat};

use crate::backend::engine::cmd::{
    Command, FixtureGroupCommand, PatchCommand, ProgrammerCommand, ProgrammerSetCommand,
    SequenceCommand,
};
use crate::backend::object::{
    AnyObjectId, AnyPreset, AnyPresetId, Cue, DimmerPreset, Executor, FixtureGroup, PresetContent,
    Sequence,
};
use crate::backend::patch::fixture::{DmxMode, Fixture, FixtureId};
use crate::backend::pipeline::Pipeline;
use crate::backend::show::Show;
use crate::dmx::{self, Multiverse};
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

        this.show.patch.gdtf_file_names = showfile.patch.gdtf_files.clone();

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
                .wrap_err("failed to generate patch: Tried to reference GDTF file index that is out of bounds")?
                .to_string();

            this.exec_cmd(Command::Patch(PatchCommand::Add { id, address, mode, gdtf_file_name }))?;
        }

        this.output_pipeline.clear_unresolved();

        Ok(this)
    }

    pub fn show(&self) -> &Show {
        &self.show
    }

    /// Do a single iteration of DMX resolving. This should be called in a
    /// loop externally, with a delay of [DMX_OUTPUT_UPDATE_INTERVAL].
    pub fn resolve_dmx(&mut self) {
        self.output_pipeline = Pipeline::default();
        dmx_resolver::resolve(&mut self.output_pipeline, &mut self.show);
    }

    pub fn output_multiverse(&self) -> &Multiverse {
        self.output_pipeline.output_multiverse()
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
                                "support creating new showfiles and defining their temporary location"
                            )
                        }
                    };

                    Path::new(&showfile_path)
                        .join(RELATIVE_GDTF_FILE_FOLDER_PATH)
                        .join(&gdtf_file_name)
                };

                let gdtf_file = fs::File::open(&gdtf_file_path).wrap_err_with(|| {
                    format!("failed to open GDTF file at '{}'", gdtf_file_path.display())
                })?;
                let fixture_type = &gdtf::GdtfFile::new(gdtf_file)
                    .wrap_err_with(|| {
                        format!("failed to read GDTF file at '{}'", gdtf_file_path.display())
                    })?
                    .description
                    .fixture_types[0];

                let fixture = Fixture::new(id, address, mode, gdtf_file_name, fixture_type)?;

                self.show.patch.fixtures.push(fixture);
            }
            Command::Patch(PatchCommand::SetAddress { id, address }) => {
                if let Some(fixture) = self.show.patch.fixture_mut(id) {
                    fixture.address = address;
                }
            }
            Command::Patch(PatchCommand::SetMode { id, mode }) => {
                if let Some(fixture) = self.show.patch.fixture_mut(id) {
                    eyre::ensure!(
                        fixture.supported_dmx_modes().contains(&mode),
                        "fixture with id '{id}' does not support dmx mode '{mode}'"
                    );

                    fixture.dmx_mode = mode;
                }
            }
            Command::Patch(PatchCommand::SetGdtfFileName { id, name }) => {
                eyre::ensure!(
                    self.show.patch.gdtf_file_names().contains(&name),
                    "the patch does not contain GDTF file with the name '{name}'"
                );

                if let Some(fixture) = self.show.patch.fixture_mut(id) {
                    fixture.gdtf_file_name = name;
                }
            }
            Command::Patch(PatchCommand::Remove { id }) => {
                self.show.patch.remove_fixture(id);
            }
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
                // NOTE: We have to completely renew the pipeline,
                //       as clearing it only clears unresolved values.
                self.show.programmer = Pipeline::new();
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
            Command::Remove { id } => {
                match id {
                    AnyObjectId::Executor(id) => {
                        self.show.executors.remove(&id);
                    }
                    AnyObjectId::Sequence(id) => {
                        self.show.sequences.remove(&id);
                    }
                    AnyObjectId::FixtureGroup(id) => {
                        self.show.fixture_groups.remove(&id);
                    }
                    AnyObjectId::Cue(id) => {
                        self.show.cues.remove(&id);
                    }
                    AnyObjectId::Preset(id) => match id {
                        AnyPresetId::Dimmer(id) => {
                            self.show.dimmer_presets.remove(&id);
                        }
                    },
                };
            }
            Command::Rename { id, name } => {
                match id {
                    AnyObjectId::Executor(id) => {
                        self.show
                            .executor_mut(id)
                            .wrap_err("executor with id '{id}' not found")?
                            .name = name;
                    }
                    AnyObjectId::Sequence(id) => {
                        self.show
                            .sequence_mut(id)
                            .wrap_err("sequence with id '{id}' not found")?
                            .name = name;
                    }
                    AnyObjectId::FixtureGroup(id) => {
                        self.show
                            .fixture_group_mut(id)
                            .wrap_err("fixture_group with id '{id}' not found")?
                            .name = name;
                    }
                    AnyObjectId::Cue(id) => {
                        self.show.cue_mut(id).wrap_err("cue with id '{id}' not found")?.name = name;
                    }
                    AnyObjectId::Preset(any_preset_id) => match any_preset_id {
                        AnyPresetId::Dimmer(_) => {
                            match self
                                .show
                                .preset_mut(any_preset_id)
                                .wrap_err("preset with id '{id}' not found")?
                            {
                                AnyPreset::Dimmer(preset) => preset.name = name,
                            }
                        }
                    },
                };
            }
            Command::FixtureGroup(id, FixtureGroupCommand::Add { id: fixture_id }) => {
                let Some(fixture_group) = self.show.fixture_groups.get_mut(&id) else {
                    eyre::bail!("fixture_group with id '{id}' not found");
                };
                fixture_group.fixtures.push(fixture_id);
            }
            Command::FixtureGroup(id, FixtureGroupCommand::ReplaceAt { index, id: fixture_id }) => {
                let Some(fixture_group) = self.show.fixture_groups.get_mut(&id) else {
                    eyre::bail!("fixture_group with id '{id}' not found");
                };
                let Some(fixture_at_index) = fixture_group.fixtures.get_mut(index) else {
                    eyre::bail!(
                        "index '{index}' is out of bounds for fixture_group '{id}' with length {}",
                        fixture_group.len()
                    );
                };
                *fixture_at_index = fixture_id;
            }
            Command::FixtureGroup(id, FixtureGroupCommand::Remove { id: fixture_id }) => {
                let Some(fixture_group) = self.show.fixture_groups.get_mut(&id) else {
                    eyre::bail!("fixture_group with id '{id}' not found");
                };
                fixture_group.fixtures.retain(|fid| *fid != fixture_id);
            }
            Command::FixtureGroup(id, FixtureGroupCommand::RemoveAt { index }) => {
                let Some(fixture_group) = self.show.fixture_groups.get_mut(&id) else {
                    eyre::bail!("fixture_group with id '{id}' not found");
                };
                fixture_group.fixtures.remove(index);
            }
            Command::FixtureGroup(id, FixtureGroupCommand::Clear) => {
                let Some(fixture_group) = self.show.fixture_groups.get_mut(&id) else {
                    eyre::bail!("fixture_group with id '{id}' not found");
                };
                fixture_group.fixtures.clear();
            }
            Command::Executor(_id, _executor_command) => todo!(),
            Command::Sequence(id, SequenceCommand::Add { cue_id }) => {
                let Some(sequence) = self.show.sequences.get_mut(&id) else {
                    eyre::bail!("sequence with id '{id}' not found");
                };
                sequence.cues.push(cue_id);
            }
            Command::Sequence(id, SequenceCommand::ReplaceAt { index, cue_id }) => {
                let Some(sequence) = self.show.sequences.get_mut(&id) else {
                    eyre::bail!("sequence with id '{id}' not found");
                };
                let Some(cue_at_index) = sequence.cues.get_mut(index) else {
                    eyre::bail!(
                        "index '{index}' is out of bounds for fixture_group '{id}' with length {}",
                        sequence.len()
                    );
                };
                *cue_at_index = cue_id;
            }
            Command::Sequence(id, SequenceCommand::Remove { cue_id }) => {
                let Some(sequence) = self.show.sequences.get_mut(&id) else {
                    eyre::bail!("sequence with id '{id}' not found");
                };
                sequence.cues.retain(|cid| *cid != cue_id);
            }
            Command::Sequence(id, SequenceCommand::RemoveAt { index }) => {
                let Some(sequence) = self.show.sequences.get_mut(&id) else {
                    eyre::bail!("sequence with id '{id}' not found");
                };
                sequence.cues.remove(index);
            }
            Command::Sequence(id, SequenceCommand::Clear) => {
                let Some(sequence) = self.show.sequences.get_mut(&id) else {
                    eyre::bail!("sequence with id '{id}' not found");
                };
                sequence.cues.clear();
            }
            Command::Cue(_id, _cue_command) => todo!(),
            Command::Preset(_id, _preset_command) => todo!(),
        }

        Ok(())
    }
}
