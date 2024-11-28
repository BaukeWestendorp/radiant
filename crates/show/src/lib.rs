pub mod assets;
pub mod attr_def;
pub mod dmx_protocols;
pub mod patch;

pub use assets::*;
pub use attr_def::*;
pub use dmx_protocols::*;
pub use patch::*;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Show {
    #[serde(default)]
    patch: Patch,
    #[serde(default)]
    assets: Assets,
    #[serde(default)]
    dmx_protocols: DmxProtocols,
}

impl Show {
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
}
