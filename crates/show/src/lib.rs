pub mod assets;
pub mod attr;
pub mod dmx_protocols;
pub mod layout;
pub mod patch;
mod showfile;

use gpui::{AppContext, Context, Model};
use std::path::PathBuf;

pub use assets::*;
pub use attr::*;
pub use dmx_protocols::*;
pub use layout::*;
pub use patch::*;

#[derive(Debug, Clone)]
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
                Patch::try_from_showfile(showfile::Patch::default()).expect(
                    "empty showfile::Patch should always be possible to convert to a show::Patch",
                )
            }),
            dmx_protocols: cx
                .new_model(|_| DmxProtocols::from_showfile(showfile::DmxProtocols::default())),

            layout: Layout::from_showfile(showfile::Layout::default(), cx),
        }
    }

    pub fn try_read(showfile_path: &PathBuf, cx: &mut AppContext) -> anyhow::Result<Self> {
        let showfile = showfile::Showfile::try_read(showfile_path)?;

        Ok(Self {
            assets: Assets::from_showfile(showfile.assets, cx),
            patch: {
                let patch = Patch::try_from_showfile(showfile.patch)?;
                cx.new_model(|_| patch)
            },
            dmx_protocols: cx.new_model(|_| DmxProtocols::from_showfile(showfile.dmx_protocols)),
            layout: Layout::from_showfile(showfile.layout, cx),
        })
    }
}
