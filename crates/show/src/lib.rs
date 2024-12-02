pub mod assets;
pub mod dmx_protocols;
pub mod layout;
pub mod patch;

pub use assets::*;
pub use dmx_protocols::*;
pub use layout::*;
pub use patch::*;

use gpui::{AppContext, Context, Model};

use showfile::Showfile;

pub struct Show {
    pub assets: Assets,
    pub patch: Model<Patch>,
    pub dmx_protocols: Model<DmxProtocols>,
    pub layout: Model<Layout>,
}

impl Show {
    pub fn try_from_showfile(showfile: Showfile, cx: &mut AppContext) -> anyhow::Result<Self> {
        Ok(Self {
            assets: Assets::from_showfile(showfile.assets, cx),
            patch: {
                let patch = showfile.patch.try_into()?;
                cx.new_model(|_| patch)
            },
            dmx_protocols: cx.new_model(|_| showfile.dmx_protocols.into()),
            layout: cx.new_model(|_| showfile.layout.into()),
        })
    }
}
