use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use eyre::Context;

use crate::engine::event::EventHandler;
use crate::engine::protocols::Protocols;
use crate::pipeline::Pipeline;
use crate::show::{Group, Object, ObjectId, ObjectKind, Show};
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
            Command::Select { selection } => {
                match selection {
                    Selection::FixtureId(fid) => {
                        self.show.update(|show| show.select_fixture(fid));
                    }
                    Selection::Object(object_ref) => {
                        self.show.update(|show| match object_ref.kind {
                            ObjectKind::Group => {
                                if let Some(group) =
                                    show.objects.get_by_pool_id::<Group>(object_ref.pool_id)
                                {
                                    for fid in group.fids().to_vec() {
                                        show.select_fixture(fid);
                                    }
                                }
                            }
                            ObjectKind::Executor => todo!(),
                            ObjectKind::Sequence => todo!(),
                            ObjectKind::PresetDimmer => todo!(),
                            ObjectKind::PresetPosition => todo!(),
                            ObjectKind::PresetGobo => todo!(),
                            ObjectKind::PresetColor => todo!(),
                            ObjectKind::PresetBeam => todo!(),
                            ObjectKind::PresetFocus => todo!(),
                            ObjectKind::PresetControl => todo!(),
                            ObjectKind::PresetShapers => todo!(),
                            ObjectKind::PresetVideo => todo!(),
                        });
                    }
                    Selection::All => {
                        self.show.update(|show| {
                            let fids =
                                show.patch().fixtures().iter().map(|f| f.fid()).collect::<Vec<_>>();
                            for fid in fids {
                                show.select_fixture(fid);
                            }
                        });
                    }
                    Selection::None => {}
                }
                self.event_handler.emit_event(EngineEvent::SelectionChanged);
            }
            Command::Clear => {
                self.show.update(|show| show.clear_selected_fixtures());
                self.event_handler.emit_event(EngineEvent::SelectionChanged);
            }
            Command::Store { destination } => {
                self.show.update(|show| match destination.kind {
                    ObjectKind::Group => {
                        let fids = show.selected_fixtures().to_vec();
                        if let Some(group) =
                            show.objects.get_mut_by_pool_id::<Group>(destination.pool_id)
                        {
                            group.fids = fids;
                        } else {
                            let mut group = Group::create(
                                ObjectId::new(),
                                destination.pool_id,
                                "New Group".to_string(),
                            );
                            group.fids = fids;
                            show.objects.insert(group);
                        }
                    }
                    ObjectKind::Executor => todo!(),
                    ObjectKind::Sequence => todo!(),
                    ObjectKind::PresetDimmer => todo!(),
                    ObjectKind::PresetPosition => todo!(),
                    ObjectKind::PresetGobo => todo!(),
                    ObjectKind::PresetColor => todo!(),
                    ObjectKind::PresetBeam => todo!(),
                    ObjectKind::PresetFocus => todo!(),
                    ObjectKind::PresetControl => todo!(),
                    ObjectKind::PresetShapers => todo!(),
                    ObjectKind::PresetVideo => todo!(),
                });
                self.exec(Command::Clear)?;
            }
            Command::Update { object: _ } => todo!(),
            Command::Delete { object } => {
                self.show.update(|show| {
                    if let Some(object_id) = object.object_id(show) {
                        show.objects.remove(&object_id);
                    }
                });
            }

            Command::Go { executor: _ } => todo!(),

            Command::SetAttribute { fid: _, attribute: _, value: _ } => todo!(),
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
