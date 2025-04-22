use gpui::{App, AppContext as _, Entity};

use crate::showfile;
pub use crate::showfile::dmx_io::{SacnOutputType, SacnSourceSettings};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// DMX IO settings.
pub struct DmxIoSettings {
    /// sACN DMX IO settings.
    pub sacn: SacnSettings,
}

impl DmxIoSettings {
    pub fn from_showfile(dmx_io_settings: showfile::DmxIoSettings, cx: &mut App) -> Self {
        Self { sacn: SacnSettings::from_showfile(dmx_io_settings.sacn, cx) }
    }

    pub fn to_showfile(&self, cx: &App) -> showfile::DmxIoSettings {
        showfile::DmxIoSettings { sacn: self.sacn.to_showfile(cx) }
    }
}

/// sACN DMX IO settings.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SacnSettings {
    pub sources: Vec<Entity<SacnSourceSettings>>,
}

impl SacnSettings {
    pub fn from_showfile(sacn_settings: showfile::SacnSettings, cx: &mut App) -> Self {
        Self { sources: sacn_settings.sources.into_iter().map(|s| cx.new(|_cx| s)).collect() }
    }

    pub fn to_showfile(&self, cx: &App) -> showfile::SacnSettings {
        showfile::SacnSettings {
            sources: self.sources.iter().map(|s| s.read(cx).clone()).collect(),
        }
    }
}
