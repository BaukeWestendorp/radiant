use anyhow::Result;
use gpui::{App, AppContext, Entity, Global};
use zeevonk::{
    Zeevonk,
    project::{file::ProjectFile, stage::FixtureId},
};

pub(crate) fn init(zv_project_file: ProjectFile, cx: &mut App) -> Result<()> {
    let app_state = AppState::new(zv_project_file, cx)?;
    cx.set_global(app_state);
    Ok(())
}

pub struct AppState {
    zeevonk: Zeevonk,

    selection: Entity<Vec<FixtureId>>,
}

impl AppState {
    pub fn new(zv_project_file: ProjectFile, cx: &mut App) -> Result<Self> {
        let zeevonk = Zeevonk::new(zv_project_file)?;
        zeevonk.start();

        Ok(Self { zeevonk, selection: cx.new(|_| Vec::new()) })
    }

    pub fn zeevonk(&self) -> &Zeevonk {
        &self.zeevonk
    }

    pub fn selection(&self) -> &Entity<Vec<FixtureId>> {
        &self.selection
    }
}

impl Global for AppState {}
