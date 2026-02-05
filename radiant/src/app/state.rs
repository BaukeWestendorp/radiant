use anyhow::Result;
use gpui::{App, Global};
use zeevonk::{Zeevonk, project::file::ProjectFile};

pub(crate) fn init(zv_project_file: ProjectFile, cx: &mut App) -> Result<()> {
    cx.set_global(AppState::new(zv_project_file)?);
    Ok(())
}

pub struct AppState {
    zeevonk: Zeevonk,
}

impl AppState {
    pub fn new(zv_project_file: ProjectFile) -> Result<Self> {
        let zeevonk = Zeevonk::new(zv_project_file)?;
        zeevonk.start();

        Ok(Self { zeevonk })
    }

    pub fn zeevonk(&self) -> &Zeevonk {
        &self.zeevonk
    }
}

impl Global for AppState {}
