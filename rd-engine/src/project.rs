use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context as _;

use crate::{
    Engine,
    mvr_gdtf::gdtf::{Gdtf, resource::ResourceKey},
    object::Objects,
    output::OutputDefinition,
    patch::PatchDefinition,
    trigger::TriggersDefinition,
};

const RELATIVE_GDTF_FOLDER_PATH: &str = "gdtf/";
const RELATIVE_PATCH_PATH: &str = "patch.ron";
const RELATIVE_OUTPUT_PATH: &str = "output.ron";
const RELATIVE_TRIGGERS_PATH: &str = "triggers.ron";
const RELATIVE_OBJECTS_PATH: &str = "objects.ron";

#[derive(Default)]
pub struct Project {
    path: Option<PathBuf>,
    gdtfs: HashMap<ResourceKey, Arc<Gdtf>>,

    patch: PatchDefinition,
    output: OutputDefinition,
    triggers: TriggersDefinition,
    objects: Objects,
}

impl Project {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_from_folder(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let path = path.into();

        let patch_path = path.join(RELATIVE_PATCH_PATH);
        let patch_str = std::fs::read_to_string(&patch_path)
            .with_context(|| format!("Failed to read patch file: {}", patch_path.display()))?;
        let patch: PatchDefinition = ron::de::from_str(&patch_str)
            .with_context(|| format!("Failed to parse patch file: {}", patch_path.display()))
            .inspect_err(|e| log::error!("{:?}", e))?;

        let gdtfs_path = path.join(RELATIVE_GDTF_FOLDER_PATH);
        let gdtfs = load_gdtfs_from_folder(&gdtfs_path)
            .with_context(|| format!("Failed to read GDTF files: {}", patch_path.display()))?;

        let output_path = path.join(RELATIVE_OUTPUT_PATH);
        let output_str = std::fs::read_to_string(&output_path)
            .with_context(|| format!("Failed to read output file: {}", output_path.display()))?;
        let output: OutputDefinition = ron::de::from_str(&output_str)
            .with_context(|| format!("Failed to parse output file: {}", output_path.display()))
            .inspect_err(|e| log::error!("{:?}", e))?;

        let triggers_path = path.join(RELATIVE_TRIGGERS_PATH);
        let triggers_str = std::fs::read_to_string(&triggers_path).with_context(|| {
            format!("Failed to read triggers file: {}", triggers_path.display())
        })?;
        let triggers: TriggersDefinition = ron::de::from_str(&triggers_str)
            .with_context(|| format!("Failed to parse triggers file: {}", triggers_path.display()))
            .inspect_err(|e| log::error!("{:?}", e))?;

        let objects_path = path.join(RELATIVE_OBJECTS_PATH);
        let objects_str = std::fs::read_to_string(&objects_path)
            .with_context(|| format!("Failed to read objects file: {}", objects_path.display()))?;
        let objects: Objects = ron::de::from_str(&objects_str)
            .with_context(|| format!("Failed to parse objects file: {}", objects_path.display()))
            .inspect_err(|e| log::error!("{:?}", e))?;

        Ok(Self { path: Some(path), patch, gdtfs, output, triggers, objects })
    }

    pub fn load_from_engine(path: PathBuf, engine: &mut Engine) -> Self {
        Self {
            path: Some(path),
            gdtfs: engine.patch().gdtfs().clone(),
            patch: engine.patch().definition().clone(),
            output: engine.output_agent().definition().clone(),
            triggers: engine.triggers_agent().definition().clone(),
            objects: engine.objects().clone(),
        }
    }

    pub fn save_to_folder(&self) -> anyhow::Result<()> {
        let ron_config = ron::ser::PrettyConfig::default().compact_arrays(true).struct_names(true);

        let path = self.path.as_ref().ok_or_else(|| {
            anyhow::anyhow!("Cannot save project to folder: project has no associated path")
        })?;

        let patch_path = path.join(RELATIVE_PATCH_PATH);
        let patch_str = ron::ser::to_string_pretty(&self.patch, ron_config.clone())
            .context("Failed to serialize patch")?;
        std::fs::write(&patch_path, patch_str)
            .with_context(|| format!("Failed to write patch file: {}", patch_path.display()))?;

        let output_path = path.join(RELATIVE_OUTPUT_PATH);
        let output_str = ron::ser::to_string_pretty(&self.output, ron_config.clone())
            .context("Failed to serialize output")?;
        std::fs::write(&output_path, output_str)
            .with_context(|| format!("Failed to write output file: {}", output_path.display()))?;

        let triggers_path = path.join(RELATIVE_TRIGGERS_PATH);
        let triggers_str = ron::ser::to_string_pretty(&self.triggers, ron_config.clone())
            .context("Failed to serialize triggers")?;
        std::fs::write(&triggers_path, triggers_str).with_context(|| {
            format!("Failed to write triggers file: {}", triggers_path.display())
        })?;

        let objects_path = path.join(RELATIVE_OBJECTS_PATH);
        let objects_str = ron::ser::to_string_pretty(&self.objects, ron_config.clone())
            .context("Failed to serialize objects")?;
        std::fs::write(&objects_path, objects_str)
            .with_context(|| format!("Failed to write objects file: {}", objects_path.display()))?;

        // FIXME: We should also save the GDTF files, but as I've not yet implemented serializing the `Gdtf`
        // struct this is not easily possible...

        Ok(())
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub fn gdtfs(&self) -> &HashMap<ResourceKey, Arc<Gdtf>> {
        &self.gdtfs
    }

    pub fn patch(&self) -> &PatchDefinition {
        &self.patch
    }

    pub fn output(&self) -> &OutputDefinition {
        &self.output
    }

    pub fn triggers(&self) -> &TriggersDefinition {
        &self.triggers
    }

    pub fn objects(&self) -> &Objects {
        &self.objects
    }
}

fn load_gdtfs_from_folder(folder: &Path) -> anyhow::Result<HashMap<ResourceKey, Arc<Gdtf>>> {
    fn visit_dir(
        base: &Path,
        dir: &Path,
        out: &mut HashMap<ResourceKey, Arc<Gdtf>>,
    ) -> anyhow::Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let ty = entry.file_type()?;

            if ty.is_dir() {
                visit_dir(base, &path, out)?;
                continue;
            }

            if !ty.is_file() {
                continue;
            }

            if !path.extension().is_some_and(|ext| ext == "gdtf") {
                continue;
            }

            let rel = path.strip_prefix(base).unwrap_or(&path);
            let resource_key = ResourceKey::new(rel.to_string_lossy().to_string());

            let bytes = std::fs::read(&path)
                .with_context(|| format!("Failed to read GDTF file: {}", path.display()))?;
            let parsed = Gdtf::from_archive_bytes(&bytes);

            out.insert(resource_key, Arc::new(parsed));
        }

        Ok(())
    }

    let mut out = HashMap::new();
    if folder.exists() {
        visit_dir(folder, folder, &mut out)?;
    }

    Ok(out)
}
