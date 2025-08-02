use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use eyre::Context;

use crate::engine::protocols::Protocols;
use crate::pipeline::Pipeline;
use crate::show::{
    Group, PresetBeam, PresetColor, PresetContent, PresetControl, PresetDimmer, PresetFocus,
    PresetGobo, PresetPosition, PresetShapers, PresetVideo, Show,
};
use crate::showfile::{SacnOutputType, Showfile};

mod command;
mod control_surface;
mod processor;
mod protocols;

pub use command::*;

pub struct Engine {
    protocols: Protocols,

    show: Arc<Show>,

    pipeline: Arc<Mutex<Pipeline>>,
}

impl Engine {
    pub fn new(showfile_path: Option<&PathBuf>) -> Result<Self, crate::error::Error> {
        let showfile = match showfile_path {
            Some(path) => Showfile::load(path).wrap_err("failed to load showfile")?,
            None => Showfile::default(),
        };

        let pipeline = Arc::new(Mutex::new(Pipeline::new()));

        let protocols = Protocols::new(pipeline.clone());
        for configuration in &showfile.protocols.sacn_source_configurations {
            let ip = match configuration.r#type {
                SacnOutputType::Unicast { destination_ip } => destination_ip,
            };

            protocols.add_sacn_source(
                configuration.name.clone(),
                ip,
                configuration.priority,
                configuration.preview_data,
            )?;
        }

        let show = Show::new(showfile).wrap_err("failed to create show from showfile")?;

        Ok(Self { protocols, show: Arc::new(show), pipeline })
    }

    pub fn start(&self) {
        processor::start(self.pipeline.clone(), self.show.clone());
        self.protocols.start();
        control_surface::start(self.show.clone());
    }

    pub fn exec(&self, command: Command) -> Result<(), crate::error::Error> {
        match command {
            Command::PatchAdd { fid, address, type_id, dmx_mode } => {
                self.show.patch().insert_fixture(fid, address, type_id, dmx_mode);
            }
            Command::CreateGroup { id, name, fids } => {
                self.show.insert_object(Group {
                    id,
                    name: name.unwrap_or("New Group".to_string()),
                    fids,
                });
            }
            Command::CreatePresetDimmer { id, name } => {
                self.show.insert_object(PresetDimmer {
                    id,
                    name: name.unwrap_or("New Dimmer Preset".to_string()),
                    content: PresetContent::default(),
                });
            }
            Command::CreatePresetPosition { id, name } => {
                self.show.insert_object(PresetPosition {
                    id,
                    name: name.unwrap_or("New Position Preset".to_string()),
                    content: PresetContent::default(),
                });
            }
            Command::CreatePresetGobo { id, name } => {
                self.show.insert_object(PresetGobo {
                    id,
                    name: name.unwrap_or("New Gobo Preset".to_string()),
                    content: PresetContent::default(),
                });
            }
            Command::CreatePresetColor { id, name } => {
                self.show.insert_object(PresetColor {
                    id,
                    name: name.unwrap_or("New Color Preset".to_string()),
                    content: PresetContent::default(),
                });
            }
            Command::CreatePresetBeam { id, name } => {
                self.show.insert_object(PresetBeam {
                    id,
                    name: name.unwrap_or("New Beam Preset".to_string()),
                    content: PresetContent::default(),
                });
            }
            Command::CreatePresetFocus { id, name } => {
                self.show.insert_object(PresetFocus {
                    id,
                    name: name.unwrap_or("New Focus Preset".to_string()),
                    content: PresetContent::default(),
                });
            }
            Command::CreatePresetControl { id, name } => {
                self.show.insert_object(PresetControl {
                    id,
                    name: name.unwrap_or("New Control Preset".to_string()),
                    content: PresetContent::default(),
                });
            }
            Command::CreatePresetShapers { id, name } => {
                self.show.insert_object(PresetShapers {
                    id,
                    name: name.unwrap_or("New Shapers Preset".to_string()),
                    content: PresetContent::default(),
                });
            }
            Command::CreatePresetVideo { id, name } => {
                self.show.insert_object(PresetVideo {
                    id,
                    name: name.unwrap_or("New Video Preset".to_string()),
                    content: PresetContent::default(),
                });
            }
            Command::ProgrammerSetAttribute { fid, attribute, value } => {
                self.show().programmer().set_value(fid, attribute, value);
            }
            Command::Go { executor } => {
                if let Some(executor) = self.show().executor(executor) {
                    self.show.
                }
            }
        }

        Ok(())
    }

    pub fn show(&self) -> &Show {
        &self.show
    }
}
