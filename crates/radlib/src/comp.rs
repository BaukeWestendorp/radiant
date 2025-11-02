use std::fs::File;
use std::io::BufReader;
use std::marker::PhantomData;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::error::Result;

pub trait Component: Send + Sync + 'static {
    fn as_any(&self) -> &dyn std::any::Any;

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    fn relative_file_path() -> &'static str
    where
        Self: Sized;

    fn load_from_showfile(showfile_path: &Path) -> Result<Self>
    where
        Self: Default + serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        let file_path = showfile_path.join(Self::relative_file_path());
        let Ok(file) = File::open(&file_path) else {
            return Ok(Self::default());
        };
        let reader = BufReader::new(file);
        let mut this: Self = serde_yaml::from_reader(reader)?;
        this.after_load_from_showfile(showfile_path)?;
        Ok(this)
    }

    fn after_load_from_showfile(&mut self, _showfile_path: &Path) -> Result<()> {
        Ok(())
    }

    fn save_to_showfile(&self, showfile_path: &Path) -> Result<()>;
}

pub struct ComponentHandle<T: Component>(Arc<Mutex<dyn Component>>, PhantomData<T>);

impl<T: Component> ComponentHandle<T> {
    pub(crate) fn new(component: Arc<Mutex<dyn Component>>) -> Self {
        Self(component, PhantomData)
    }

    pub fn read<R, F: FnOnce(&T) -> R>(&self, f: F) -> R {
        let guard = self.0.lock().unwrap();
        let component = guard.as_any().downcast_ref::<T>().expect("Component type mismatch");
        f(component)
    }

    pub(crate) fn update<R, F: FnOnce(&mut T) -> R>(&mut self, f: F) -> R {
        let mut guard = self.0.lock().unwrap();
        let component = guard.as_any_mut().downcast_mut::<T>().expect("Component type mismatch");
        f(component)
    }
}
