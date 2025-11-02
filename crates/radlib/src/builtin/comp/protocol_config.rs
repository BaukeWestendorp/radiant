use std::fs::File;
use std::io::Write;
use std::net::IpAddr;
use std::path::Path;

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

    fn save_to_showfile(&self, showfile_path: &Path) -> Result<()> {
        let file_path = showfile_path.join(Self::relative_file_path());
        let mut file = File::create(&file_path)?;
        let yaml = serde_yaml::to_string(self)?;
        file.write_all(yaml.as_bytes())?;
        Ok(())
    }
}
