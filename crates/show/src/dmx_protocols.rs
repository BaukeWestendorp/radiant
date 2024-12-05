use crate::showfile;

#[derive(Debug, Clone, PartialEq)]
pub struct DmxProtocols {
    artnet: Vec<ArtnetNodeSettings>,
}

impl DmxProtocols {
    pub fn artnet(&self) -> impl Iterator<Item = &ArtnetNodeSettings> {
        self.artnet.iter()
    }
}

impl DmxProtocols {
    pub(crate) fn from_showfile(protocols: showfile::DmxProtocols) -> Self {
        Self {
            artnet: protocols
                .artnet
                .into_iter()
                .map(ArtnetNodeSettings::from_showfile)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArtnetNodeSettings {
    pub destination_ip: std::net::Ipv4Addr,
    pub universe: dmx::DmxUniverseId,
}

impl ArtnetNodeSettings {
    pub(crate) fn from_showfile(settings: showfile::ArtnetNodeSettings) -> Self {
        Self {
            destination_ip: settings.destination_ip,
            universe: settings.universe,
        }
    }
}
