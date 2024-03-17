use self::artnet::Artnet;

pub mod artnet;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct DmxProtocols {
    pub artnet: Vec<Artnet>,
}
