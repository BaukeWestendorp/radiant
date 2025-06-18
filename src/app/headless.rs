use eyre::Context;

use crate::backend::engine::Engine;
use crate::backend::engine::cmd::Cmd;
use crate::backend::object::{
    AnyPresetId, Cue, CueContent, DimmerPreset, Executor, FixtureGroup, PresetContent, Recipe,
    SelectivePreset, Sequence,
};
use crate::backend::patch::attr::{Attribute, AttributeValue};
use crate::backend::patch::fixture::FixtureId;
use crate::error::Result;
use crate::showfile::Showfile;

/// Starts the app in headless mode.
pub fn run(showfile: Showfile) -> Result<()> {
    let mut engine = Engine::new(showfile).context("Failed to create engine")?;
    engine.start().context("Failed to start engine")?;

    let dimmer_preset = DimmerPreset::new(
        0,
        PresetContent::Selective({
            let mut content = SelectivePreset::default();
            content.set_attribute_value(FixtureId(0), Attribute::Dimmer, AttributeValue::new(0.5));
            content
        }),
    );

    let fixture_group = FixtureGroup::new(0).with_fixture(FixtureId(0));

    let sequence = Sequence::new(0).with_cue(Cue::new(CueContent::Recipe(
        Recipe::new().with_combination(fixture_group.id, AnyPresetId::from(dimmer_preset.id)),
    )));

    let executor = Executor::new(0).with_sequence(sequence.id);
    // executor.set_active_cue_index(Some(0), engine.show());

    engine.exec_cmd(Cmd::New(dimmer_preset.into()))?;
    engine.exec_cmd(Cmd::New(fixture_group.into()))?;
    engine.exec_cmd(Cmd::New(sequence.into()))?;
    engine.exec_cmd(Cmd::New(executor.into()))?;

    Ok(())
}
