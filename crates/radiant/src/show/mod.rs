use gpui::SharedString;

use self::data_pools::DataPools;
use self::layout::Layout;
use self::patch::{FixtureId, PatchList};
use self::presets::Presets;

pub mod data_pools;
pub mod layout;
pub mod patch;
pub mod presets;

pub use layout::Window;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Show {
    pub name: SharedString,
    pub presets: Presets,
    pub data_pools: DataPools,
    pub layout: Layout,
    pub patch_list: PatchList,

    pub programmer: Programmer,

    #[serde(skip)]
    pub command_line: SharedString,
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

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Programmer {
    pub selection: Vec<FixtureId>,
}

impl Programmer {
    pub fn new() -> Self {
        Self {
            selection: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.selection.clear();
    }

    pub fn are_fixtures_selected(&self, fixture_ids: &Vec<FixtureId>) -> bool {
        for fixture_id in fixture_ids {
            if !self.selection.contains(&fixture_id) {
                return false;
            }
        }
        true
    }
}
