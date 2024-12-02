pub type FixtureId = u32;

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Patch {
    pub fixtures: Vec<Fixture>,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Fixture {
    pub id: FixtureId,
    pub dmx_address: dmx::DmxAddress,
    pub gdtf_file_name: String,
    pub dmx_mode: String,
}
