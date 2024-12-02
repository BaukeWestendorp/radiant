pub mod assets;
pub mod dmx_protocols;
pub mod layout;
pub mod patch;
mod showfile;

use gpui::{AppContext, Context, Model};
use std::path::PathBuf;

pub use assets::*;
pub use dmx_protocols::*;
pub use layout::*;
pub use patch::*;

pub struct Show {
    pub assets: Assets,
    pub patch: Model<Patch>,
    pub dmx_protocols: Model<DmxProtocols>,
    pub layout: Layout,
}

impl Show {
    pub fn new(cx: &mut AppContext) -> Self {
        Self {
            assets: Assets::from_showfile(showfile::Assets::default(), cx),
            patch: cx.new_model(|_| {
                showfile::Patch::default().try_into().expect(
                    "empty showfile::Patch should always be possible to convert to a show::Patch",
                )
            }),
            dmx_protocols: cx.new_model(|_| showfile::DmxProtocols::default().into()),
            layout: Layout::from_showfile(showfile::Layout::default(), cx),
        }
    }

    pub fn try_read(showfile_path: &PathBuf, cx: &mut AppContext) -> anyhow::Result<Self> {
        let showfile = showfile::Showfile::try_read(showfile_path)?;

        Ok(Self {
            assets: Assets::from_showfile(showfile.assets, cx),
            patch: {
                let patch = showfile.patch.try_into()?;
                cx.new_model(|_| patch)
            },
            dmx_protocols: cx.new_model(|_| showfile.dmx_protocols.into()),
            layout: Layout::from_showfile(showfile.layout, cx),
        })
    }
}
