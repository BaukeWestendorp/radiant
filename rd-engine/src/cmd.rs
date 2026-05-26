use zeevonk::project::FixtureId;

use crate::{
    Engine, Event, Executor, ExecutorButton, ExecutorButtonAction, ExecutorContent, ExecutorId,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    SelectionAdd { fixture_ids: Vec<FixtureId> },
    SelectionRemove { fixture_ids: Vec<FixtureId> },
    SelectionSet { fixture_ids: Vec<FixtureId> },
    SelectionClear,
    SelectionAll,

    ExecutorSetMaster { executor_id: ExecutorId, value: f32 },
    ExecutorToggleEnabled { executor_id: ExecutorId },
    ExecutorSetEnabled { executor_id: ExecutorId, value: bool },
    ExecutorButton { executor_id: ExecutorId, button: ExecutorButton, pressed: bool },
}

impl Command {
    pub fn execute(self, engine: &mut Engine) -> anyhow::Result<()> {
        match self {
            Command::SelectionAdd { fixture_ids } => {
                for fixture_id in fixture_ids {
                    if !engine.selection.contains(&fixture_id) {
                        engine.selection.fixtures.push(fixture_id);
                    }
                }
                engine.emit_event(Event::SelectionChanged);
            }
            Command::SelectionRemove { fixture_ids } => {
                for fixture_id in fixture_ids {
                    if let Some(pos) =
                        engine.selection.fixtures().iter().position(|x| x == &fixture_id)
                    {
                        engine.selection.fixtures.remove(pos);
                    }
                }
                engine.emit_event(Event::SelectionChanged);
            }
            Command::SelectionSet { fixture_ids } => {
                engine.selection.fixtures.clear();
                for fixture_id in fixture_ids {
                    if !engine.selection.contains(&fixture_id) {
                        engine.selection.fixtures.push(fixture_id);
                    }
                }
                engine.emit_event(Event::SelectionChanged);
            }
            Command::SelectionClear => {
                engine.selection.fixtures.clear();
                engine.emit_event(Event::SelectionChanged);
            }
            Command::SelectionAll => {
                engine.selection.fixtures.clear();
                let fixture_ids = engine.stage().fixtures().keys().copied().collect::<Vec<_>>();
                for fixture_id in fixture_ids {
                    if !engine.selection.contains(&fixture_id) {
                        engine.selection.fixtures.push(fixture_id);
                    }
                }
                engine.emit_event(Event::SelectionChanged);
            }
            Command::ExecutorSetMaster { executor_id, value } => {
                let page = engine.objects.executor_pages.get_by_object_id_mut(&executor_id.page)?;
                let executor = page.executor_mut(executor_id.executor)?;

                let prev_master = executor.master();
                executor.set_master(value);
                let new_master = executor.master();

                if let Some(ExecutorContent::CueList { master_controls_enabled, .. }) =
                    executor.content()
                {
                    if *master_controls_enabled {
                        if prev_master == 0.0 && new_master > 0.0 {
                            executor.set_enabled(true);
                        }
                        if prev_master > 0.0 && new_master == 0.0 {
                            executor.set_enabled(false);
                        }
                    }
                }

                reset_cue_list_to_start_if_disabled(executor);
                engine.emit_event(Event::ExecutorChanged(executor_id));
            }
            Command::ExecutorToggleEnabled { executor_id } => {
                let page = engine.objects.executor_pages.get_by_object_id_mut(&executor_id.page)?;
                let executor = page.executor_mut(executor_id.executor)?;

                executor.set_enabled(!executor.enabled());
                reset_cue_list_to_start_if_disabled(executor);
                engine.emit_event(Event::ExecutorChanged(executor_id));
            }
            Command::ExecutorSetEnabled { executor_id, value } => {
                let page = engine.objects.executor_pages.get_by_object_id_mut(&executor_id.page)?;
                let executor = page.executor_mut(executor_id.executor)?;
                executor.set_enabled(value);
                reset_cue_list_to_start_if_disabled(executor);
                engine.emit_event(Event::ExecutorChanged(executor_id));
            }
            Command::ExecutorButton { executor_id, button, pressed } => {
                let page = engine.objects.executor_pages.get_by_object_id_mut(&executor_id.page)?;
                let executor = page.executor_mut(executor_id.executor)?;

                let action = match executor.content() {
                    Some(ExecutorContent::CueList { button1, button2, button3, .. }) => {
                        match button {
                            ExecutorButton::Button1 => *button1,
                            ExecutorButton::Button2 => *button2,
                            ExecutorButton::Button3 => *button3,
                        }
                    }
                    None => return Ok(()),
                };

                match action {
                    ExecutorButtonAction::ToggleEnabled => {
                        if pressed {
                            executor.set_enabled(!executor.enabled());
                            reset_cue_list_to_start_if_disabled(executor);
                        }
                    }
                    ExecutorButtonAction::SetEnabled { value } => {
                        if pressed {
                            executor.set_enabled(value);
                            reset_cue_list_to_start_if_disabled(executor);
                        }
                    }
                    ExecutorButtonAction::FlashMaster => {
                        if pressed {
                            executor.flash_master_press();
                        } else {
                            executor.flash_master_release();
                        }
                    }
                    ExecutorButtonAction::CueGoNext => {
                        if pressed {
                            if let Some(ExecutorContent::CueList { cue_list, cue_index, .. }) =
                                executor.content_mut().as_mut()
                            {
                                let cue_list_obj =
                                    engine.objects.cue_lists.get_by_object_id(cue_list)?;
                                if *cue_index + 1 < cue_list_obj.cues().len() {
                                    *cue_index += 1;
                                }
                            }
                        }
                    }
                    ExecutorButtonAction::CueGoPrevious => {
                        if pressed {
                            if let Some(ExecutorContent::CueList { cue_index, .. }) =
                                executor.content_mut().as_mut()
                            {
                                *cue_index = cue_index.saturating_sub(1);
                            }
                        }
                    }
                }
                engine.emit_event(Event::ExecutorChanged(executor_id));
            }
        }

        Ok(())
    }
}

fn reset_cue_list_to_start_if_disabled(executor: &mut Executor) {
    if executor.enabled() {
        return;
    }

    let Some(ExecutorContent::CueList { cue_index, reset_to_start_on_disable, .. }) =
        executor.content_mut().as_mut()
    else {
        return;
    };

    if *reset_to_start_on_disable {
        *cue_index = 0;
    }
}
