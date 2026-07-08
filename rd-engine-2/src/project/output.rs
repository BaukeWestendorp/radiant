#[derive(Default, Clone)]
#[derive(facet::Facet)]
pub struct OutputConfig {
    pub sacn: SacnOutputConfig,
}

#[derive(Default, Clone)]
#[derive(facet::Facet)]
pub struct SacnOutputConfig {}
