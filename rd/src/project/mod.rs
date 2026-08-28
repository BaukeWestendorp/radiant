use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};

use anyhow::Context as _;
use rd_rigger::gdtf::{FixtureTypeId, Gdtf};

mod output;
mod patch;
mod trigger;

pub use output::*;
pub use patch::*;
pub use trigger::*;

const RELATIVE_PATCH_PATH: &str = "patch.json";
const RELATIVE_OUTPUT_PATH: &str = "output.json";
const RELATIVE_TRIGGER_PATH: &str = "trigger.json";
const RELATIVE_GDTF_FOLDER_PATH: &str = "gdtf/";

#[derive(Default, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Project {
    pub path: Option<PathBuf>,

    pub patch: patch::PatchConfig,
    pub output: output::OutputConfig,
    pub trigger: trigger::TriggerConfig,
}

impl Project {
    pub fn save_to_folder(&self) -> anyhow::Result<()> {
        let started_at = Instant::now();
        let project_path = self
            .path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<unsaved project>".to_string());

        log::info!("Saving project to disk: '{}'", project_path);

        log::error!("FIXME: Save project to disk: '{}'", project_path);

        log::info!("Project saved to disk in {:?}: '{}'", started_at.elapsed(), project_path);

        Ok(())
    }

    pub fn load_from_folder(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let started_at = Instant::now();
        let path = path.into();

        let patch_path = path.join(RELATIVE_PATCH_PATH);
        let patch_str = std::fs::read_to_string(&patch_path)
            .with_context(|| format!("Failed to read patch file: {}", patch_path.display()))?;
        let mut patch: patch::PatchConfig = serde_json::from_str(&patch_str)
            .with_context(|| format!("Failed to parse patch file: {}", patch_path.display()))?;
        let gdtfs_path = path.join(RELATIVE_GDTF_FOLDER_PATH);
        patch.gdtfs = load_gdtfs_from_folder(&gdtfs_path)
            .with_context(|| format!("Failed to read GDTF files: {}", patch_path.display()))?;

        let output_path = path.join(RELATIVE_OUTPUT_PATH);
        let output_str = std::fs::read_to_string(&output_path)
            .with_context(|| format!("Failed to read output file: {}", output_path.display()))?;
        let output: output::OutputConfig = serde_json::from_str(&output_str)
            .with_context(|| format!("Failed to parse output file: {}", output_path.display()))?;

        let trigger_path = path.join(RELATIVE_TRIGGER_PATH);
        let trigger_str = std::fs::read_to_string(&trigger_path)
            .with_context(|| format!("Failed to read trigger file: {}", trigger_path.display()))?;
        let trigger: trigger::TriggerConfig = serde_json::from_str(&trigger_str)
            .with_context(|| format!("Failed to parse trigger file: {}", trigger_path.display()))?;

        log::info!("Project loaded from disk in {:?}: '{}'", started_at.elapsed(), path.display());

        Ok(Self { path: Some(path.into()), patch, output, trigger })
    }
}

fn load_gdtfs_from_folder(folder: &Path) -> anyhow::Result<HashMap<FixtureTypeId, Arc<Gdtf>>> {
    fn visit_dir(
        base: &Path,
        dir: &Path,
        out: &mut HashMap<FixtureTypeId, Arc<Gdtf>>,
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

            let bytes = std::fs::read(&path)
                .with_context(|| format!("Failed to read GDTF file: {}", path.display()))?;
            let parsed = Gdtf::from_archive_bytes(&bytes);

            out.insert(parsed.fixture_type_id(), Arc::new(parsed));
        }

        Ok(())
    }

    let mut out = HashMap::new();
    if folder.exists() {
        visit_dir(folder, folder, &mut out)?;
    }

    Ok(out)
}
