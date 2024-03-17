use anyhow::{anyhow, Result};

use gpui::SharedString;

use crate::dmx::DmxOutput;

use self::layout::Layout;
use self::patch::PatchList;
use self::presets::Presets;

pub mod layout;
pub mod patch;
pub mod presets;

pub use layout::PoolWindow;
pub use layout::Window;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Show {
    pub name: SharedString,
    pub presets: Presets,
    pub layout: Layout,
    pub patch_list: PatchList,

    #[serde(skip)]
    pub dmx_output: DmxOutput,
}

impl Show {
    pub fn from_file(path: &str) -> Result<Self> {
        let show_json = std::fs::read_to_string(path)
            .map_err(|e| anyhow!("Failed to read show file '{}': {}", path, e))?;
        let loaded_show = serde_json::from_str(&show_json)
            .map_err(|e| anyhow!("Failed to parse show file '{}': {}", path, e))?;
        Ok(loaded_show)
    }

    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let show_json = serde_json::to_string_pretty(self)
            .map_err(|e| anyhow!("Failed to serialize show to json: {}", e))?;
        std::fs::write(path, show_json)
            .map_err(|e| anyhow!("Failed to write show file '{}': {}", path, e))?;
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Programmer {}
