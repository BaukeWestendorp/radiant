pub mod assets;
pub mod dmx_protocols;
pub mod layout;
pub mod patch;

pub use assets::*;
pub use dmx_protocols::*;
pub use layout::*;
pub use patch::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Showfile {
    pub assets: Assets,
    pub patch: Patch,
    pub dmx_protocols: DmxProtocols,
    pub layout: Layout,
}
