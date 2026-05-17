use std::{
    fs,
    path::{Path, PathBuf},
};

use zeevonk::project::ProjectFile as ZeevonkProjectFile;

use crate::object::{CueList, Effect, Group, Object, ObjectRegistry};

const PROJECT_FILE_NAME: &str = "project.json";

#[derive(Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Showfile {
    #[serde(skip)]
    path: Option<PathBuf>,

    showfile_name: String,
}

impl Showfile {
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub fn showfile_name(&self) -> &str {
        &self.showfile_name
    }

    /// Loads a showfile from a directory path.
    pub fn load_from_dir(
        path: impl AsRef<Path>,
    ) -> Result<(Self, ObjectRegistry, ZeevonkProjectFile), crate::Error> {
        let root = path.as_ref();
        let manifest_file = root.join(PROJECT_FILE_NAME);

        // Read the manifest.
        let manifest_content = fs::read_to_string(&manifest_file)
            .map_err(|_| crate::Error::ProjectJsonNotFound(root.to_path_buf()))?;
        let mut showfile: Showfile = serde_json::from_str(&manifest_content)?;
        showfile.path = Some(root.to_path_buf());

        // Load objects.
        let mut registry = ObjectRegistry::new();
        load_collection_from_file::<Effect>(&mut registry, root.join("obj/effects.json"))?;
        load_collection_from_file::<Group>(&mut registry, root.join("obj/groups.json"))?;
        load_collection_from_file::<CueList>(&mut registry, root.join("obj/cue_lists.json"))?;

        // Load zeevonk project file.
        let zv_project_file = ZeevonkProjectFile::load_from_folder(&root.join("zv/"))?;

        Ok((showfile, registry, zv_project_file))
    }

    /// Saves the project.
    pub fn save_to_dir(
        &self,
        obj_registry: &ObjectRegistry,
        path: impl AsRef<Path>,
    ) -> Result<(), crate::Error> {
        let root = path.as_ref();
        fs::create_dir_all(root)?;

        let manifest_json = serde_json::to_string_pretty(self)?;
        fs::write(root.join(PROJECT_FILE_NAME), manifest_json)?;

        save_collection_to_file::<Effect>(obj_registry, root.join("obj/effects.json"))?;
        save_collection_to_file::<Group>(obj_registry, root.join("obj/groups.json"))?;
        save_collection_to_file::<CueList>(obj_registry, root.join("obj/cue_lists.json"))?;

        Ok(())
    }
}

fn load_collection_from_file<T>(
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

fn save_collection_to_file<T>(
    obj_registry: &ObjectRegistry,
    file: PathBuf,
) -> Result<(), crate::Error>
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
