use std::{collections::HashMap, fs::File, path::Path, path::PathBuf};

use anyhow::{Context, Result};
use zeevonk::project::file::ProjectFile;

use crate::{
    layout::Layout,
    object::{Effect, EffectId, Group, GroupId},
};

const ZEEVONK_FOLDER_RELATIVE_PATH: &str = "zv/";
const GROUPS_RELATIVE_PATH: &str = "objects/groups.json";
const EFFECT_SCRIPTS_RELATIVE_PATH: &str = "objects/effects/";
const EFFECT_DEFINITIONS_RELATIVE_PATH: &str = "objects/effects.json";
const LAYOUT_RELATIVE_PATH: &str = "layout.json";

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Showfile {
    zv_project_file: ProjectFile,

    groups: HashMap<GroupId, Group>,
    effect_definitions: HashMap<EffectId, EffectDefinition>,
    #[serde(skip)]
    effects: HashMap<EffectId, Effect>,

    layout: Layout,
}

impl Showfile {
    pub fn load_from_folder(showfile_path: &PathBuf) -> Result<Self> {
        let zv_project_file =
            ProjectFile::load_from_folder(&showfile_path.join(ZEEVONK_FOLDER_RELATIVE_PATH))
                .context("failed to load zeevonk project file")?;

        let groups = load_groups(showfile_path)?;
        let (effect_definitions, effects) = load_effects_and_definitions(showfile_path)?;
        let layout = load_layout(showfile_path)?;

        Ok(Self { zv_project_file, groups, effect_definitions, effects, layout })
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

fn load_groups(showfile_path: &Path) -> Result<HashMap<GroupId, Group>> {
    let file = File::open(showfile_path.join(GROUPS_RELATIVE_PATH))
        .context("failed to open groups file")?;

    serde_json::from_reader(file).context("failed to deserialize groups file")
}

fn load_effects_and_definitions(
    showfile_path: &Path,
) -> Result<(HashMap<EffectId, EffectDefinition>, HashMap<EffectId, Effect>)> {
    let definitions_file = File::open(showfile_path.join(EFFECT_DEFINITIONS_RELATIVE_PATH))
        .context("failed to open effect definitions file")?;

    let effect_definitions: HashMap<EffectId, EffectDefinition> =
        serde_json::from_reader(definitions_file)
            .context("failed to deserialize effect definitions file")?;

    let effects: HashMap<EffectId, Effect> = effect_definitions
        .iter()
        .map(|(effect_id, definition)| {
            let script_path =
                showfile_path.join(EFFECT_SCRIPTS_RELATIVE_PATH).join(&definition.script_path);

            let script = std::fs::read_to_string(&script_path).with_context(|| {
                format!("failed to read effect script at {}", script_path.display())
            })?;

            let effect = Effect::new(definition.name.clone(), script);
            Ok((*effect_id, effect))
        })
        .collect::<Result<_>>()?;

    Ok((effect_definitions, effects))
}

fn load_layout(showfile_path: &Path) -> Result<Layout> {
    let file = File::open(showfile_path.join(LAYOUT_RELATIVE_PATH))
        .context("failed to open layout file")?;

    serde_json::from_reader(file).context("failed to deserialize layout file")
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub struct EffectDefinition {
    name: String,
    script_path: String,
}
