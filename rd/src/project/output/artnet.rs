use rd_artnet::PortAddress;

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ArtnetOutputConfig {
    pub network: rd_artnet::NodeNetworkConfig,
    pub instances: Vec<ArtnetOutputInstanceConfig>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ArtnetOutputInstanceConfig {
    pub name: String,
    pub port_address: PortAddress,
    pub local_universe: rd_dmx::UniverseId,
}
