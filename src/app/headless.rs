use std::sync::{Arc, Mutex};
use std::time::Duration;

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

const DMX_OUTPUT_UPDATE_INTERVAL: Duration = Duration::from_millis(40);

/// Starts the app in headless mode.
pub fn run(showfile: Showfile) -> Result<()> {
    let engine = Engine::new(showfile).context("Failed to create engine")?;
    let engine = Arc::new(Mutex::new(engine));

    let handle = std::thread::spawn({
        let engine = engine.clone();
        move || loop {
            engine.lock().unwrap().resolve_dmx();
            spin_sleep::sleep(DMX_OUTPUT_UPDATE_INTERVAL);
        }
    });

    do_some_fun_stuff_with_engine(engine)?;

    handle.join().unwrap();

    Ok(())
}

fn do_some_fun_stuff_with_engine(engine: Arc<Mutex<Engine>>) -> Result<()> {
    {
        let mut engine = engine.lock().unwrap();

        let dimmer_preset = DimmerPreset::new(
            0,
            PresetContent::Selective({
                let mut content = SelectivePreset::default();
                content.set_attribute_value(
                    FixtureId(0),
                    Attribute::Dimmer,
                    AttributeValue::new(0.5),
                );
                content
            }),
        );
        engine.exec_cmd(Cmd::New(dimmer_preset.clone().into()))?;

        let fixture_group = FixtureGroup::new(0).with_fixture(FixtureId(0));
        engine.exec_cmd(Cmd::New(fixture_group.clone().into()))?;

        let sequence = Sequence::new(0).with_cue(Cue::new(CueContent::Recipe(
            Recipe::new().with_combination(fixture_group.id, AnyPresetId::from(dimmer_preset.id)),
        )));
        engine.exec_cmd(Cmd::New(sequence.clone().into()))?;

        let mut executor = Executor::new(0).with_sequence(sequence.id);
        executor.set_active_cue_index(Some(0), engine.show());
        engine.exec_cmd(Cmd::New(executor.into()))?;
    }

    std::thread::sleep(Duration::from_secs(2));

    {
        let mut engine = engine.lock().unwrap();

        let dimmer_preset = DimmerPreset::new(
            1,
            PresetContent::Selective({
                let mut content = SelectivePreset::default();
                content.set_attribute_value(
                    FixtureId(1),
                    Attribute::Dimmer,
                    AttributeValue::new(0.25),
                );
                content
            }),
        );
        engine.exec_cmd(Cmd::New(dimmer_preset.clone().into()))?;

        let fixture_group = FixtureGroup::new(1).with_fixture(FixtureId(1));
        engine.exec_cmd(Cmd::New(fixture_group.clone().into()))?;

        let sequence = Sequence::new(1).with_cue(Cue::new(CueContent::Recipe(
            Recipe::new().with_combination(fixture_group.id, AnyPresetId::from(dimmer_preset.id)),
        )));
        engine.exec_cmd(Cmd::New(sequence.clone().into()))?;

        let mut executor = Executor::new(1).with_sequence(sequence.id);
        executor.set_active_cue_index(Some(0), engine.show());
        engine.exec_cmd(Cmd::New(executor.into()))?;
    }

    Ok(())
}
