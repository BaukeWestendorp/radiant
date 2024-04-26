use std::path::PathBuf;

use gpui::{AppContext, VisualContext, WindowOptions};

use crate::{
    output::{artnet::ArtnetDmxProtocol, OutputManager},
    showfile::Showfile,
    workspace::Workspace,
};

pub fn run_app(app: gpui::App, showfile_path: Option<PathBuf>) {
    app.run(move |cx: &mut AppContext| {
        Showfile::init(showfile_path, cx)
            .map_err(|err| log::error!("Failed to initialize showfile: {err}"))
            .ok();

        OutputManager::init(cx);
        OutputManager::register_protocol(ArtnetDmxProtocol::new("0.0.0.0", 0, 0).unwrap(), cx);

        cx.open_window(WindowOptions::default(), |cx| {
            let view = Workspace::build(cx);
            cx.focus_view(&view);
            view
        });
    });
}
