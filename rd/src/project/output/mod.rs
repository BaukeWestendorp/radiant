mod artnet;

pub use artnet::*;

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(facet::Facet)]
#[facet(deny_unknown_fields)]
pub struct OutputConfig {
    pub artnet: artnet::ArtnetOutputConfig,
}
