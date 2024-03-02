use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ColorPresetWindow {}

impl ColorPresetWindow {
    pub fn new() -> Self {
        Self {}
    }
}
