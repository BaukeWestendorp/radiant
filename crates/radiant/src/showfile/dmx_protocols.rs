use dmx::DmxUniverseId;
use std::net::Ipv4Addr;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct DmxProtocols {
    artnet: Vec<ArtnetNodeSettings>,
}

impl DmxProtocols {
    pub fn artnet(&self) -> &[ArtnetNodeSettings] {
        &self.artnet
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArtnetNodeSettings {
    pub destination_ip: Ipv4Addr,
    pub universe: DmxUniverseId,
}
