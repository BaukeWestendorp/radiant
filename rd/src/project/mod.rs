use std::path::PathBuf;

use anyhow::Context as _;

mod output;
mod trigger;

pub use output::*;
pub use trigger::*;

const RELATIVE_OUTPUT_PATH: &str = "output.json";
const RELATIVE_TRIGGER_PATH: &str = "trigger.json";

#[derive(Default, Clone)]
#[derive(facet::Facet)]
pub struct Project {
    pub path: Option<PathBuf>,

    pub output: output::OutputConfig,
    pub trigger: trigger::TriggerConfig,
}

impl Project {
    pub fn save_to_folder(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn load_from_folder(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let path = path.into();

        let output_path = path.join(RELATIVE_OUTPUT_PATH);
        let output_str = std::fs::read_to_string(&output_path)
            .with_context(|| format!("Failed to read output file: {}", output_path.display()))?;
        let output: output::OutputConfig = facet_json::from_str(&output_str)
            .map_err(|e| anyhow::anyhow!("{}", e.path.map(|p| p.to_string()).unwrap_or_default()))
            .with_context(|| format!("Failed to parse output file: {}", output_path.display()))?;

        let trigger_path = path.join(RELATIVE_TRIGGER_PATH);
        let trigger_str = std::fs::read_to_string(&trigger_path)
            .with_context(|| format!("Failed to read trigger file: {}", trigger_path.display()))?;
        let trigger: trigger::TriggerConfig = facet_json::from_str(&trigger_str)
            .map_err(|e| anyhow::anyhow!("{}", e.path.map(|p| p.to_string()).unwrap_or_default()))
            .with_context(|| format!("Failed to parse trigger file: {}", trigger_path.display()))?;

        Ok(Self { path: Some(path.into()), output, trigger })
    }
}
