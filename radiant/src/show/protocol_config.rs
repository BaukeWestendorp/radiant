use std::net::IpAddr;

#[derive(Debug, Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProtocolConfig {
    pub(crate) sacn_source_configurations: Vec<SacnSourceConfiguration>,
}

/// Configuration for a single sACN source.
///
/// A [SacnSourceConfiguration] describes how one or more local DMX universes
/// are mapped to a destination universe and transmitted, including network
/// addressing and output type.
#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SacnSourceConfiguration {
    pub(crate) name: String,
    pub(crate) priority: u8,
    pub(crate) preview_data: bool,
    pub(crate) r#type: SacnOutputType,
}

/// Specifies the output type for an sACN source.
///
/// The [SacnOutputType] determines how DMX data is transmitted over the
/// network, such as unicast to a specific IP address.
#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SacnOutputType {
    /// Unicast output to a specific destination IP address.
    Unicast {
        /// The destination IP.
        destination_ip: IpAddr,
    },
}
