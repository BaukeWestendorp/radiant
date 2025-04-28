#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, Eq, Default)]
pub struct Patch {
    pub fixtures: Vec<Fixture>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, Eq)]
pub struct Fixture {
    pub id: u32,
    pub address: dmx::Address,
    pub gdtf_file_name: String,
    pub dmx_mode: String,
}
