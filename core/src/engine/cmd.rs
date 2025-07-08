use std::fs;
use std::path::Path;

use eyre::{Context, ContextCompat};

use crate::cmd::{
    Command, CueCommand, ExecutorCommand, FixtureGroupCommand, ObjectCommand, PatchCommand,
    PresetCommand, ProgrammerCommand, SequenceCommand,
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
        Command::Object(cmd) => exec_object_command(engine, cmd),
    }
}

fn exec_patch_command(engine: &mut Engine, cmd: PatchCommand) -> Result<()> {
    match cmd {
        PatchCommand::Add { fid, address, gdtf, mode } => {
            let gdtf_file_path = {
                let showfile_path = match engine.show.path() {
                    Some(path) => path,
                    None => {
                        todo!(
                            "support creating new showfiles and defining their temporary location"
                        )
                    }
                };

                Path::new(&showfile_path).join(RELATIVE_GDTF_FILE_FOLDER_PATH).join(&gdtf)
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

            let fixture = Fixture::new(fid, address, mode, gdtf, fixture_type)?;

            engine.show.patch.fixtures.push(fixture);
        }
        PatchCommand::SetAddress { fid, address } => {
            if let Some(fixture) = engine.show.patch.fixture_mut(fid) {
                fixture.address = address;
            }
        }
        PatchCommand::SetMode { fid, mode } => {
            if let Some(fixture) = engine.show.patch.fixture_mut(fid) {
                eyre::ensure!(
                    fixture.supported_dmx_modes().contains(&mode),
                    "fixture with id '{fid}' does not support dmx mode '{mode}'"
                );

                fixture.dmx_mode = mode;
            }
        }
        PatchCommand::SetGdtf { fid, name } => {
            eyre::ensure!(
                engine.show.patch.gdtfs().contains(&name),
                "the patch does not contain GDTF file with the name '{name}'"
            );

            if let Some(fixture) = engine.show.patch.fixture_mut(fid) {
                fixture.gdtf = name;
            }
        }
        PatchCommand::Remove { fid } => {
            engine.show.patch.remove_fixture(fid);
        }
    }

    Ok(())
}

fn exec_programmer_command(engine: &mut Engine, cmd: ProgrammerCommand) -> Result<()> {
    match cmd {
        ProgrammerCommand::SetAddress { address, value } => {
            engine.show.programmer.set_dmx_value(address, value);
        }
        ProgrammerCommand::SetAttribute { fid, attribute, value } => {
            engine.show.programmer.set_attribute_value(fid, attribute, value);
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

fn exec_object_command(engine: &mut Engine, cmd: ObjectCommand) -> Result<()> {
    match cmd {
        ObjectCommand::Executor(id, cmd) => exec_executor_command(engine, id, cmd),
        ObjectCommand::Sequence(id, cmd) => exec_sequence_command(engine, id, cmd),
        ObjectCommand::Cue(id, cmd) => exec_cue_command(engine, id, cmd),
        ObjectCommand::FixtureGroup(id, cmd) => exec_fixture_group_command(engine, id, cmd),
        ObjectCommand::Preset(id, cmd) => exec_preset_command(engine, id, cmd),
    }
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
        FixtureGroupCommand::Add { fid } => {
            fixture_group.fixtures.push(fid);
        }
        FixtureGroupCommand::ReplaceAt { index, fid } => {
            let Some(fixture_at_index) = fixture_group.fixtures.get_mut(index) else {
                eyre::bail!(
                    "index '{index}' is out of bounds for fixture_group '{fid}' with length {}",
                    fixture_group.len()
                );
            };
            *fixture_at_index = fid;
        }
        FixtureGroupCommand::Remove { fid } => {
            fixture_group.fixtures.retain(|f| *f != fid);
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
        ExecutorCommand::ButtonSetMode { mode } => executor.button.set_mode(mode),
        ExecutorCommand::ButtonPress => executor.button.press(),
        ExecutorCommand::ButtonRelease => executor.button.release(),
        ExecutorCommand::FaderSetMode { mode } => executor.fader.set_mode(mode),
        ExecutorCommand::FaderSetLevel { level } => executor.fader.set_level(level),
        ExecutorCommand::SetSequence { sequence_id } => executor.sequence_id = Some(sequence_id),
        ExecutorCommand::Clear => executor.sequence_id = None,
    }

    Ok(())
}

fn exec_sequence_command(engine: &mut Engine, id: SequenceId, cmd: SequenceCommand) -> Result<()> {
    let Some(sequence) = engine.show.sequences.get_mut(&id) else {
        eyre::bail!("sequence with id '{id}' not found");
    };

    match cmd {
        SequenceCommand::Add { cue_id } => {
            sequence.cues.push(cue_id);
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
        CueCommand::Add { recipe } => {
            cue.recipes.push(recipe);
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
                            PresetContent::Selective(preset) => {
                                preset.set_attribute_value(fid, attr, value);
                            }
                            PresetContent::Universal(preset) => {
                                preset.set_attribute_value(attr, value);
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
                            PresetContent::Selective(preset) => {
                                preset.set_attribute_value(fid, attr, value);
                            }
                            PresetContent::Universal(preset) => {
                                preset.set_attribute_value(attr, value);
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
                    PresetContent::Selective(preset) => preset.clear(),
                    PresetContent::Universal(preset) => preset.clear(),
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
                    PresetContent::Universal(preset) => {
                        preset.clear();
                    }
                }
            }
        },
    }

    Ok(())
}
