use crate::show::ProtocolSettings;
use gpui::{App, Entity};
use sacn::SacnOutputProtocolManager;

mod sacn;

/// Manages all output protocols.
pub struct OutputProtocolManager {
    /// sACN Protocol Manager.
    sacn: SacnOutputProtocolManager,
}

impl OutputProtocolManager {
    /// Creates a new [OutputProtocolManager].
    pub fn new(
        settings: &ProtocolSettings,
        multiverse: Entity<dmx::Multiverse>,
        cx: &App,
    ) -> anyhow::Result<Self> {
        let sacn = SacnOutputProtocolManager::new(settings, multiverse, cx)?;
        Ok(Self { sacn })
    }

    /// Starts all output protocols.
    pub fn start(&self, cx: &mut App) {
        self.sacn.start(cx);
    }
}
