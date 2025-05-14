use std::net::IpAddr;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// DMX IO settings.
pub struct DmxIoSettings {
    /// sACN DMX IO settings.
    pub sacn: SacnSettings,
}

/// sACN DMX IO settings.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SacnSettings {
    pub sources: Vec<gpui::Entity<SacnSourceSettings>>,
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

pub(crate) mod showfile {
    use gpui::AppContext as _;

    use super::SacnSourceSettings;

    #[derive(Default)]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct DmxIoSettings {
        pub sacn: SacnSettings,
    }

    impl DmxIoSettings {
        pub fn into_show(self, cx: &mut gpui::App) -> super::DmxIoSettings {
            super::DmxIoSettings { sacn: self.sacn.to_show(cx) }
        }

        pub fn from_show(from: crate::dmx_io::DmxIoSettings, cx: &gpui::App) -> Self {
            Self { sacn: SacnSettings::from_show(from.sacn, cx) }
        }
    }

    #[derive(Default)]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct SacnSettings {
        pub sources: Vec<SacnSourceSettings>,
    }

    impl SacnSettings {
        pub fn to_show(self, cx: &mut gpui::App) -> super::SacnSettings {
            super::SacnSettings {
                sources: self.sources.into_iter().map(|source| cx.new(|_| source)).collect(),
            }
        }

        pub fn from_show(from: crate::dmx_io::SacnSettings, cx: &gpui::App) -> Self {
            Self {
                sources: from.sources.into_iter().map(|source| source.read(cx).clone()).collect(),
            }
        }
    }
}
