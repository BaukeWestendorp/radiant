use std::any::TypeId;
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
    components: HashMap<TypeId, Arc<Mutex<dyn Component>>>,
    pipeline: Arc<Mutex<Pipeline>>,
    event_tx: crossbeam_channel::Sender<EngineEvent>,
    event_rx: crossbeam_channel::Receiver<EngineEvent>,
}

impl Engine {
    pub fn new(showfile_path: Option<PathBuf>) -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();

        let showfile_path = match showfile_path {
            Some(showfile_path) => showfile_path,
            None => create_temp_showfile().expect("failed to crate temporary showfile"),
        };

        Self {
            showfile_path,
            components: HashMap::new(),
            pipeline: Arc::new(Mutex::new(Pipeline::new())),
            event_rx: rx,
            event_tx: tx,
        }
    }

    pub fn showfile_path(&self) -> &PathBuf {
        &self.showfile_path
    }

    pub fn register_component<T>(&mut self) -> Result<()>
    where
        T: Component + Default + serde::Serialize + for<'de> serde::Deserialize<'de>,
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

    pub fn components(&self) -> impl Iterator<Item = &Arc<Mutex<dyn Component>>> {
        self.components.values()
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

fn create_temp_showfile() -> Result<PathBuf> {
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::{env, fs};

    let mut temp_dir = env::temp_dir().join("radiant");

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let folder_name = format!("showfile_{timestamp}");

    temp_dir.push(folder_name);

    fs::create_dir_all(&temp_dir)
        .wrap_err_with(|| format!("failed to create temp showfile directory at {temp_dir:?}"))?;

    Ok(temp_dir)
}
