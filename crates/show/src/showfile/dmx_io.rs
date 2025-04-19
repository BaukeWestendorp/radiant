use std::net::IpAddr;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// DMX IO preferences.
pub struct DmxIo {
    /// The interface to use for DMX IO.
    pub interface: Interface,
    /// sACN DMX IO preferences.
    pub sacn: Sacn,
}

/// Preferences about the interface to use for DMX IO.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Interface {
    /// The name of the interface to use (e.g. 'en0').
    name: String,
}

/// sACN DMX IO preferences.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Sacn {
    pub outputs: Vec<SacnOutput>,
}

/// sACN DMX Output preferences.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SacnOutput {
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
