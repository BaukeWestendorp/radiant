mod artnet;

pub use artnet::*;

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OutputConfig {
    pub artnet: artnet::ArtnetOutputConfig,
}
