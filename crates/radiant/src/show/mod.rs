use anyhow::{anyhow, Result};

use gpui::SharedString;

use crate::dmx::{DmxChannel, DmxOutput, DmxUniverse};
use crate::dmx_protocols::DmxProtocols;

use self::data_pools::DataPools;
use self::layout::Layout;
use self::patch::PatchList;
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

    #[serde(skip)]
    pub dmx_output: DmxOutput,

    pub dmx_protocols: DmxProtocols,
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

    pub fn init(&mut self) {
        log::info!("Initializing show");

        self.init_dmx_protocols();
        self.init_dmx_output();
    }

    fn init_dmx_protocols(&mut self) {
        log::info!("Initializing DMX protocols");
        for artnet in self.dmx_protocols.artnet.iter_mut() {
            artnet.open();
        }
    }

    fn init_dmx_output(&mut self) {
        log::info!("Initializing DMX output");
        self.dmx_output = DmxOutput::new();
        for fixture in self.patch_list.fixtures.iter() {
            if let Some(patch) = &fixture.patch {
                let universe = DmxUniverse::new(patch.universe).unwrap();
                self.dmx_output.add_universe_if_absent(universe);
            }
        }
    }

    pub fn update_dmx_output(&mut self) {
        for fixture in self.patch_list.fixtures.iter() {
            let patch = match &fixture.patch {
                Some(patch) => patch,
                None => continue,
            };

            for (offset, value) in fixture.dmx_values.iter().enumerate() {
                let channel = DmxChannel {
                    universe: patch.universe,
                    address: patch.address + offset as u16,
                };
                self.dmx_output.set_channel(channel, *value);
            }
        }
    }

    pub fn send_output_over_active_protocols(&self) {
        for artnet in self.dmx_protocols.artnet.iter() {
            artnet.send_dmx_universe(&self.dmx_output);
        }
    }
}
