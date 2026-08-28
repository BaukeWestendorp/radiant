use std::{path::PathBuf, time::Instant};

use anyhow::Context as _;

mod output;
mod patch;
mod trigger;

pub use output::*;
pub use patch::*;
pub use trigger::*;

const RELATIVE_PATCH_PATH: &str = "patch.json";
const RELATIVE_OUTPUT_PATH: &str = "output.json";
const RELATIVE_TRIGGER_PATH: &str = "trigger.json";

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
        let patch: patch::PatchConfig = serde_json::from_str(&patch_str)
            .with_context(|| format!("Failed to parse patch file: {}", patch_path.display()))?;

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
