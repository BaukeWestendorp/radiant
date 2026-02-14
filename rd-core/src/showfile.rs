use std::{
    fs,
    path::{Path, PathBuf},
};

use zeevonk::project::ProjectFile as ZeevonkProjectFile;

use crate::object::{Effect, Group, Object, ObjectRegistry};

#[derive(Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Showfile {
    showfile_name: String,
}

impl Showfile {
    pub fn showfile_name(&self) -> &str {
        &self.showfile_name
    }

    /// Loads a showfile from a directory path.
    pub fn load_from_dir(
        path: impl AsRef<Path>,
    ) -> Result<(Self, ObjectRegistry, ZeevonkProjectFile), crate::Error> {
        let root = path.as_ref();
        let manifest_file = root.join("project.json");

        // Read the manifest.
        let manifest_content = fs::read_to_string(&manifest_file)
            .map_err(|_| crate::Error::ProjectJsonNotFound(root.to_path_buf()))?;

        let showfile: Showfile = serde_json::from_str(&manifest_content)?;
        let mut registry = ObjectRegistry::new();

        // Load objects.
        load_collection_from_file::<Effect>(&mut registry, root.join("obj/effects.json"))?;
        load_collection_from_file::<Group>(&mut registry, root.join("obj/groups.json"))?;

        // Load zeevonk project file.
        let zv_project_file = ZeevonkProjectFile::load_from_folder(&root.join("zv/"))?;

        Ok((showfile, registry, zv_project_file))
    }

    /// Saves the project.
    pub fn save_to_dir(
        &self,
        registry: &ObjectRegistry,
        path: impl AsRef<Path>,
    ) -> Result<(), crate::Error> {
        let root = path.as_ref();
        fs::create_dir_all(root)?;

        let manifest_json = serde_json::to_string_pretty(self)?;
        fs::write(root.join("project.json"), manifest_json)?;

        save_collection_to_file::<Effect>(registry, root.join("obj/effects.json"))?;
        save_collection_to_file::<Group>(registry, root.join("obj/groups.json"))?;

        Ok(())
    }
}

fn load_collection_from_file<T>(
    registry: &mut ObjectRegistry,
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
        registry.insert(object);
    }
    Ok(())
}

fn save_collection_to_file<T>(registry: &ObjectRegistry, file: PathBuf) -> Result<(), crate::Error>
where
    T: Object + Clone + serde::Serialize + 'static,
{
    if let Some(parent) = file.parent() {
        fs::create_dir_all(parent)?;
    }

    let items: Vec<_> = registry.get_all::<T>().into_iter().cloned().collect();
    let json = serde_json::to_string_pretty(&items)?;
    fs::write(file, json)?;
    Ok(())
}
