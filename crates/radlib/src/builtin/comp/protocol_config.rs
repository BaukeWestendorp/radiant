use std::net::IpAddr;

use crate::comp::Component;
use crate::engine::Engine;
use crate::error::Result;

pub(crate) fn register(engine: &mut Engine) -> Result<()> {
    engine.register_component::<ProtocolConfig>()?;
    Ok(())
}

#[derive(Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProtocolConfig {
    #[serde(default)]
    pub(crate) sacn_sources: Vec<SacnSourceConfiguration>,
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SacnSourceConfiguration {
    pub(crate) name: String,
    pub(crate) priority: u8,
    pub(crate) preview_data: bool,
    pub(crate) r#type: SacnOutputType,
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SacnOutputType {
    Unicast { destination_ip: IpAddr },
}

impl Component for ProtocolConfig {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn relative_file_path() -> &'static str {
        "protocols.yaml"
    }
}
