use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use eyre::Context;

use crate::engine::event::EventHandler;
use crate::engine::protocols::Protocols;
use crate::error::Result;
use crate::pipeline::Pipeline;
use crate::show::{
    Attribute, AttributeValue, FixtureId, Group, Object, ObjectId, ObjectKind, PresetBeam,
    PresetColor, PresetContent, PresetControl, PresetDimmer, PresetFocus, PresetGobo, PresetObject,
    PresetPosition, PresetShapers, PresetVideo, Show,
};
use crate::showfile::Showfile;

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
    command_history: Vec<Command>,
}

impl Engine {
    pub fn new(showfile_path: Option<&PathBuf>) -> Result<Self> {
        let showfile = match showfile_path {
            Some(path) => Showfile::load(path).wrap_err("failed to load showfile")?,
            None => Showfile::default(),
        };

        let pipeline = Arc::new(Mutex::new(Pipeline::new()));

        let protocols = Protocols::new(pipeline.clone(), &showfile.protocols.protocol_config)?;

        let show = Show::new(showfile).wrap_err("failed to create show from showfile")?;

        Ok(Self {
            protocols,
            show: ShowHandle { show: Arc::new(Mutex::new(show)) },
            pipeline,
            event_handler: Arc::new(EventHandler::new()),
            is_running: false,
            command_history: Vec::new(),
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

    pub fn command_history(&self) -> &[Command] {
        &self.command_history
    }

    pub fn pending_events(&self) -> Vec<EngineEvent> {
        self.event_handler.pending_events()
    }

    pub fn drain_pending_events(&self) -> impl IntoIterator<Item = EngineEvent> {
        self.event_handler.drain_pending_events()
    }

    pub fn exec(&mut self, command: Command) -> Result<()> {
        match &command {
            Command::Select { selection } => {
                match selection {
                    Selection::FixtureId(fid) => {
                        self.show.update(|show| show.select_fixture(*fid));
                    }
                    Selection::Object(object) => match object.kind {
                        ObjectKind::Group => {
                            self.show.update(|show| {
                                if let Some(group) =
                                    show.objects.get_by_pool_id::<Group>(object.pool_id)
                                {
                                    for fid in group.fids().to_vec() {
                                        show.select_fixture(fid);
                                    }
                                }
                            });
                        }
                        ObjectKind::Executor => todo!(),
                        ObjectKind::Sequence => todo!(),
                        ObjectKind::PresetDimmer
                        | ObjectKind::PresetPosition
                        | ObjectKind::PresetGobo
                        | ObjectKind::PresetColor
                        | ObjectKind::PresetBeam
                        | ObjectKind::PresetFocus
                        | ObjectKind::PresetControl
                        | ObjectKind::PresetShapers
                        | ObjectKind::PresetVideo => {
                            self.show.update(|show| {
                                if let Some(preset) = show
                                    .objects
                                    .get_any_by_obj_ref(object)
                                    .and_then(|obj| obj.as_preset())
                                {
                                    for fid in preset.fids(show.patch()).to_vec() {
                                        show.select_fixture(fid);
                                    }
                                }
                            });
                        }
                    },
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
                self.show.update(|show| {
                    if !show.selected_fixtures().is_empty() {
                        show.clear_selected_fixtures();
                        self.event_handler.emit_event(EngineEvent::SelectionChanged);
                    } else if !show.programmer().is_empty() {
                        show.programmer.clear();
                    }
                });
            }
            Command::Store { destination } => {
                match destination.kind {
                    ObjectKind::Group => {
                        self.show.update(|show| {
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
                        });
                    }
                    ObjectKind::Executor => todo!(),
                    ObjectKind::Sequence => todo!(),
                    ObjectKind::PresetDimmer => self.store_preset::<PresetDimmer>(destination),
                    ObjectKind::PresetPosition => self.store_preset::<PresetPosition>(destination),
                    ObjectKind::PresetGobo => self.store_preset::<PresetGobo>(destination),
                    ObjectKind::PresetColor => self.store_preset::<PresetColor>(destination),
                    ObjectKind::PresetBeam => self.store_preset::<PresetBeam>(destination),
                    ObjectKind::PresetFocus => self.store_preset::<PresetFocus>(destination),
                    ObjectKind::PresetControl => self.store_preset::<PresetControl>(destination),
                    ObjectKind::PresetShapers => self.store_preset::<PresetShapers>(destination),
                    ObjectKind::PresetVideo => self.store_preset::<PresetVideo>(destination),
                }
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
            Command::Rename { object, name } => {
                self.show.update(|show| {
                    if let Some(object_id) = object.object_id(show) {
                        if let Some(object) = show.objects.get_any_mut(object_id) {
                            object.as_object_mut().set_name(name.clone());
                        }
                    }
                });
            }
            Command::Recall { object } => match object.kind {
                ObjectKind::Group => {
                    self.exec(Command::Select { selection: Selection::Object(object.clone()) })?;
                }
                ObjectKind::Executor => todo!(),
                ObjectKind::Sequence => todo!(),
                ObjectKind::PresetDimmer => self.recall_preset::<PresetDimmer>(&object)?,
                ObjectKind::PresetPosition => self.recall_preset::<PresetPosition>(&object)?,
                ObjectKind::PresetGobo => self.recall_preset::<PresetGobo>(&object)?,
                ObjectKind::PresetColor => self.recall_preset::<PresetColor>(&object)?,
                ObjectKind::PresetBeam => self.recall_preset::<PresetBeam>(&object)?,
                ObjectKind::PresetFocus => self.recall_preset::<PresetFocus>(&object)?,
                ObjectKind::PresetControl => self.recall_preset::<PresetControl>(&object)?,
                ObjectKind::PresetShapers => self.recall_preset::<PresetShapers>(&object)?,
                ObjectKind::PresetVideo => self.recall_preset::<PresetVideo>(&object)?,
            },

            Command::Go { executor: _ } => todo!(),

            Command::SetAttribute { fid, attribute, value } => {
                self.show.update(|show| {
                    show.programmer.set_value(*fid, attribute.clone(), *value);
                });
            }

            Command::Save => {
                let showfile = self.show.read(|show| Showfile::from(show));
                showfile.save()?
            }
        }

        self.command_history.push(command);

        Ok(())
    }

    pub fn show(&self) -> &ShowHandle {
        &self.show
    }

    fn store_preset<T>(&self, object: &ObjectReference)
    where
        T: PresetObject + Default + 'static,
    {
        // FIXME: Implement filtering.
        self.show.update(|show| {
            let preset_values = show
                .programmer()
                .values()
                .into_iter()
                .map(|(fid, attr, value)| (*fid, attr.clone(), *value))
                .collect::<Vec<_>>();

            fn insert_values(
                content: &mut PresetContent,
                show: &Show,
                preset_values: &[(FixtureId, Attribute, AttributeValue)],
            ) {
                for (fid, attribute, value) in preset_values {
                    let attribute = attribute.clone();
                    match content {
                        PresetContent::Universal(preset) => {
                            preset.set_value(attribute, *value);
                        }
                        PresetContent::Global(preset) => {
                            if let Some(fixture) = show.patch().fixture(*fid) {
                                let fixture_type_id = fixture.fixture_type_id();
                                preset.set_value(*fixture_type_id, attribute, *value);
                            }
                        }
                        PresetContent::Selective(preset) => {
                            preset.set_value(*fid, attribute, *value);
                        }
                    }
                }
            }

            if show.objects().get_any_by_obj_ref(object).is_none() {
                show.objects.insert(
                    T::create(ObjectId::new(), object.pool_id, "New Dimmer Preset".to_string())
                        .into_any_object(),
                );
            }

            let preset = show.objects.get_by_pool_id::<T>(object.pool_id).unwrap();
            let mut content = preset.content().clone();
            insert_values(&mut content, show, &preset_values);
            let preset = show.objects.get_mut_by_pool_id::<T>(object.pool_id).unwrap();
            *preset.content_mut() = content;
        });
    }

    fn recall_preset<T>(&mut self, object: &ObjectReference) -> Result<()>
    where
        T: PresetObject + Default + 'static,
    {
        self.exec(Command::Select { selection: Selection::Object(object.clone()) })?;

        let Some(content) = self.show.read(|show| {
            show.objects.get_by_pool_id::<T>(object.pool_id).map(|preset| preset.content().clone())
        }) else {
            return Ok(());
        };

        match content {
            PresetContent::Universal(preset) => {
                let fids = self.show.read(|show| show.selected_fixtures().to_vec());
                for selected_fid in fids {
                    for (attribute, &value) in preset.values() {
                        self.exec(Command::SetAttribute {
                            fid: selected_fid,
                            attribute: attribute.clone(),
                            value,
                        })?;
                    }
                }
            }
            PresetContent::Global(preset) => {
                let fids = self.show.read(|show| show.selected_fixtures().to_vec());
                for selected_fid in fids {
                    for ((fixture_type_id, attribute), &value) in preset.values() {
                        let has_fti = self.show.read(|show| {
                            show.patch()
                                .fixture(selected_fid)
                                .is_some_and(|f| f.fixture_type_id() == fixture_type_id)
                        });

                        if has_fti {
                            self.exec(Command::SetAttribute {
                                fid: selected_fid,
                                attribute: attribute.clone(),
                                value,
                            })?;
                        }
                    }
                }
            }
            PresetContent::Selective(preset) => {
                let fids = self.show.read(|show| show.selected_fixtures().to_vec());
                for selected_fid in fids {
                    for ((fid, attribute), &value) in preset.values() {
                        if selected_fid == *fid {
                            self.exec(Command::SetAttribute {
                                fid: selected_fid,
                                attribute: attribute.clone(),
                                value,
                            })?;
                        }
                    }
                }
            }
        }

        Ok(())
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
