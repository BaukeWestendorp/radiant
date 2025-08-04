use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use eyre::Context;

use crate::engine::event::EventHandler;
use crate::engine::protocols::Protocols;
use crate::pipeline::Pipeline;
use crate::show::{
    AnyObjectId, Group, PresetBeam, PresetColor, PresetContent, PresetControl, PresetDimmer,
    PresetFocus, PresetGobo, PresetPosition, PresetShapers, PresetVideo, Show,
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
            Command::CreateGroup { id, name, fids } => {
                self.show().update(|show| {
                    show.groups.insert(Group {
                        id,
                        name: name.unwrap_or("New Group".to_string()),
                        fids,
                    });
                });
            }
            Command::CreatePresetDimmer { id, name } => {
                self.show().update(|show| {
                    show.presets_dimmer.insert(PresetDimmer {
                        id,
                        name: name.unwrap_or("New Dimmer Preset".to_string()),
                        content: PresetContent::default(),
                    });
                });
            }
            Command::CreatePresetPosition { id, name } => {
                self.show().update(|show| {
                    show.presets_position.insert(PresetPosition {
                        id,
                        name: name.unwrap_or("New Position Preset".to_string()),
                        content: PresetContent::default(),
                    });
                });
            }
            Command::CreatePresetGobo { id, name } => {
                self.show().update(|show| {
                    show.presets_gobo.insert(PresetGobo {
                        id,
                        name: name.unwrap_or("New Gobo Preset".to_string()),
                        content: PresetContent::default(),
                    });
                });
            }
            Command::CreatePresetColor { id, name } => {
                self.show().update(|show| {
                    show.presets_color.insert(PresetColor {
                        id,
                        name: name.unwrap_or("New Color Preset".to_string()),
                        content: PresetContent::default(),
                    });
                });
            }
            Command::CreatePresetBeam { id, name } => {
                self.show().update(|show| {
                    show.presets_beam.insert(PresetBeam {
                        id,
                        name: name.unwrap_or("New Beam Preset".to_string()),
                        content: PresetContent::default(),
                    });
                });
            }
            Command::CreatePresetFocus { id, name } => {
                self.show().update(|show| {
                    show.presets_focus.insert(PresetFocus {
                        id,
                        name: name.unwrap_or("New Focus Preset".to_string()),
                        content: PresetContent::default(),
                    });
                });
            }
            Command::CreatePresetControl { id, name } => {
                self.show().update(|show| {
                    show.presets_control.insert(PresetControl {
                        id,
                        name: name.unwrap_or("New Control Preset".to_string()),
                        content: PresetContent::default(),
                    });
                });
            }
            Command::CreatePresetShapers { id, name } => {
                self.show().update(|show| {
                    show.presets_shapers.insert(PresetShapers {
                        id,
                        name: name.unwrap_or("New Shapers Preset".to_string()),
                        content: PresetContent::default(),
                    });
                });
            }
            Command::CreatePresetVideo { id, name } => self.show().update(|show| {
                show.presets_video.insert(PresetVideo {
                    id,
                    name: name.unwrap_or("New Video Preset".to_string()),
                    content: PresetContent::default(),
                });
            }),
            Command::ProgrammerSetAttribute { fid, attribute, value } => {
                self.show().update(|show| {
                    show.programmer.set_value(fid, attribute, value);
                });
            }
            Command::Go { executor } => {
                self.show().update(|show| {
                    let Some(executor) = show.executors.get(executor) else { return };
                    let Some(sequence_id) = executor.sequence_id else { return };
                    let Some(sequence) = show.sequences.get_mut(sequence_id) else { return };

                    sequence.set_current_cue(sequence.next_cue().map(|cue| cue.id().clone()));
                });
            }
            Command::SelectReferencedFixtures { id } => self.show.update(|show| {
                let fids = match id {
                    AnyObjectId::Group(id) => {
                        show.groups.get(id).map(|group| group.fids.clone()).unwrap_or_default()
                    }
                    AnyObjectId::Sequence(_) => vec![],
                    AnyObjectId::Executor(_) => vec![],
                    AnyObjectId::PresetDimmer(id) => show
                        .presets_dimmer
                        .get(id)
                        .map(|preset| preset.fixture_ids(show.patch()))
                        .unwrap_or_default(),
                    AnyObjectId::PresetPosition(id) => show
                        .presets_dimmer
                        .get(id)
                        .map(|preset| preset.fixture_ids(show.patch()))
                        .unwrap_or_default(),
                    AnyObjectId::PresetGobo(id) => show
                        .presets_dimmer
                        .get(id)
                        .map(|preset| preset.fixture_ids(show.patch()))
                        .unwrap_or_default(),
                    AnyObjectId::PresetColor(id) => show
                        .presets_dimmer
                        .get(id)
                        .map(|preset| preset.fixture_ids(show.patch()))
                        .unwrap_or_default(),
                    AnyObjectId::PresetBeam(id) => show
                        .presets_dimmer
                        .get(id)
                        .map(|preset| preset.fixture_ids(show.patch()))
                        .unwrap_or_default(),
                    AnyObjectId::PresetFocus(id) => show
                        .presets_dimmer
                        .get(id)
                        .map(|preset| preset.fixture_ids(show.patch()))
                        .unwrap_or_default(),
                    AnyObjectId::PresetControl(id) => show
                        .presets_dimmer
                        .get(id)
                        .map(|preset| preset.fixture_ids(show.patch()))
                        .unwrap_or_default(),
                    AnyObjectId::PresetShapers(id) => show
                        .presets_dimmer
                        .get(id)
                        .map(|preset| preset.fixture_ids(show.patch()))
                        .unwrap_or_default(),
                    AnyObjectId::PresetVideo(id) => show
                        .presets_dimmer
                        .get(id)
                        .map(|preset| preset.fixture_ids(show.patch()))
                        .unwrap_or_default(),
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
