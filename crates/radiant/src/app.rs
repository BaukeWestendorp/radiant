use std::path::PathBuf;

use gpui::{AppContext, WindowOptions};

use crate::{output, showfile::Showfile, workspace::Workspace};

pub fn run_app(app: gpui::App, showfile_path: Option<PathBuf>) {
    app.run(move |cx: &mut AppContext| {
        Showfile::init(showfile_path, cx)
            .map_err(|err| log::error!("Failed to initialize showfile: {err}"))
            .ok();

        output::start_dmx_output_loop(cx);

        cx.open_window(WindowOptions::default(), |cx| {
            let view = Workspace::build(cx);
            view
        });
    });
}
