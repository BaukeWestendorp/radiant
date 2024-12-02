#[derive(Debug, Clone, PartialEq)]
pub struct DmxProtocols {
    artnet: Vec<ArtnetNodeSettings>,
}

impl DmxProtocols {
    pub fn artnet(&self) -> impl Iterator<Item = &ArtnetNodeSettings> {
        self.artnet.iter()
    }
}

impl From<showfile::DmxProtocols> for DmxProtocols {
    fn from(protocols: showfile::DmxProtocols) -> Self {
        Self {
            artnet: protocols
                .artnet
                .into_iter()
                .map(ArtnetNodeSettings::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArtnetNodeSettings {
    pub destination_ip: std::net::Ipv4Addr,
    pub universe: dmx::DmxUniverseId,
}

impl From<showfile::ArtnetNodeSettings> for ArtnetNodeSettings {
    fn from(settings: showfile::ArtnetNodeSettings) -> Self {
        Self {
            destination_ip: settings.destination_ip,
            universe: settings.universe,
        }
    }
}
