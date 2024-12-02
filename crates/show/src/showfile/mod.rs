pub mod assets;
pub mod dmx_protocols;
pub mod layout;
pub mod patch;

use std::{fs::File, path::PathBuf};

pub use assets::*;
pub use dmx_protocols::*;
pub use layout::*;
pub use patch::*;

const ASSETS_FILE_PATH: &str = "assets.json";
const PATCH_FILE_PATH: &str = "patch.json";
const DMX_PROTOCOLS_FILE_PATH: &str = "dmx_protocols.json";
const LAYOUT_FILE_PATH: &str = "layout.json";

#[derive(Clone)]
pub struct Showfile {
    pub assets: Assets,
    pub patch: Patch,
    pub dmx_protocols: DmxProtocols,
    pub layout: Layout,
}

impl Showfile {
    pub fn try_read(showfile_path: &PathBuf) -> anyhow::Result<Self> {
        let assets_file = File::open(showfile_path.join(ASSETS_FILE_PATH)).map_err(|err| {
            log::error!("Error opening assets file: {}", err);
            err
        })?;

        let assets = serde_json::from_reader(assets_file).map_err(|err| {
            log::error!("Error parsing assets file: {}", err);
            err
        })?;

        let patch_file = File::open(showfile_path.join(PATCH_FILE_PATH)).map_err(|err| {
            log::error!("Error opening patch file: {}", err);
            err
        })?;

        let patch = serde_json::from_reader(patch_file).map_err(|err| {
            log::error!("Error parsing patch file: {}", err);
            err
        })?;

        let dmx_protocols_file =
            File::open(showfile_path.join(DMX_PROTOCOLS_FILE_PATH)).map_err(|err| {
                log::error!("Error opening dmx protocols file: {}", err);
                err
            })?;

        let dmx_protocols = serde_json::from_reader(dmx_protocols_file).map_err(|err| {
            log::error!("Error parsing dmx protocols file: {}", err);
            err
        })?;

        let layout_file = File::open(showfile_path.join(LAYOUT_FILE_PATH)).map_err(|err| {
            log::error!("Error opening layout file: {}", err);
            err
        })?;

        let layout = serde_json::from_reader(layout_file).map_err(|err| {
            log::error!("Error parsing layout file: {}", err);
            err
        })?;

        Ok(Self {
            assets,
            patch,
            dmx_protocols,
            layout,
        })
    }
}
