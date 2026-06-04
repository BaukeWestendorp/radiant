use std::time::Instant;

use crate::{
    Engine,
    event::Event,
    object::{
        Executor, ExecutorButton, ExecutorButtonAction, ExecutorContent, ExecutorId, ObjectId,
        ObjectKind,
    },
    patch::FixtureId,
};

pub enum Command {
    Activate { object_kind: ObjectKind, object_id: ObjectId },
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
                ObjectKind::Preset(_) => {}
            },
            Command::SelectionAdd { fixture_ids } => {
                for fixture_id in fixture_ids {
                    if !engine.selection.contains(&fixture_id) {
                        engine.selection.fixtures.push(fixture_id);
                    }
                }
                engine.emit(Event::SelectionChanged);
            }
            Command::SelectionRemove { fixture_ids } => {
                for fixture_id in fixture_ids {
                    if let Some(pos) =
                        engine.selection.fixtures().iter().position(|x| x == &fixture_id)
                    {
                        engine.selection.fixtures.remove(pos);
                    }
                }
                engine.emit(Event::SelectionChanged);
            }
            Command::SelectionSet { fixture_ids } => {
                engine.selection.fixtures.clear();
                for fixture_id in fixture_ids {
                    if !engine.selection.contains(&fixture_id) {
                        engine.selection.fixtures.push(fixture_id);
                    }
                }
                engine.emit(Event::SelectionChanged);
            }
            Command::SelectionClear => {
                engine.selection.fixtures.clear();
                engine.emit(Event::SelectionChanged);
            }
            Command::SelectionAll => {
                engine.selection.fixtures.clear();
                let fixture_ids = engine.patch().fixture_ids().cloned().collect::<Vec<_>>();
                for fixture_id in fixture_ids {
                    if !engine.selection.contains(&fixture_id) {
                        engine.selection.fixtures.push(fixture_id);
                    }
                }
                engine.emit(Event::SelectionChanged);
            }
            Command::HighlightToggle => {
                engine.highlight = !engine.highlight;
                engine.emit(Event::HighlightChanged);
            }
            Command::Highlight { enabled } => {
                engine.highlight = enabled;
                engine.emit(Event::HighlightChanged);
            }
            Command::ExecutorSetMaster { executor_id, value } => {
                let page = engine.objects.executor_pages.get_by_object_id_mut(&executor_id.page)?;

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
                engine.emit(Event::ExecutorChanged(executor_id));
            }
            Command::ExecutorToggleEnabled { executor_id } => {
                let page = engine.objects.executor_pages.get_by_object_id_mut(&executor_id.page)?;
                let executor = page.executor_mut(executor_id.slot)?;

                executor.set_enabled(!executor.enabled());
                reset_sequence_to_start_if_disabled(executor);
                engine.emit(Event::ExecutorChanged(executor_id));
            }
            Command::ExecutorSetEnabled { executor_id, value } => {
                let page = engine.objects.executor_pages.get_by_object_id_mut(&executor_id.page)?;
                let executor = page.executor_mut(executor_id.slot)?;
                executor.set_enabled(value);
                reset_sequence_to_start_if_disabled(executor);
                engine.emit(Event::ExecutorChanged(executor_id));
            }
            Command::ExecutorButton { executor_id, button, pressed } => {
                let page = engine.objects.executor_pages.get_by_object_id_mut(&executor_id.page)?;
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
                                engine.objects.sequences.get_by_object_id(&sc.sequence())?;
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

                engine.emit(Event::ExecutorChanged(executor_id));
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
