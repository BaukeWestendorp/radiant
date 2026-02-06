use std::collections::HashMap;

use anyhow::Result;
use gpui::{App, Global};
use zeevonk::{
    Zeevonk,
    project::{file::ProjectFile, stage::FixtureId},
};

use crate::app::object::{Group, GroupId, Preset, PresetId};

pub(crate) fn init(zv_project_file: ProjectFile, cx: &mut App) -> Result<()> {
    cx.set_global(AppState::new(zv_project_file)?);
    Ok(())
}

pub struct AppState {
    zeevonk: Zeevonk,

    pub selection: Vec<FixtureId>,
    pub groups: HashMap<GroupId, Group>,
    pub presets: HashMap<PresetId, Preset>,
}

impl AppState {
    pub fn new(zv_project_file: ProjectFile) -> Result<Self> {
        let zeevonk = Zeevonk::new(zv_project_file)?;
        zeevonk.start();

        Ok(Self { zeevonk, selection: Vec::new(), groups: HashMap::new(), presets: HashMap::new() })
    }

    pub fn zeevonk(&self) -> &Zeevonk {
        &self.zeevonk
    }
}

impl Global for AppState {}
