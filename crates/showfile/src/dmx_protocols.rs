#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct DmxProtocols {
    pub artnet: Vec<ArtnetNodeSettings>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ArtnetNodeSettings {
    pub destination_ip: std::net::Ipv4Addr,
    pub universe: dmx::DmxUniverseId,
}
