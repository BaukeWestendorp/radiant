use std::net::IpAddr;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// DMX IO settings.
pub struct DmxIoSettings {
    /// The interface to use for DMX IO.
    pub interface: InterfaceSettings,
    /// sACN DMX IO settings.
    pub sacn: SacnSettings,
}

/// Preferences about the interface to use for DMX IO.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InterfaceSettings {
    /// The name of the interface to use (e.g. 'en0').
    pub name: String,
}

/// sACN DMX IO settings.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SacnSettings {
    pub sources: Vec<SacnSourceSettings>,
}

/// sACN DMX source settings.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SacnSourceSettings {
    /// The name of this sACN source.
    pub name: String,
    /// The local universes to send over this sACN source.
    pub local_universes: Vec<sacn::UniverseNumber>,
    /// The destination universe for this source.
    pub destination_universe: sacn::UniverseNumber,
    /// The priority of the packets for this source.
    pub priority: u8,
    /// Whether to send the packets as preview data for this source.
    pub preview_data: bool,
    /// The type of sACN output for this source.
    pub r#type: SacnOutputType,
}

/// The type of sACN output to use.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SacnOutputType {
    /// Sends sACN packets using Unicast UDP.
    Unicast { destination_ip: Option<IpAddr> },
}

impl Default for SacnOutputType {
    fn default() -> Self {
        Self::Unicast { destination_ip: None }
    }
}
