use std::fs;
use std::path::Path;

use eyre::{Context, ContextCompat};

use crate::cmd::{
    Command, CueCommand, ExecutorButtonCommand, ExecutorCommand, ExecutorFaderCommand,
    FixtureGroupCommand, PatchCommand, PresetCommand, ProgrammerCommand, ProgrammerSetCommand,
    SequenceCommand,
};
use crate::engine::Engine;
use crate::error::Result;
use crate::object::{
    AnyObjectId, AnyPresetId, ColorPreset, Cue, CueId, DimmerPreset, Executor, ExecutorId,
    FixtureGroup, FixtureGroupId, PresetContent, SelectivePreset, Sequence, SequenceId,
};
use crate::patch::{FeatureGroup, Fixture};
use crate::pipeline::Pipeline;
use crate::showfile::RELATIVE_GDTF_FILE_FOLDER_PATH;

pub fn exec_cmd(engine: &mut Engine, cmd: Command) -> Result<()> {
    match cmd {
        Command::Patch(cmd) => exec_patch_command(engine, cmd),
        Command::Programmer(cmd) => exec_programmer_command(engine, cmd),
        Command::Create { id, name } => exec_create_command(engine, id, name),
        Command::Remove { id } => exec_remove_command(engine, id),
        Command::Rename { id, name } => exec_rename_command(engine, id, name),
        Command::FixtureGroup(id, cmd) => exec_fixture_group_command(engine, id, cmd),
        Command::Executor(id, cmd) => exec_executor_command(engine, id, cmd),
        Command::Sequence(id, cmd) => exec_sequence_command(engine, id, cmd),
        Command::Cue(id, cmd) => exec_cue_command(engine, id, cmd),
        Command::Preset(id, cmd) => exec_preset_command(engine, id, cmd),
    }
}

fn exec_patch_command(engine: &mut Engine, cmd: PatchCommand) -> Result<()> {
    match cmd {
        PatchCommand::Add { id, address, gdtf_file_name, mode } => {
            let gdtf_file_path = {
                let showfile_path = match engine.show.path() {
                    Some(path) => path,
                    None => {
                        todo!(
                            "support creating new showfiles and defining their temporary location"
                        )
                    }
                };

                Path::new(&showfile_path).join(RELATIVE_GDTF_FILE_FOLDER_PATH).join(&gdtf_file_name)
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

            engine.show.patch.fixtures.push(fixture);
        }
        PatchCommand::SetAddress { id, address } => {
            if let Some(fixture) = engine.show.patch.fixture_mut(id) {
                fixture.address = address;
            }
        }
        PatchCommand::SetMode { id, mode } => {
            if let Some(fixture) = engine.show.patch.fixture_mut(id) {
                eyre::ensure!(
                    fixture.supported_dmx_modes().contains(&mode),
                    "fixture with id '{id}' does not support dmx mode '{mode}'"
                );

                fixture.dmx_mode = mode;
            }
        }
        PatchCommand::SetGdtfFileName { id, name } => {
            eyre::ensure!(
                engine.show.patch.gdtf_file_names().contains(&name),
                "the patch does not contain GDTF file with the name '{name}'"
            );

            if let Some(fixture) = engine.show.patch.fixture_mut(id) {
                fixture.gdtf_file_name = name;
            }
        }
        PatchCommand::Remove { id } => {
            engine.show.patch.remove_fixture(id);
        }
    }

    Ok(())
}

fn exec_programmer_command(engine: &mut Engine, cmd: ProgrammerCommand) -> Result<()> {
    match cmd {
        ProgrammerCommand::Set(ProgrammerSetCommand::Direct { address, value }) => {
            engine.show.programmer.set_dmx_value(address, value);
        }
        ProgrammerCommand::Set(ProgrammerSetCommand::Attribute { id, attribute, value }) => {
            engine.show.programmer.set_attribute_value(id, attribute, value);
        }
        ProgrammerCommand::Clear => {
            // NOTE: We have to completely renew the pipeline,
            //       as clearing it only clears unresolved values.
            engine.show.programmer = Pipeline::new();
        }
    }

    Ok(())
}

fn exec_create_command(engine: &mut Engine, id: AnyObjectId, name: Option<String>) -> Result<()> {
    match id {
        AnyObjectId::Executor(id) => {
            let mut executor = Executor::new(id);
            if let Some(name) = name {
                executor.name = name;
            }
            engine.show.executors.insert(executor.id(), executor);
        }
        AnyObjectId::Sequence(id) => {
            let mut sequence = Sequence::new(id);
            if let Some(name) = name {
                sequence.name = name;
            }
            engine.show.sequences.insert(sequence.id(), sequence);
        }
        AnyObjectId::FixtureGroup(id) => {
            let mut fixture_group = FixtureGroup::new(id);
            if let Some(name) = name {
                fixture_group.name = name;
            }
            engine.show.fixture_groups.insert(fixture_group.id(), fixture_group);
        }
        AnyObjectId::Cue(id) => {
            let mut cue = Cue::new(id);
            if let Some(name) = name {
                cue.name = name;
            }
            engine.show.cues.insert(cue.id(), cue);
        }
        AnyObjectId::Preset(id) => match id {
            AnyPresetId::Dimmer(id) => {
                let mut preset = DimmerPreset::new(
                    id,
                    PresetContent::Selective(SelectivePreset::new(FeatureGroup::Dimmer)),
                );

                if let Some(name) = name {
                    preset.name = name;
                }

                engine.show.dimmer_presets.insert(preset.id(), preset);
            }
            AnyPresetId::Color(id) => {
                let mut preset = ColorPreset::new(
                    id,
                    PresetContent::Selective(SelectivePreset::new(FeatureGroup::Color)),
                );

                if let Some(name) = name {
                    preset.name = name;
                }

                engine.show.color_presets.insert(preset.id(), preset);
            }
        },
    }

    Ok(())
}

fn exec_remove_command(engine: &mut Engine, id: AnyObjectId) -> Result<()> {
    match id {
        AnyObjectId::Executor(id) => {
            engine.show.executors.remove(&id);
        }
        AnyObjectId::Sequence(id) => {
            engine.show.sequences.remove(&id);
        }
        AnyObjectId::FixtureGroup(id) => {
            engine.show.fixture_groups.remove(&id);
        }
        AnyObjectId::Cue(id) => {
            engine.show.cues.remove(&id);
        }
        AnyObjectId::Preset(id) => match id {
            AnyPresetId::Dimmer(id) => {
                engine.show.dimmer_presets.remove(&id);
            }
            AnyPresetId::Color(id) => {
                engine.show.color_presets.remove(&id);
            }
        },
    }

    Ok(())
}

fn exec_rename_command(engine: &mut Engine, id: AnyObjectId, name: String) -> Result<()> {
    match id {
        AnyObjectId::Executor(id) => {
            engine.show.executor_mut(id).wrap_err("executor with id '{id}' not found")?.name = name;
        }
        AnyObjectId::Sequence(id) => {
            engine.show.sequence_mut(id).wrap_err("sequence with id '{id}' not found")?.name = name;
        }
        AnyObjectId::FixtureGroup(id) => {
            engine
                .show
                .fixture_group_mut(id)
                .wrap_err("fixture_group with id '{id}' not found")?
                .name = name;
        }
        AnyObjectId::Cue(id) => {
            engine.show.cue_mut(id).wrap_err("cue with id '{id}' not found")?.name = name;
        }
        AnyObjectId::Preset(any_preset_id) => match any_preset_id {
            AnyPresetId::Dimmer(id) => {
                let preset = engine
                    .show
                    .dimmer_presets
                    .get_mut(&id)
                    .wrap_err("preset with id '{id}' not found")?;
                preset.name = name;
            }
            AnyPresetId::Color(id) => {
                let preset = engine
                    .show
                    .color_presets
                    .get_mut(&id)
                    .wrap_err("preset with id '{id}' not found")?;
                preset.name = name;
            }
        },
    }
    Ok(())
}

fn exec_fixture_group_command(
    engine: &mut Engine,
    id: FixtureGroupId,
    cmd: FixtureGroupCommand,
) -> Result<()> {
    let Some(fixture_group) = engine.show.fixture_groups.get_mut(&id) else {
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

fn exec_executor_command(engine: &mut Engine, id: ExecutorId, cmd: ExecutorCommand) -> Result<()> {
    let Some(executor) = engine.show.executors.get_mut(&id) else {
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

fn exec_sequence_command(engine: &mut Engine, id: SequenceId, cmd: SequenceCommand) -> Result<()> {
    let Some(sequence) = engine.show.sequences.get_mut(&id) else {
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

fn exec_cue_command(engine: &mut Engine, id: CueId, cmd: CueCommand) -> Result<()> {
    let Some(cue) = engine.show.cues.get_mut(&id) else {
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

fn exec_preset_command(engine: &mut Engine, id: AnyPresetId, cmd: PresetCommand) -> Result<()> {
    match cmd {
        PresetCommand::Store => {
            engine.show.programmer.resolve(&engine.show.patch);
            let resolved_attribute_values =
                engine.show().programmer.resolved_attribute_values().clone();

            for ((fid, attr), value) in resolved_attribute_values {
                match id {
                    AnyPresetId::Dimmer(id) => {
                        match &mut engine
                            .show
                            .dimmer_presets
                            .get_mut(&id)
                            .wrap_err("preset with id '{id}' not found")?
                            .content
                        {
                            PresetContent::Selective(selective_preset) => {
                                selective_preset.set_attribute_value(fid, attr, value);
                            }
                        }
                    }
                    AnyPresetId::Color(id) => {
                        match &mut engine
                            .show
                            .color_presets
                            .get_mut(&id)
                            .wrap_err("preset with id '{id}' not found")?
                            .content
                        {
                            PresetContent::Selective(selective_preset) => {
                                selective_preset.set_attribute_value(fid, attr, value);
                            }
                        }
                    }
                }
            }
        }
        PresetCommand::Clear => match id {
            AnyPresetId::Dimmer(id) => {
                match &mut engine
                    .show
                    .dimmer_presets
                    .get_mut(&id)
                    .wrap_err("preset with id '{id}' not found")?
                    .content
                {
                    PresetContent::Selective(selective_preset) => {
                        selective_preset.clear();
                    }
                }
            }
            AnyPresetId::Color(id) => {
                match &mut engine
                    .show
                    .color_presets
                    .get_mut(&id)
                    .wrap_err("preset with id '{id}' not found")?
                    .content
                {
                    PresetContent::Selective(selective_preset) => {
                        selective_preset.clear();
                    }
                }
            }
        },
    }

    Ok(())
}
