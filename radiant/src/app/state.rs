use anyhow::Result;
use gpui::{App, Global};
use zeevonk::Zeevonk;

use crate::{show::Show, showfile::Showfile};

pub(crate) fn init(showfile: Showfile, cx: &mut App) -> Result<()> {
    let app_state = AppState::new(showfile, cx)?;
    cx.set_global(app_state);
    Ok(())
}

pub struct AppState {
    zeevonk: Zeevonk,

    show: Show,
}

impl AppState {
    pub fn new(showfile: Showfile, cx: &mut App) -> Result<Self> {
        let zeevonk = Zeevonk::new(showfile.zv_project_file().clone())?;
        zeevonk.start();

        let show = Show::from_showfile(&showfile, cx);

        Ok(Self { zeevonk, show })
    }

    pub fn zeevonk(&self) -> &Zeevonk {
        &self.zeevonk
    }

    pub fn show(&self) -> &Show {
        &self.show
    }
}

impl Global for AppState {}
