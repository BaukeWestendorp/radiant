//! In Radiant, protocols are used to send and receive DMX data. All DMX input or output
//! goes through one of the supported protocols.
//!
//! Supported Protocols:
//! - sACN (basic output supported)

use crate::show::ProtocolSettings;
use gpui::{App, Entity};
use output::OutputProtocolManager;

mod output;

/// Manages all protcols used for DMX IO.
pub struct ProtocolManager {
    /// Output protocols.
    output: OutputProtocolManager,
}

impl ProtocolManager {
    /// Creates a new [ProtocolManager]
    pub fn new(
        output_multiverse: Entity<dmx::Multiverse>,
        settings: &ProtocolSettings,
        cx: &App,
    ) -> anyhow::Result<Self> {
        let output = OutputProtocolManager::new(settings, output_multiverse, cx)?;
        Ok(Self { output })
    }

    /// Starts all input and output protocols.
    pub fn start(&self, cx: &mut App) {
        self.output.start(cx)
    }
}
