use crate::show::ProtocolConfig;
use crate::showfile::ShowfileComponent;

/// Represents the protocols configuration for a
/// [Showfile][crate::showfile::Showfile].
///
/// The [Protocols] struct contains configuration for all supported output
/// protocols, such as sACN. It is responsible for describing how DMX data is
/// sent to external systems and devices.
#[derive(Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Protocols {
    pub(crate) protocol_config: ProtocolConfig,
}

impl ShowfileComponent for Protocols {
    const RELATIVE_FILE_PATH: &str = "protocols.yaml";
}
