use std::collections::HashMap;

use crate::attr::{Attribute, AttributeValue};
use crate::builtin::FixtureId;
use crate::comp::ShowfileComponent;
use crate::engine::Engine;
use crate::error::Result;

pub(crate) fn register(engine: &mut Engine) -> Result<()> {
    engine.register_component::<Programmer>()?;
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Programmer {
    selected_fixtures: Vec<FixtureId>,
    values: HashMap<(FixtureId, Attribute), AttributeValue>,
}

impl Programmer {
    pub fn selected_fixtures(&self) -> &[FixtureId] {
        &self.selected_fixtures
    }

    pub fn select_fixture(&mut self, fid: FixtureId) {
        if !self.selected_fixtures.contains(&fid) {
            self.selected_fixtures.push(fid);
        }
    }

    pub fn clear_selected_fixtures(&mut self) {
        self.selected_fixtures.clear();
    }
}

impl ShowfileComponent for Programmer {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn relative_file_path() -> &'static str {
        "programmer.yaml"
    }
}
