use std::any::TypeId;
use std::collections::HashMap;
use std::path::PathBuf;

use eyre::Context;

use crate::builtin::{self, Objects, Patch, Pools, Programmer};
use crate::cmd::Command;
use crate::comp::ShowfileComponent;
use crate::error::Result;

mod proc;

pub struct Engine {
    showfile_path: PathBuf,
    components: HashMap<TypeId, Box<dyn ShowfileComponent>>,
}

impl Engine {
    pub fn new(showfile_path: PathBuf) -> Self {
        Self { showfile_path, components: HashMap::new() }
    }

    pub fn register_component<T>(&mut self) -> Result<()>
    where
        T: ShowfileComponent,
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        let type_id = TypeId::of::<T>();
        let component = T::read_from_showfile(&self.showfile_path)
            .wrap_err("failed to read component from showfile")?;
        self.components.insert(type_id, Box::new(component));
        Ok(())
    }

    pub fn component<T: ShowfileComponent>(&self) -> &T {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)
            .expect("component not registered")
            .as_any()
            .downcast_ref::<T>()
            .expect("component type mismatch")
    }

    pub fn component_mut<T: ShowfileComponent>(&mut self) -> &mut T {
        let type_id = TypeId::of::<T>();
        self.components
            .get_mut(&type_id)
            .expect("component not registered")
            .as_any_mut()
            .downcast_mut::<T>()
            .expect("component type mismatch")
    }

    pub fn start(&mut self) -> Result<()> {
        builtin::register(self).wrap_err("failed to register builtins")?;

        let showfile_path = self.showfile_path.clone();
        for comp in self.components.values_mut() {
            comp.after_load_from_file(&showfile_path)
                .wrap_err("failed to run post-load initializations")?;
        }

        proc::ctrl_surf::start();
        proc::processor::start();
        proc::protocols::start();

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
    pub fn patch(&self) -> &Patch {
        self.component::<Patch>()
    }

    #[inline]
    pub(crate) fn patch_mut(&mut self) -> &mut Patch {
        self.component_mut::<Patch>()
    }

    #[inline]
    pub fn objects(&self) -> &Objects {
        self.component::<Objects>()
    }

    #[inline]
    pub(crate) fn objects_mut(&mut self) -> &mut Objects {
        self.component_mut::<Objects>()
    }

    #[inline]
    pub fn pools(&self) -> &Pools {
        self.component::<Pools>()
    }

    #[inline]
    pub(crate) fn pools_mut(&mut self) -> &mut Pools {
        self.component_mut::<Pools>()
    }

    #[inline]
    pub fn programmer(&self) -> &Programmer {
        self.component::<Programmer>()
    }

    #[inline]
    pub(crate) fn programmer_mut(&mut self) -> &mut Programmer {
        self.component_mut::<Programmer>()
    }
}
