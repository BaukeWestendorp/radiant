use std::path::PathBuf;

use anyhow::Context as _;

use crate::engine::{Object, ObjectId, ObjectKind, SlotId};

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Effect {
    id: ObjectId,
    slot_id: SlotId,
    name: String,

    file_name: String,
}

impl Effect {
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn load_lua_source(&self, showfile_path: Option<&PathBuf>) -> anyhow::Result<String> {
        let showfile_path = showfile_path.context("no showfile to find lua files in")?;
        let effect_path = showfile_path.join("obj/effects/").join(&self.file_name);
        let source = std::fs::read_to_string(&effect_path)?;
        Ok(source)
    }
}

impl Object for Effect {
    fn kind() -> ObjectKind {
        ObjectKind::Effect
    }

    fn id(&self) -> ObjectId {
        self.id
    }

    fn slot_id(&self) -> SlotId {
        self.slot_id
    }

    fn name(&self) -> &str {
        &self.name
    }
}
