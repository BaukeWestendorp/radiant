use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::attr::{Attribute, AttributeValue};
use crate::builtin::FixtureId;
use crate::comp::Component;
use crate::engine::Engine;
use crate::error::Result;

pub(crate) fn register(engine: &mut Engine) -> Result<()> {
    engine.register_component::<Programmer>()?;
    Ok(())
}

#[derive(Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Programmer {
    #[serde(default)]
    selection: Vec<FixtureId>,
    #[serde(default)]
    values: HashMap<(FixtureId, Attribute), AttributeValue>,
}

impl Programmer {
    pub fn selection(&self) -> &[FixtureId] {
        &self.selection
    }

    pub fn has_selection(&self) -> bool {
        !self.selection.is_empty()
    }

    pub(crate) fn select(&mut self, fid: FixtureId) {
        if !self.selection.contains(&fid) {
            self.selection.push(fid);
        }
    }

    pub(crate) fn clear_selection(&mut self) {
        self.selection.clear();
    }

    pub fn values(&self) -> &HashMap<(FixtureId, Attribute), AttributeValue> {
        &self.values
    }

    pub fn has_values(&self) -> bool {
        !self.values.is_empty()
    }

    pub(crate) fn set_value(
        &mut self,
        fid: FixtureId,
        attribute: Attribute,
        value: AttributeValue,
    ) {
        self.values.insert((fid, attribute), value);
    }

    pub(crate) fn clear_values(&mut self) {
        self.values.clear();
    }
}

impl Component for Programmer {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn relative_file_path() -> &'static str {
        "programmer.yaml"
    }

    fn save_to_showfile(&self, showfile_path: &Path) -> Result<()> {
        let file_path = showfile_path.join(Self::relative_file_path());
        let mut file = File::create(&file_path)?;
        let yaml = serde_yaml::to_string(self)?;
        file.write_all(yaml.as_bytes())?;
        Ok(())
    }
}
