use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context as _;

use crate::{
    mvr_gdtf::gdtf::{Gdtf, resource::ResourceKey},
    object::Objects,
    output::OutputDefinition,
    patch::PatchDefinition,
    trigger::TriggersDefinition,
};

#[derive(Default)]
pub struct Project {
    file: Option<ProjectFile>,

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
        let project_file = ProjectFile::load_from_folder(path)?;

        let patch_path = project_file.path().join(project_file.manifest().paths().patch());
        let patch_str = std::fs::read_to_string(&patch_path)
            .with_context(|| format!("Failed to read patch file: {}", patch_path.display()))?;
        let patch: PatchDefinition = ron::from_str(&patch_str)
            .with_context(|| format!("Failed to parse patch file: {}", patch_path.display()))?;

        let output_path = project_file.path().join(project_file.manifest().paths().output());
        let output_str = std::fs::read_to_string(&output_path)
            .with_context(|| format!("Failed to read output file: {}", output_path.display()))?;
        let output: OutputDefinition = ron::from_str(&output_str)
            .with_context(|| format!("Failed to parse output file: {}", output_path.display()))?;

        let triggers_path = project_file.path().join(project_file.manifest().paths().triggers());
        let triggers_str = std::fs::read_to_string(&triggers_path).with_context(|| {
            format!("Failed to read triggers file: {}", triggers_path.display())
        })?;
        let triggers: TriggersDefinition = ron::from_str(&triggers_str).with_context(|| {
            format!("Failed to parse triggers file: {}", triggers_path.display())
        })?;

        let objects_path = project_file.path().join(project_file.manifest().paths().objects());
        let objects_str = std::fs::read_to_string(&objects_path)
            .with_context(|| format!("Failed to read objects file: {}", objects_path.display()))?;
        let objects: Objects = ron::from_str(&objects_str)
            .with_context(|| format!("Failed to parse objects file: {}", objects_path.display()))?;

        Ok(Self { file: Some(project_file), patch, output, triggers, objects })
    }

    pub fn file(&self) -> Option<&ProjectFile> {
        self.file.as_ref()
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

#[derive(Default)]
pub struct ProjectFile {
    path: PathBuf,

    manifest: Manifest,

    gdtfs: HashMap<ResourceKey, Arc<Gdtf>>,
}

impl ProjectFile {
    pub fn load_from_folder(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let path = path.into();

        let manifest_path = path.join(Manifest::FILE_NAME);
        let manifest_str = std::fs::read_to_string(&manifest_path).with_context(|| {
            format!("Failed to read manifest file: {}", manifest_path.display())
        })?;
        let manifest: Manifest = ron::from_str(&manifest_str).with_context(|| {
            format!("Failed to parse manifest file: {}", manifest_path.display())
        })?;

        let gdtf_folder = path.join(manifest.paths.gdtf_folder());
        let gdtfs = load_gdtfs_from_folder(&gdtf_folder)
            .with_context(|| format!("Failed to load GDTF folder: {}", gdtf_folder.display()))?;

        Ok(Self { path, manifest, gdtfs })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    pub fn gdtfs(&self) -> &HashMap<ResourceKey, Arc<Gdtf>> {
        &self.gdtfs
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Manifest {
    pub paths: RelativePaths,
}

impl Manifest {
    pub const FILE_NAME: &str = "manifest.ron";

    pub fn paths(&self) -> &RelativePaths {
        &self.paths
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RelativePaths {
    gdtf_folder: PathBuf,
    patch: PathBuf,
    output: PathBuf,
    triggers: PathBuf,
    objects: PathBuf,
}

impl RelativePaths {
    pub fn gdtf_folder(&self) -> &Path {
        self.gdtf_folder.as_path()
    }

    pub fn patch(&self) -> &Path {
        self.patch.as_path()
    }

    pub fn output(&self) -> &PathBuf {
        &self.output
    }

    pub fn triggers(&self) -> &PathBuf {
        &self.triggers
    }

    pub fn objects(&self) -> &PathBuf {
        &self.objects
    }
}

impl Default for Manifest {
    fn default() -> Self {
        Self { paths: RelativePaths::default() }
    }
}

impl Default for RelativePaths {
    fn default() -> Self {
        Self {
            gdtf_folder: PathBuf::from("gdtf/"),
            patch: PathBuf::from("patch.ron"),
            output: PathBuf::from("output.ron"),
            triggers: PathBuf::from("triggers.ron"),
            objects: PathBuf::from("objects.ron"),
        }
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
