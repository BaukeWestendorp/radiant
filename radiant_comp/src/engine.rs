use std::any::TypeId;
use std::collections::HashMap;
use std::path::PathBuf;

use eyre::Context;

use crate::builtin::{self, Objects, Patch, Pools};
use crate::cmd::CommandDefinition;
use crate::comp::ShowfileComponent;
use crate::error::Result;

mod proc;

pub struct Engine {
    showfile_path: PathBuf,
    commands: Vec<CommandDefinition>,
    components: HashMap<TypeId, Box<dyn ShowfileComponent>>,
}

impl Engine {
    pub fn new(showfile_path: PathBuf) -> Self {
        Self { showfile_path, commands: Vec::new(), components: HashMap::new() }
    }

    pub fn register_command(&mut self, cmd: CommandDefinition) {
        self.commands.push(cmd);
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

    pub fn patch(&self) -> &Patch {
        self.component::<Patch>()
    }

    pub fn objects(&self) -> &Objects {
        self.component::<Objects>()
    }

    pub fn pools(&self) -> &Pools {
        self.component::<Pools>()
    }
}
