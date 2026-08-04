use std::{
    path::{Path, PathBuf},
    time::Instant,
};

use anyhow::Context as _;
use ariadne::{Color, Label, Report, ReportKind, Source};

mod output;
mod trigger;

pub use output::*;
pub use trigger::*;

const RELATIVE_OUTPUT_PATH: &str = "output.json";
const RELATIVE_TRIGGER_PATH: &str = "trigger.json";

#[derive(Default, Clone, PartialEq)]
#[derive(facet::Facet)]
#[facet(deny_unknown_fields)]
pub struct Project {
    pub path: Option<PathBuf>,

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

        let result = Ok(());

        log::info!("Project saved to disk in {:?}: '{}'", started_at.elapsed(), project_path);

        result
    }

    pub fn load_from_folder(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let started_at = Instant::now();
        let path = path.into();

        let output_path = path.join(RELATIVE_OUTPUT_PATH);
        let output_str = std::fs::read_to_string(&output_path)
            .with_context(|| format!("Failed to read output file: {}", output_path.display()))?;

        let output: output::OutputConfig = facet_json::from_str(&output_str)
            .map_err(|e| anyhow::anyhow!("\n{}", format_parse_error(&output_path, &output_str, e)))
            .with_context(|| format!("Failed to parse output file: {}", output_path.display()))?;

        let trigger_path = path.join(RELATIVE_TRIGGER_PATH);
        let trigger_str = std::fs::read_to_string(&trigger_path)
            .with_context(|| format!("Failed to read trigger file: {}", trigger_path.display()))?;

        let trigger: trigger::TriggerConfig = facet_json::from_str(&trigger_str)
            .map_err(|e| {
                anyhow::anyhow!("\n{}", format_parse_error(&trigger_path, &trigger_str, e))
            })
            .with_context(|| format!("Failed to parse trigger file: {}", trigger_path.display()))?;

        log::info!("Project loaded from disk in {:?}: '{}'", started_at.elapsed(), path.display());

        Ok(Self { path: Some(path.into()), output, trigger })
    }
}

fn format_parse_error(file_path: &Path, source: &str, err: facet_json::DeserializeError) -> String {
    let file_id_owned = file_path.display().to_string();
    let file_id = file_id_owned.as_str();

    let mut buf = Vec::new();

    let offset = err.span.as_ref().map(|s| s.offset as usize).unwrap_or(0);
    let len = err.span.as_ref().map(|s| s.len as usize).unwrap_or(0);

    let mut builder = Report::build(ReportKind::Error, (file_id, offset..(offset + len)))
        .with_message(format!("Invalid configuration data."));

    if let Some(span) = err.span {
        let label_msg = if let Some(p) = err.path {
            format!("Invalid at `{}`", p)
        } else {
            "Error occurred here".to_string()
        };

        let span_offset = span.offset as usize;
        let span_len = span.len as usize;

        builder = builder.with_label(
            Label::new((file_id, span_offset..(span_offset + span_len)))
                .with_message(label_msg)
                .with_color(Color::Red),
        );
    } else if let Some(p) = err.path {
        builder = builder.with_note(format!("Error occurred at path: {}", p));
    }

    builder.finish().write((file_id, Source::from(source)), &mut buf).unwrap();

    String::from_utf8_lossy(&buf).into_owned()
}
