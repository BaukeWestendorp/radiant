use std::path::PathBuf;

use gpui::*;
use ui::theme::Theme;

use crate::workspace::Workspace;

actions!(app, [Quit, Open]);

pub struct RadiantApp {
    workspace: Option<Workspace>,
}

impl RadiantApp {
    pub fn new() -> Self {
        Self { workspace: None }
    }

    pub fn run(
        &mut self,
        showfile_path: Option<PathBuf>,
        cx: &mut AppContext,
    ) -> anyhow::Result<()> {
        cx.set_global(Theme::default());

        ui::init(cx);
        flow::gpui::init(cx);

        self.workspace = Some(Workspace::new(showfile_path, cx)?);

        cx.activate(true);

        Ok(())
    }
}
