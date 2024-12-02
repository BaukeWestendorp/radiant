pub mod assets;
pub mod dmx_protocols;
pub mod layout;
pub mod patch;

pub use assets::*;
pub use dmx_protocols::*;
pub use layout::*;
pub use patch::*;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Showfile {
    pub layout: Layout,
    pub assets: Assets,
    pub dmx_protocols: DmxProtocols,
    pub patch: Patch,
}
