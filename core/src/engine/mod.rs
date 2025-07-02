use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use eyre::{Context, ContextCompat};

use super::pipeline::Pipeline;
use crate::adapters::midi::MidiAdapter;
use crate::midi::MidiCommand;
use crate::showfile::{RELATIVE_GDTF_FILE_FOLDER_PATH, Showfile};
use crate::{
    AnyObjectId, AnyPreset, AnyPresetId, Cue, CueId, DimmerPreset, DmxMode, Executor, ExecutorId,
    Fixture, FixtureGroup, FixtureGroupId, FixtureId, PresetContent, Result, Sequence, SequenceId,
    Show,
};

pub use cmd::*;

pub mod cmd;
mod dmx_resolver;

pub const DMX_OUTPUT_UPDATE_INTERVAL: Duration = Duration::from_millis(40);

/// The [Engine] controls the flow of output data,
/// and is the interface between the user interface
/// (including a headless app) and
/// the show.
pub struct Engine {
    show: Show,

    /// The final output that will be sent to the DMX sources.
    output_pipeline: Pipeline,

    // Needs to be kept alive.
    _midi_adapter: MidiAdapter,
    /// Receives MIDI commands from the MIDI adapter.
    midi_rx: mpsc::Receiver<MidiCommand>,
}

impl Engine {
    /// Creates a new [Engine] and internally converts the provided [Showfile] into a [Show].
    pub fn new(showfile: Showfile) -> Result<Self> {
        let show = Show::new(showfile.path().cloned());

        let (midi_tx, midi_rx) = mpsc::channel();

        let mut this = Self {
            show,
            output_pipeline: Pipeline::new(),
            _midi_adapter: MidiAdapter::new(showfile.adapters().midi(), midi_tx)
                .context("failed to create midi controller")?,
            midi_rx,
        };

        this.show.patch.gdtf_file_names = showfile.patch().gdtf_files().to_vec();

        // Initialize patch.
        for fixture in showfile.patch().fixtures() {
            let id = FixtureId(fixture.id());

            let address = dmx::Address::new(
                dmx::UniverseId::new(fixture.universe())?,
                dmx::Channel::new(fixture.channel())?,
            );

            let mode = DmxMode::new(fixture.dmx_mode());

            let gdtf_file_name = showfile
                .patch()
                .gdtf_files()
                .get(fixture.gdtf_file_index())
                .wrap_err("failed to generate patch: tried to reference GDTF file index that is out of bounds")?
                .to_string();

            this.exec_cmd(Command::Patch(PatchCommand::Add { id, address, mode, gdtf_file_name }))?;
        }

        // Run init commands
        for cmd in showfile.init_commands() {
            this.exec_cmd(cmd.clone()).context("failed to run init command")?;
        }

        this.output_pipeline.clear_unresolved();

        Ok(this)
    }

    pub fn show(&self) -> &Show {
        &self.show
    }

    /// Do a single iteration of DMX resolving and executor state management.
    /// This should be called in a loop externally, with a delay of [DMX_OUTPUT_UPDATE_INTERVAL].
    pub fn resolve_dmx(&mut self) {
        // FIXME: Cloning the whole show is extremely cursed.
        let show = &self.show.clone();
        for executor in self.show.executors.values_mut() {
            executor.manage_state(&show);
        }

        self.output_pipeline = Pipeline::default();
        dmx_resolver::resolve(&mut self.output_pipeline, &mut self.show);
    }

    /// Gets the resolved output [Multiverse].
    pub fn output_multiverse(&self) -> &dmx::Multiverse {
        self.output_pipeline.resolved_multiverse()
    }

    /// Execute a [Command] to interface with the backend.
    pub fn exec_cmd(&mut self, cmd: Command) -> Result<()> {
        match cmd {
            Command::Patch(cmd) => self.exec_patch_command(cmd),
            Command::Programmer(cmd) => self.exec_programmer_command(cmd),
            Command::Create { id, name } => self.exec_create_command(id, name),
            Command::Remove { id } => self.exec_remove_command(id),
            Command::Rename { id, name } => self.exec_rename_command(id, name),
            Command::FixtureGroup(id, cmd) => self.exec_fixture_group_command(id, cmd),
            Command::Executor(id, cmd) => self.exec_executor_command(id, cmd),
            Command::Sequence(id, cmd) => self.exec_sequence_command(id, cmd),
            Command::Cue(id, cmd) => self.exec_cue_command(id, cmd),
            Command::Preset(id, cmd) => self.exec_preset_command(id, cmd),
        }
    }

    fn exec_patch_command(&mut self, cmd: PatchCommand) -> Result<()> {
        match cmd {
            PatchCommand::Add { id, address, gdtf_file_name, mode } => {
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
            PatchCommand::SetAddress { id, address } => {
                if let Some(fixture) = self.show.patch.fixture_mut(id) {
                    fixture.address = address;
                }
            }
            PatchCommand::SetMode { id, mode } => {
                if let Some(fixture) = self.show.patch.fixture_mut(id) {
                    eyre::ensure!(
                        fixture.supported_dmx_modes().contains(&mode),
                        "fixture with id '{id}' does not support dmx mode '{mode}'"
                    );

                    fixture.dmx_mode = mode;
                }
            }
            PatchCommand::SetGdtfFileName { id, name } => {
                eyre::ensure!(
                    self.show.patch.gdtf_file_names().contains(&name),
                    "the patch does not contain GDTF file with the name '{name}'"
                );

                if let Some(fixture) = self.show.patch.fixture_mut(id) {
                    fixture.gdtf_file_name = name;
                }
            }
            PatchCommand::Remove { id } => {
                self.show.patch.remove_fixture(id);
            }
        }

        Ok(())
    }

    fn exec_programmer_command(&mut self, cmd: ProgrammerCommand) -> Result<()> {
        match cmd {
            ProgrammerCommand::Set(ProgrammerSetCommand::Direct { address, value }) => {
                self.show.programmer.set_dmx_value(address, value);
            }
            ProgrammerCommand::Set(ProgrammerSetCommand::Attribute { id, attribute, value }) => {
                self.show.programmer.set_attribute_value(id, attribute, value);
            }
            ProgrammerCommand::Clear => {
                // NOTE: We have to completely renew the pipeline,
                //       as clearing it only clears unresolved values.
                self.show.programmer = Pipeline::new();
            }
        }

        Ok(())
    }

    fn exec_create_command(&mut self, id: AnyObjectId, name: Option<String>) -> Result<()> {
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
        }

        Ok(())
    }

    fn exec_remove_command(&mut self, id: AnyObjectId) -> Result<()> {
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
        }

        Ok(())
    }

    fn exec_rename_command(&mut self, id: AnyObjectId, name: String) -> Result<()> {
        match id {
            AnyObjectId::Executor(id) => {
                self.show.executor_mut(id).wrap_err("executor with id '{id}' not found")?.name =
                    name;
            }
            AnyObjectId::Sequence(id) => {
                self.show.sequence_mut(id).wrap_err("sequence with id '{id}' not found")?.name =
                    name;
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
                        AnyPreset::Dimmer(preset) => {
                            preset.name = name;
                        }
                    }
                }
            },
        }
        Ok(())
    }

    fn exec_fixture_group_command(
        &mut self,
        id: FixtureGroupId,
        cmd: FixtureGroupCommand,
    ) -> Result<()> {
        let Some(fixture_group) = self.show.fixture_groups.get_mut(&id) else {
            eyre::bail!("fixture_group with id '{id}' not found");
        };

        match cmd {
            FixtureGroupCommand::Add { ids } => {
                fixture_group.fixtures.extend(ids);
            }
            FixtureGroupCommand::ReplaceAt { index, id: fixture_id } => {
                let Some(fixture_at_index) = fixture_group.fixtures.get_mut(index) else {
                    eyre::bail!(
                        "index '{index}' is out of bounds for fixture_group '{id}' with length {}",
                        fixture_group.len()
                    );
                };
                *fixture_at_index = fixture_id;
            }
            FixtureGroupCommand::Remove { id: fixture_id } => {
                fixture_group.fixtures.retain(|fid| *fid != fixture_id);
            }
            FixtureGroupCommand::RemoveAt { index } => {
                fixture_group.fixtures.remove(index);
            }
            FixtureGroupCommand::Clear => {
                fixture_group.fixtures.clear();
            }
        }

        Ok(())
    }

    fn exec_executor_command(&mut self, id: ExecutorId, cmd: ExecutorCommand) -> Result<()> {
        let Some(executor) = self.show.executors.get_mut(&id) else {
            eyre::bail!("executor with id '{id}' not found");
        };

        match cmd {
            ExecutorCommand::Button(cmd) => match cmd {
                ExecutorButtonCommand::SetMode { mode } => executor.button.set_mode(mode),
                ExecutorButtonCommand::Press => executor.button.press(),
                ExecutorButtonCommand::Release => executor.button.release(),
            },
            ExecutorCommand::Fader(cmd) => match cmd {
                ExecutorFaderCommand::SetMode { mode } => executor.fader.set_mode(mode),
                ExecutorFaderCommand::SetLevel { level } => executor.fader.set_level(level),
            },
            ExecutorCommand::SetSequence { sequence_id } => {
                executor.sequence_id = Some(sequence_id);
            }
            ExecutorCommand::Clear => {
                executor.sequence_id = None;
            }
        }

        Ok(())
    }

    fn exec_sequence_command(&mut self, id: SequenceId, cmd: SequenceCommand) -> Result<()> {
        let Some(sequence) = self.show.sequences.get_mut(&id) else {
            eyre::bail!("sequence with id '{id}' not found");
        };

        match cmd {
            SequenceCommand::Add { cue_ids } => {
                sequence.cues.extend(cue_ids);
            }
            SequenceCommand::ReplaceAt { index, cue_id } => {
                let Some(cue_at_index) = sequence.cues.get_mut(index) else {
                    eyre::bail!(
                        "index '{index}' is out of bounds for sequence '{id}' with length {}",
                        sequence.len()
                    );
                };
                *cue_at_index = cue_id;
            }
            SequenceCommand::Remove { cue_id } => {
                sequence.cues.retain(|cid| *cid != cue_id);
            }
            SequenceCommand::RemoveAt { index } => {
                sequence.cues.remove(index);
            }
            SequenceCommand::Clear => {
                sequence.cues.clear();
            }
        }

        Ok(())
    }

    fn exec_cue_command(&mut self, id: CueId, cmd: CueCommand) -> Result<()> {
        let Some(cue) = self.show.cues.get_mut(&id) else {
            eyre::bail!("cue with id '{id}' not found");
        };

        match cmd {
            CueCommand::Add { recipes } => {
                cue.recipes.extend(recipes);
            }
            CueCommand::ReplaceAt { index, recipe } => {
                let Some(recipe_at_index) = cue.recipes.get_mut(index) else {
                    eyre::bail!(
                        "index '{index}' is out of bounds for sequence '{id}' with length {}",
                        cue.recipes.len()
                    );
                };
                *recipe_at_index = recipe;
            }
            CueCommand::RemoveAt { index } => {
                cue.recipes.remove(index);
            }
            CueCommand::Clear => {
                cue.recipes.clear();
            }
        }

        Ok(())
    }

    fn exec_preset_command(&mut self, id: AnyPresetId, cmd: PresetCommand) -> Result<()> {
        match cmd {
            PresetCommand::Store => {
                self.show.programmer.resolve(&self.show.patch);
                let resolved_attribute_values =
                    self.show().programmer.resolved_attribute_values().clone();
                let Some(preset) = self.show.preset_mut(id) else {
                    eyre::bail!("preset with id '{id}' not found");
                };
                for ((fid, attr), value) in resolved_attribute_values {
                    match preset {
                        AnyPreset::Dimmer(preset) => match &mut preset.content {
                            PresetContent::Selective(selective_preset) => {
                                selective_preset.set_attribute_value(fid, attr, value);
                            }
                        },
                    }
                }
            }
            PresetCommand::Clear => {
                let Some(preset) = self.show.preset_mut(id) else {
                    eyre::bail!("preset with id '{id}' not found");
                };
                match preset {
                    AnyPreset::Dimmer(preset) => match &mut preset.content {
                        PresetContent::Selective(selective_preset) => {
                            selective_preset.clear();
                        }
                    },
                }
            }
        }

        Ok(())
    }

    pub fn handle_control_input(&mut self) -> Result<()> {
        for midi_message in self.midi_rx.try_iter().collect::<Vec<_>>() {
            match midi_message {
                MidiCommand::ExecutorButtonPress { executor_id } => {
                    self.exec_cmd(crate::cmd!(&format!("executor {executor_id} button press")))?;
                }
                MidiCommand::ExecutorButtonRelease { executor_id } => {
                    self.exec_cmd(crate::cmd!(&format!("executor {executor_id} button release")))?;
                }
                MidiCommand::ExecutorFaderSetValue { executor_id, value } => {
                    self.exec_cmd(crate::cmd!(&format!(
                        "executor {executor_id} fader level {value:?}"
                    )))?;
                }
            }
        }
        Ok(())
    }
}
