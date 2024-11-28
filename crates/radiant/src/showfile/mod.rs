pub mod assets;
pub mod dmx_protocols;
pub mod patch;
pub mod windows;

pub use assets::*;
pub use dmx_protocols::*;
pub use patch::*;
pub use windows::*;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Showfile {
    #[serde(default)]
    patch: Patch,
    #[serde(default)]
    assets: Assets,
    #[serde(default)]
    dmx_protocols: DmxProtocols,
    #[serde(default)]
    windows: Windows,
}

impl Showfile {
    pub fn patch(&self) -> &Patch {
        &self.patch
    }

    pub fn patch_mut(&mut self) -> &mut Patch {
        &mut self.patch
    }

    pub fn assets(&self) -> &Assets {
        &self.assets
    }

    pub fn assets_mut(&mut self) -> &mut Assets {
        &mut self.assets
    }

    pub fn dmx_protocols(&self) -> &DmxProtocols {
        &self.dmx_protocols
    }

    pub fn dmx_protocols_mut(&mut self) -> &mut DmxProtocols {
        &mut self.dmx_protocols
    }

    pub fn windows(&self) -> &Windows {
        &self.windows
    }

    pub fn windows_mut(&mut self) -> &mut Windows {
        &mut self.windows
    }
}

impl gpui::Global for Showfile {}
