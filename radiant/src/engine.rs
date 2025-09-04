use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use eyre::Context;

use crate::builtin::{self, Objects, Patch, Pools, Programmer, ProtocolConfig};
use crate::cmd::Command;
use crate::comp::{Component, ComponentHandle};
use crate::engine::event::EngineEvent;
use crate::engine::pipeline::Pipeline;
use crate::error::Result;

pub mod event;
mod pipeline;
mod proc;

pub struct Engine {
    showfile_path: PathBuf,
    components: HashMap<TypeId, Arc<Mutex<dyn Any + Send + Sync>>>,
    pipeline: Arc<Mutex<Pipeline>>,
    event_tx: crossbeam_channel::Sender<EngineEvent>,
    event_rx: crossbeam_channel::Receiver<EngineEvent>,
}

impl Engine {
    pub fn new(showfile_path: PathBuf) -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();

        Self {
            showfile_path,
            components: HashMap::new(),
            pipeline: Arc::new(Mutex::new(Pipeline::new())),
            event_rx: rx,
            event_tx: tx,
        }
    }

    pub fn register_component<T>(&mut self) -> Result<()>
    where
        T: Component + serde::Serialize + for<'de> serde::Deserialize<'de> + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<T>();
        let component = T::load_from_showfile(&self.showfile_path)
            .wrap_err("failed to read component from showfile")?;
        self.components.insert(type_id, Arc::new(Mutex::new(component)));
        Ok(())
    }

    pub fn component<T: Component + 'static>(&self) -> ComponentHandle<T> {
        let type_id = TypeId::of::<T>();
        let component = self.components.get(&type_id).expect("component not registered");
        ComponentHandle::new(component.clone())
    }

    pub fn start(&mut self) -> Result<()> {
        builtin::register(self).wrap_err("failed to register builtins")?;

        proc::ctrl_surf::start();
        proc::processor::start(
            self.objects(),
            self.patch(),
            self.programmer(),
            self.pipeline.clone(),
        );
        proc::protocols::start(self.protocol_config(), self.pipeline.clone());

        Ok(())
    }

    #[inline]
    pub fn exec(&mut self, command: Command) -> Result<()> {
        command.exec(self)
    }

    #[inline]
    pub fn exec_and_log_err(&mut self, command: Command) {
        self.exec(command.clone())
            .map_err(|err| log::error!("failed to run command '{command}': {err}"))
            .ok();
    }

    #[inline]
    pub(crate) fn emit(&mut self, event: EngineEvent) {
        self.event_tx.send(event).map_err(|err| format!("failed to send event: {err}")).ok();
    }

    pub fn event_rx(&self) -> crossbeam_channel::Receiver<EngineEvent> {
        self.event_rx.clone()
    }

    #[inline]
    pub fn patch(&self) -> ComponentHandle<Patch> {
        self.component::<Patch>()
    }

    #[inline]
    pub fn objects(&self) -> ComponentHandle<Objects> {
        self.component::<Objects>()
    }

    #[inline]
    pub fn pools(&self) -> ComponentHandle<Pools> {
        self.component::<Pools>()
    }

    #[inline]
    pub fn programmer(&self) -> ComponentHandle<Programmer> {
        self.component::<Programmer>()
    }

    #[inline]
    pub fn protocol_config(&self) -> ComponentHandle<ProtocolConfig> {
        self.component::<ProtocolConfig>()
    }
}
