use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context, Result};
use zeevonk::project::file::ProjectFile;

use crate::{
    layout::Layout,
    object::{Effect, EffectId, Group, GroupId},
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Showfile {
    zv_project_file: ProjectFile,

    groups: HashMap<GroupId, Group>,
    effects: HashMap<EffectId, Effect>,

    layout: Layout,
}

impl Showfile {
    pub fn load_from_folder(showfile_path: &PathBuf) -> Result<Self> {
        const ZEEVONK_FOLDER_RELATIVE_PATH: &str = "zv/";
        const GROUPS_RELATIVE_PATH: &str = "objects/groups.json";
        const EFFECTS_RELATIVE_PATH: &str = "objects/effects.json";
        const LAYOUT_RELATIVE_PATH: &str = "layout.json";

        let zv_project_file =
            ProjectFile::load_from_folder(&showfile_path.join(ZEEVONK_FOLDER_RELATIVE_PATH))
                .context("failed to load zeevonk project file")?;

        let groups = serde_json::from_reader(
            std::fs::File::open(showfile_path.join(GROUPS_RELATIVE_PATH))
                .context("failed to open groups file")?,
        )
        .context("failed to deserialize groups file")?;

        let effects = serde_json::from_reader(
            std::fs::File::open(showfile_path.join(EFFECTS_RELATIVE_PATH))
                .context("failed to open effects file")?,
        )
        .context("failed to deserialize effects file")?;

        let layout = serde_json::from_reader(
            std::fs::File::open(showfile_path.join(LAYOUT_RELATIVE_PATH))
                .context("failed to open layout file")?,
        )
        .context("failed to deserialize layout file")?;

        Ok(Self { zv_project_file, groups, effects, layout })
    }

    pub fn zv_project_file(&self) -> &ProjectFile {
        &self.zv_project_file
    }

    pub fn groups(&self) -> &HashMap<GroupId, Group> {
        &self.groups
    }

    pub fn effects(&self) -> &HashMap<EffectId, Effect> {
        &self.effects
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }
}
