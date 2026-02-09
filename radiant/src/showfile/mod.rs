use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context, Result};
use zeevonk::project::file::ProjectFile;

use crate::object::{Group, GroupId};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Showfile {
    zv_project_file: ProjectFile,

    groups: HashMap<GroupId, Group>,
}

impl Showfile {
    pub fn load_from_folder(showfile_path: &PathBuf) -> Result<Self> {
        const ZEEVONK_FOLDER_RELATIVE_PATH: &str = "zv/";
        const GROUPS_RELATIVE_PATH: &str = "groups.json";

        let zv_project_file =
            ProjectFile::load_from_folder(&showfile_path.join(ZEEVONK_FOLDER_RELATIVE_PATH))
                .context("failed to load zeevonk project file")?;

        let groups = serde_json::from_reader(
            std::fs::File::open(showfile_path.join(GROUPS_RELATIVE_PATH))
                .context("failed to open groups file")?,
        )
        .context("failed to deserialize groups file")?;

        Ok(Self { zv_project_file, groups })
    }

    pub fn zv_project_file(&self) -> &ProjectFile {
        &self.zv_project_file
    }

    pub fn groups(&self) -> &HashMap<GroupId, Group> {
        &self.groups
    }
}
