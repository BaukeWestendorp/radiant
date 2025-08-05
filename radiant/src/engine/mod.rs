use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use eyre::Context;

use crate::engine::event::EventHandler;
use crate::engine::protocols::Protocols;
use crate::pipeline::Pipeline;
use crate::show::{
    AnyObject, Executor, Group, PresetBeam, PresetColor, PresetControl, PresetDimmer, PresetFocus,
    PresetGobo, PresetPosition, PresetShapers, PresetVideo, Sequence, Show,
};
use crate::showfile::{SacnOutputType, Showfile};

mod command;
mod control_surface;
mod event;
mod processor;
mod protocols;

pub use command::*;
pub use event::EngineEvent;

pub struct Engine {
    protocols: Protocols,
    show: ShowHandle,
    pipeline: Arc<Mutex<Pipeline>>,
    event_handler: Arc<EventHandler>,
    is_running: bool,
}

impl Engine {
    pub fn new(showfile_path: Option<&PathBuf>) -> Result<Self, crate::error::Error> {
        let showfile = match showfile_path {
            Some(path) => Showfile::load(path).wrap_err("failed to load showfile")?,
            None => Showfile::default(),
        };

        let pipeline = Arc::new(Mutex::new(Pipeline::new()));

        let protocols = Protocols::new(pipeline.clone());
        for configuration in &showfile.protocols.sacn_source_configurations {
            let ip = match configuration.r#type {
                SacnOutputType::Unicast { destination_ip } => destination_ip,
            };

            protocols.add_sacn_source(
                configuration.name.clone(),
                ip,
                configuration.priority,
                configuration.preview_data,
            )?;
        }

        let show = Show::new(showfile).wrap_err("failed to create show from showfile")?;

        Ok(Self {
            protocols,
            show: ShowHandle { show: Arc::new(Mutex::new(show)) },
            pipeline,
            event_handler: Arc::new(EventHandler::new()),
            is_running: false,
        })
    }

    pub fn start(&mut self) {
        if self.is_running {
            return;
        };

        processor::start(self.pipeline.clone(), self.show.clone(), self.event_handler.clone());
        self.protocols.start();
        control_surface::start();

        self.is_running = true;
    }

    pub fn pending_events(&self) -> Vec<EngineEvent> {
        self.event_handler.pending_events()
    }

    pub fn drain_pending_events(&self) -> impl IntoIterator<Item = EngineEvent> {
        self.event_handler.drain_pending_events()
    }

    pub fn exec(&self, command: Command) -> Result<(), crate::error::Error> {
        match command {
            Command::PatchAdd { fid, address, type_id, dmx_mode } => {
                self.show()
                    .update(|show| show.patch.insert_fixture(fid, address, type_id, dmx_mode));
            }
            Command::CreateGroup { pool_id, name, fids } => {
                self.show().update(|show| {
                    show.insert_object(Group::new(
                        pool_id,
                        name.unwrap_or("New Group".to_string()),
                        fids,
                    ));
                });
            }
            Command::CreateSequence { pool_id, name } => {
                self.show().update(|show| {
                    show.insert_object(Sequence::new(
                        pool_id,
                        name.unwrap_or("New Sequence".to_string()),
                    ));
                });
            }
            Command::CreateExecutor { pool_id, name } => {
                self.show().update(|show| {
                    show.insert_object(Executor::new(
                        pool_id,
                        name.unwrap_or("New Executor".to_string()),
                    ));
                });
            }
            Command::CreatePresetDimmer { pool_id, name } => {
                self.show().update(|show| {
                    show.insert_object(PresetDimmer::new(
                        pool_id,
                        name.unwrap_or("New Dimmer Preset".to_string()),
                    ));
                });
            }
            Command::CreatePresetPosition { pool_id, name } => {
                self.show().update(|show| {
                    show.insert_object(PresetPosition::new(
                        pool_id,
                        name.unwrap_or("New Position Preset".to_string()),
                    ));
                });
            }
            Command::CreatePresetGobo { pool_id, name } => {
                self.show().update(|show| {
                    show.insert_object(PresetGobo::new(
                        pool_id,
                        name.unwrap_or("New Gobo Preset".to_string()),
                    ));
                });
            }
            Command::CreatePresetColor { pool_id, name } => {
                self.show().update(|show| {
                    show.insert_object(PresetColor::new(
                        pool_id,
                        name.unwrap_or("New Color Preset".to_string()),
                    ));
                });
            }
            Command::CreatePresetBeam { pool_id, name } => {
                self.show().update(|show| {
                    show.insert_object(PresetBeam::new(
                        pool_id,
                        name.unwrap_or("New Beam Preset".to_string()),
                    ));
                });
            }
            Command::CreatePresetFocus { pool_id, name } => {
                self.show().update(|show| {
                    show.insert_object(PresetFocus::new(
                        pool_id,
                        name.unwrap_or("New Focus Preset".to_string()),
                    ));
                });
            }
            Command::CreatePresetControl { pool_id, name } => {
                self.show().update(|show| {
                    show.insert_object(PresetControl::new(
                        pool_id,
                        name.unwrap_or("New Control Preset".to_string()),
                    ));
                });
            }
            Command::CreatePresetShapers { pool_id, name } => {
                self.show().update(|show| {
                    show.insert_object(PresetShapers::new(
                        pool_id,
                        name.unwrap_or("New Shapers Preset".to_string()),
                    ));
                });
            }
            Command::CreatePresetVideo { pool_id, name } => self.show().update(|show| {
                show.insert_object(PresetVideo::new(
                    pool_id,
                    name.unwrap_or("New Video Preset".to_string()),
                ));
            }),
            Command::ProgrammerSetAttribute { fid, attribute, value } => {
                self.show().update(|show| {
                    show.programmer.set_value(fid, attribute, value);
                });
            }
            Command::Go { executor_id } => {
                self.show().update(|show| {
                    let Some(executor) = show.executor(&executor_id) else { return };
                    let Some(sequence_id) = executor.sequence_id else { return };
                    let Some(sequence) = show.sequence_mut(&sequence_id) else { return };

                    sequence.set_current_cue(sequence.next_cue().map(|cue| cue.id().clone()));
                });
            }
            Command::SelectReferencedFixtures { id } => self.show.update(|show| {
                let fids = match show.object(&id) {
                    Some(AnyObject::Group(group)) => group.fids.clone(),
                    Some(AnyObject::Sequence(_)) => Vec::new(),
                    Some(AnyObject::Executor(_)) => Vec::new(),
                    Some(AnyObject::PresetDimmer(preset)) => preset.fixture_ids(show.patch()),
                    Some(AnyObject::PresetPosition(preset)) => preset.fixture_ids(show.patch()),
                    Some(AnyObject::PresetGobo(preset)) => preset.fixture_ids(show.patch()),
                    Some(AnyObject::PresetColor(preset)) => preset.fixture_ids(show.patch()),
                    Some(AnyObject::PresetBeam(preset)) => preset.fixture_ids(show.patch()),
                    Some(AnyObject::PresetFocus(preset)) => preset.fixture_ids(show.patch()),
                    Some(AnyObject::PresetControl(preset)) => preset.fixture_ids(show.patch()),
                    Some(AnyObject::PresetShapers(preset)) => preset.fixture_ids(show.patch()),
                    Some(AnyObject::PresetVideo(preset)) => preset.fixture_ids(show.patch()),
                    None => Vec::new(),
                };
                show.selected_fixtures.extend(fids);
                self.event_handler.emit_event(EngineEvent::SelectionChanged);
            }),
            Command::SelectFixture { fid } => {
                self.show().update(|show| show.selected_fixtures.push(fid));
                self.event_handler.emit_event(EngineEvent::SelectionChanged);
            }
            Command::ClearFixtureSelection => {
                self.show().update(|show| show.selected_fixtures.clear());
                self.event_handler.emit_event(EngineEvent::SelectionChanged);
            }
        }

        Ok(())
    }

    pub fn show(&self) -> &ShowHandle {
        &self.show
    }
}

#[derive(Clone)]
pub struct ShowHandle {
    show: Arc<Mutex<Show>>,
}

impl ShowHandle {
    pub fn read<F: FnOnce(&Show) -> R, R>(&self, f: F) -> R {
        let show = self.show.lock().unwrap();
        f(show.deref())
    }

    pub fn update<F: FnOnce(&mut Show) -> R, R>(&self, f: F) -> R {
        let mut show = self.show.lock().unwrap();
        f(show.deref_mut())
    }
}
