use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    thread,
};

use crate::{
    effect::EffectAgent,
    object::{CueList, Effect, Group, Object, ObjectRegistry},
    output::OutputAgent,
    programmer::Programmer,
};

use zeevonk::{Zeevonk, project::ProjectFile as ZeevonkProjectFile};

pub mod compositor;
pub mod effect;
mod error;
mod lua;
pub mod object;
pub mod output;
pub mod parameter;
pub mod programmer;

pub use error::*;

pub use ::zeevonk as zv;

pub struct Engine {
    showfile_path: Option<PathBuf>,

    objects: Arc<ObjectRegistry>,
    programmer: Arc<Programmer>,
    output_agent: Arc<OutputAgent>,
    effect_agent: Arc<EffectAgent>,

    zeevonk: Arc<Zeevonk>,
}

impl Engine {
    pub fn new(showfile_path: Option<PathBuf>) -> Result<Self, crate::Error> {
        let path = showfile_path;

        // Load objects.
        let mut objects = ObjectRegistry::default();
        let mut zv_project_file = ZeevonkProjectFile::default();

        match &path {
            Some(path) => {
                // Load objects.
                load_objects_from_file::<Effect>(&mut objects, path.join("obj/effects.json"))?;
                load_objects_from_file::<Group>(&mut objects, path.join("obj/groups.json"))?;
                load_objects_from_file::<CueList>(&mut objects, path.join("obj/cue_lists.json"))?;

                // Load zeevonk project file.
                zv_project_file = ZeevonkProjectFile::load_from_folder(&path.join("zv/"))?;
            }
            None => {}
        };

        let objects = Arc::new(objects);
        let effect_agent = Arc::new(EffectAgent::new(Arc::clone(&objects), path.clone()));
        let programmer = Arc::new(Programmer::new());
        let zeevonk = Arc::new(Zeevonk::new(zv_project_file)?);
        let output_agent = Arc::new(OutputAgent::new(
            Arc::clone(&objects),
            Arc::clone(&programmer),
            Arc::clone(&effect_agent),
            Arc::clone(&zeevonk),
        ));

        Ok(Self { showfile_path: path, objects, programmer, output_agent, effect_agent, zeevonk })
    }

    pub fn save_to_showfile_dir(
        &self,
        showfile_path: impl AsRef<Path>,
    ) -> Result<(), crate::Error> {
        let path = showfile_path.as_ref();
        fs::create_dir_all(path)?;

        // Save object files.
        save_objects_to_file::<Effect>(self.objects(), path.join("obj/effects.json"))?;
        save_objects_to_file::<Group>(self.objects(), path.join("obj/groups.json"))?;
        save_objects_to_file::<CueList>(self.objects(), path.join("obj/cue_lists.json"))?;

        self.zeevonk().project().file().save_to_folder(&path.join("zv/"))?;

        Ok(())
    }

    pub fn showfile_path(&self) -> Option<&Path> {
        self.showfile_path.as_deref()
    }

    pub fn objects(&self) -> &ObjectRegistry {
        &self.objects
    }

    pub fn programmer(&self) -> &Programmer {
        &self.programmer
    }

    pub fn output_agent(&self) -> &OutputAgent {
        &self.output_agent
    }

    pub fn effect_agent(&self) -> &EffectAgent {
        &self.effect_agent
    }

    pub fn zeevonk(&self) -> &Zeevonk {
        &self.zeevonk
    }

    pub fn start(&self) {
        self.zeevonk().start();
        self.output_agent().start();

        let _ = thread::Builder::new().name("rd_engine_main".to_string()).spawn({
            let output_agent = Arc::clone(&self.output_agent);
            move || {
                let _ = output_agent;
            }
        });
    }
}

fn load_objects_from_file<T>(
    obj_registry: &mut ObjectRegistry,
    file: PathBuf,
) -> Result<(), crate::Error>
where
    T: Object + serde::de::DeserializeOwned + 'static,
{
    if !file.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&file)?;
    let objects: Vec<T> =
        serde_json::from_str(&content).map_err(|e| crate::Error::ParseError(file.clone(), e))?;

    for object in objects {
        obj_registry.insert(object);
    }
    Ok(())
}

fn save_objects_to_file<T>(obj_registry: &ObjectRegistry, file: PathBuf) -> Result<(), crate::Error>
where
    T: Object + Clone + serde::Serialize + 'static,
{
    if let Some(parent) = file.parent() {
        fs::create_dir_all(parent)?;
    }

    let items: Vec<_> = obj_registry.get_all::<T>().into_iter().cloned().collect();
    let json = serde_json::to_string_pretty(&items)?;
    fs::write(file, json)?;
    Ok(())
}
