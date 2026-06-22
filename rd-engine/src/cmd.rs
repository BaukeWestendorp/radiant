use std::{sync::Arc, time::Instant};

use anyhow::Context;

use crate::{
    Engine, FixtureCollection,
    event::Event,
    gdtf::attr::AttributeName,
    object::{
        Executor, ExecutorButton, ExecutorButtonAction, ExecutorContent, ExecutorId, Object,
        ObjectId, ObjectKind, Preset, PresetContent, PresetId, PresetKind, Slot,
    },
    patch::FixtureId,
    value::{AttributeValue, AttributeValues},
};

pub enum Command {
    Activate { object_kind: ObjectKind, object_id: ObjectId },
    // FIXME: Convert these to FixtureCollections.
    SelectionAdd { fixture_ids: Vec<FixtureId> },
    SelectionRemove { fixture_ids: Vec<FixtureId> },
    SelectionSet { fixture_ids: Vec<FixtureId> },
    SelectionClear,
    SelectionAll,

    HighlightToggle,
    Highlight { enabled: bool },

    ExecutorSetMaster { executor_id: ExecutorId, value: f32 },
    ExecutorToggleEnabled { executor_id: ExecutorId },
    ExecutorSetEnabled { executor_id: ExecutorId, value: bool },
    ExecutorButton { executor_id: ExecutorId, button: ExecutorButton, pressed: bool },

    ProgrammerSet { fixtures: FixtureCollection, attribute: AttributeName, value: AttributeValue },
    ProgrammerActivate { fixtures: FixtureCollection, attribute: AttributeName },
    ProgrammerClear,

    Store { kind: StoreKind },

    EncoderSetValue { encoder_ix: usize, value: f32 },
}

impl Command {
    pub fn execute(self, engine: &mut Engine) -> anyhow::Result<()> {
        match self {
            Command::Activate { object_kind, object_id } => match object_kind {
                ObjectKind::Group => {
                    let group = engine.objects().groups().get_by_object_id(&object_id)?;
                    let fixture_ids = group.fixture_ids().to_vec();
                    Command::SelectionAdd { fixture_ids }.execute(engine)?;
                }
                ObjectKind::Sequence => {}
                ObjectKind::ExecutorPage => {}
                ObjectKind::LayoutPage => {}
                ObjectKind::Preset(preset_kind) => {
                    let preset = engine
                        .objects()
                        .preset_by_object_id(&PresetId::new(preset_kind, object_id))?;

                    let mut programmer_values = AttributeValues::new();
                    for fixture_id in engine.selection().fixture_ids() {
                        let fixture = engine.patch().fixture(&fixture_id).with_context(|| {
                            format!("Could not find fixture with id {fixture_id}")
                        })?;

                        for (attribute, value) in preset.universal() {
                            programmer_values.set(*fixture_id, attribute.clone(), *value);
                        }

                        for (fixture_type_id, values) in preset.global() {
                            if fixture.gdtf().fixture_type_id() == *fixture_type_id {
                                for (attribute, value) in values {
                                    programmer_values.set(*fixture_id, attribute.clone(), *value);
                                }
                            }
                        }

                        for (preset_fixture_id, values) in preset.selective() {
                            if *fixture_id == *preset_fixture_id {
                                for (attribute, value) in values {
                                    programmer_values.set(*fixture_id, attribute.clone(), *value);
                                }
                            }
                        }
                    }

                    let programmer = Arc::make_mut(&mut engine.programmer);
                    for (fixture_id, attribute, value) in programmer_values.values() {
                        programmer.set(*fixture_id, attribute.clone(), *value);
                    }
                }
            },
            Command::SelectionAdd { fixture_ids } => {
                let selection = Arc::make_mut(&mut engine.selection);
                for fixture_id in fixture_ids {
                    if !selection.contains(&fixture_id) {
                        selection.fixture_ids.push(fixture_id);
                    }
                }

                engine.emit(Event::SelectionChanged);
            }
            Command::SelectionRemove { fixture_ids } => {
                let selection = Arc::make_mut(&mut engine.selection);
                for fixture_id in fixture_ids {
                    if let Some(pos) = selection.fixture_ids().iter().position(|x| x == &fixture_id)
                    {
                        selection.fixture_ids.remove(pos);
                    }
                }

                engine.emit(Event::SelectionChanged);
            }
            Command::SelectionSet { fixture_ids } => {
                let selection = Arc::make_mut(&mut engine.selection);
                selection.fixture_ids.clear();
                for fixture_id in fixture_ids {
                    if !selection.contains(&fixture_id) {
                        selection.fixture_ids.push(fixture_id);
                    }
                }

                engine.emit(Event::SelectionChanged);
            }
            Command::SelectionClear => {
                let selection = Arc::make_mut(&mut engine.selection);
                selection.fixture_ids.clear();

                engine.emit(Event::SelectionChanged);
            }
            Command::SelectionAll => {
                let fixture_ids = engine.patch().fixture_ids().cloned().collect::<Vec<_>>();

                let selection = Arc::make_mut(&mut engine.selection);
                selection.fixture_ids.clear();
                for fixture_id in fixture_ids {
                    if !selection.contains(&fixture_id) {
                        selection.fixture_ids.push(fixture_id);
                    }
                }

                engine.emit(Event::SelectionChanged);
            }
            Command::HighlightToggle => {
                engine.highlight = !engine.highlight;
                engine.emit(Event::HighlightChanged { enabled: engine.highlight });
            }
            Command::Highlight { enabled } => {
                engine.highlight = enabled;
                engine.emit(Event::HighlightChanged { enabled: engine.highlight });
            }
            Command::ExecutorSetMaster { executor_id, value } => {
                let objects = Arc::make_mut(&mut engine.objects);
                let page = objects.executor_pages.get_by_object_id_mut(&executor_id.page)?;
                let executor = page.executor_mut(executor_id.slot)?;

                let prev_master = executor.master();
                executor.master = value;
                let new_master = executor.master();

                if let Some(ExecutorContent::Sequence(sc)) = executor.content() {
                    if sc.master_controls_enabled() {
                        if prev_master == 0.0 && new_master > 0.0 {
                            executor.set_enabled(true);
                        }
                        if prev_master > 0.0 && new_master == 0.0 {
                            executor.set_enabled(false);
                        }
                    }
                }

                reset_sequence_to_start_if_disabled(executor);

                let object_id = page.id();
                engine.emit(Event::ObjectChanged { kind: ObjectKind::ExecutorPage, object_id });
            }
            Command::ExecutorToggleEnabled { executor_id } => {
                let objects = Arc::make_mut(&mut engine.objects);
                let page = objects.executor_pages.get_by_object_id_mut(&executor_id.page)?;
                let executor = page.executor_mut(executor_id.slot)?;

                executor.set_enabled(!executor.enabled());
                reset_sequence_to_start_if_disabled(executor);

                let object_id = page.id();
                engine.emit(Event::ObjectChanged { kind: ObjectKind::ExecutorPage, object_id });
            }
            Command::ExecutorSetEnabled { executor_id, value } => {
                let objects = Arc::make_mut(&mut engine.objects);
                let page = objects.executor_pages.get_by_object_id_mut(&executor_id.page)?;
                let executor = page.executor_mut(executor_id.slot)?;

                executor.set_enabled(value);
                reset_sequence_to_start_if_disabled(executor);

                let object_id = page.id();
                engine.emit(Event::ObjectChanged { kind: ObjectKind::ExecutorPage, object_id });
            }
            Command::ExecutorButton { executor_id, button, pressed } => {
                let objects = Arc::make_mut(&mut engine.objects);
                let page = objects.executor_pages.get_by_object_id_mut(&executor_id.page)?;
                let executor = page.executor_mut(executor_id.slot)?;

                let action = match executor.content() {
                    Some(ExecutorContent::Sequence(sc)) => match button {
                        ExecutorButton::Button1 => sc.button1(),
                        ExecutorButton::Button2 => sc.button2(),
                        ExecutorButton::Button3 => sc.button3(),
                    },
                    None => return Ok(()),
                };

                match action {
                    ExecutorButtonAction::ToggleEnabled => {
                        if pressed {
                            executor.set_enabled(!executor.enabled());
                            reset_sequence_to_start_if_disabled(executor);
                        }
                    }
                    ExecutorButtonAction::SetEnabled { value } => {
                        if pressed {
                            executor.set_enabled(value);
                            reset_sequence_to_start_if_disabled(executor);
                        }
                    }
                    ExecutorButtonAction::FlashMaster => {
                        if pressed {
                            if executor.flash_restore_master.is_none() {
                                executor.flash_restore_master = Some(executor.master());
                            }
                            executor.master = 1.0;
                        } else {
                            if let Some(prev) = executor.flash_restore_master.take() {
                                executor.master = prev;
                            }
                        }
                    }
                    ExecutorButtonAction::CueGoNext => {
                        let Some(ExecutorContent::Sequence(sc)) = &mut executor.content else {
                            return Ok(());
                        };

                        if pressed {
                            let sequence_obj =
                                objects.sequences.get_by_object_id(&sc.sequence())?;
                            if sc.cue_index() + 1 < sequence_obj.cues().len() {
                                sc.cue_index += 1;
                            }
                        }
                    }
                    ExecutorButtonAction::CueGoPrevious => {
                        let Some(ExecutorContent::Sequence(sc)) = &mut executor.content else {
                            return Ok(());
                        };

                        if pressed {
                            sc.cue_index = sc.cue_index().saturating_sub(1);
                        }
                    }
                }

                match &mut executor.content {
                    Some(ExecutorContent::Sequence(sc)) => sc.last_activation_time = Instant::now(),
                    None => {}
                }

                let object_id = page.id();
                engine.emit(Event::ObjectChanged { kind: ObjectKind::ExecutorPage, object_id });
            }

            Command::ProgrammerSet { fixtures, attribute, value } => {
                let fixture_ids = fixtures
                    .fixture_ids(engine.objects(), engine.patch())?
                    .cloned()
                    .collect::<Vec<_>>();

                let programmer = Arc::make_mut(&mut engine.programmer);
                for fixture_id in fixture_ids {
                    programmer.set(fixture_id, attribute.clone(), value);
                }
                engine.emit(Event::ProgrammerChanged);
            }
            Command::ProgrammerActivate { fixtures, attribute } => {
                let fixture_ids = fixtures
                    .fixture_ids(engine.objects(), engine.patch())?
                    .cloned()
                    .collect::<Vec<_>>();

                for fixture_id in fixture_ids {
                    let Some(value) =
                        engine.pipeline().attribute_values().get(&fixture_id, &attribute)
                    else {
                        continue;
                    };

                    let programmer = Arc::make_mut(&mut engine.programmer);
                    programmer.set(fixture_id, attribute.clone(), value);
                }
                engine.emit(Event::ProgrammerChanged);
            }
            Command::ProgrammerClear => {
                let programmer = Arc::make_mut(&mut engine.programmer);
                programmer.clear();
                engine.emit(Event::ProgrammerChanged);
            }

            Command::Store { kind: StoreKind::Preset { slot, kind } } => {
                let programmer_values = engine.programmer().values();
                let mut filtered_values = AttributeValues::new();
                for (fixture_id, attribute_name, value) in programmer_values.values() {
                    let Some(fixture) = engine.patch().fixture(fixture_id) else { continue };
                    let Some(attribute) = fixture.gdtf().attribute(attribute_name) else {
                        continue;
                    };
                    let Some(fg) = attribute.feature_group(fixture.gdtf()) else { continue };

                    match (fg.name().as_str().to_ascii_lowercase().trim(), kind) {
                        ("dimmer", PresetKind::Dimmer) => {}
                        ("position", PresetKind::Position) => {}
                        ("gobo", PresetKind::Gobo) => {}
                        ("color", PresetKind::Color) => {}
                        ("beam", PresetKind::Beam) => {}
                        ("focus", PresetKind::Focus) => {}
                        ("control", PresetKind::Control) => {}
                        ("shapers", PresetKind::Shapers) => {}
                        ("video", PresetKind::Video) => {}
                        _ => continue,
                    }

                    filtered_values.set(*fixture_id, attribute_name.clone(), *value);
                }

                let preset_content = filtered_values.preset_content(engine.patch());

                let (object_id, name) = match engine.objects().preset_by_slot(&slot, &kind) {
                    Ok(preset) => (preset.id(), preset.name().to_string()),
                    Err(_) => (ObjectId::random(), "New Preset".to_string()),
                };
                let preset = Preset::new(object_id, slot, name);
                let objects = Arc::make_mut(&mut engine.objects);
                let preset = match objects.preset_by_slot_mut(&slot, &kind) {
                    Ok(preset) => preset,
                    Err(_) => {
                        objects.insert_preset(preset, &kind)?;
                        objects.preset_by_slot_mut(&slot, &kind)?
                    }
                };

                // FIXME: Implement overwrite store mode.
                match preset_content {
                    PresetContent::Universal(content) => {
                        preset.universal.merge(content);
                    }
                    PresetContent::Global(content) => {
                        preset.global.merge(content);
                    }
                    PresetContent::Selective(content) => {
                        preset.selective.merge(content);
                    }
                }

                engine.emit(Event::ObjectChanged { kind: ObjectKind::Preset(kind), object_id });
            }

            Command::EncoderSetValue { encoder_ix, value } => {
                engine.emit(Event::EncoderChanged { encoder_ix, value });
            }
        }

        Ok(())
    }
}

fn reset_sequence_to_start_if_disabled(executor: &mut Executor) {
    if executor.enabled() {
        return;
    }

    let Some(ExecutorContent::Sequence(sc)) = &mut executor.content else {
        return;
    };

    if sc.reset_to_start_on_disable() {
        sc.cue_index = 0;
    }
}

pub enum StoreKind {
    Preset { slot: Slot, kind: PresetKind },
}
