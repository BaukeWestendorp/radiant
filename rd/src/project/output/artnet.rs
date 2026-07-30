use rd_artnet::PortAddress;

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(facet::Facet)]
#[facet(deny_unknown_fields)]
pub struct ArtnetOutputConfig {
    pub network: rd_artnet::NodeNetworkConfig,
    pub instances: Vec<ArtnetOutputInstanceConfig>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(facet::Facet)]
#[facet(deny_unknown_fields)]
pub struct ArtnetOutputInstanceConfig {
    #[facet(facet_validate::min_length = 1, facet_validate::max_length = 17)]
    pub name: String,
    pub port_address: PortAddress,
    pub local_universe: rd_dmx::UniverseId,
}
