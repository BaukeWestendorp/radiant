use std::path::PathBuf;

use gpui::{actions, AppContext, KeyBinding, Menu, MenuItem, VisualContext, WindowOptions};

actions!(app, [Quit]);

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

        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

        cx.set_menus(vec![Menu {
            name: "radiant",
            items: vec![MenuItem::action("Quit", Quit)],
        }]);

        cx.on_action::<Quit>(|_action, cx| {
            log::info!("Quitting Radiant...");
            cx.quit();
            log::info!("Exited Radiant. Goodbye!")
        });

        cx.open_window(WindowOptions::default(), |cx| {
            let view = Workspace::build(cx);
            cx.focus_view(&view);
            view
        });
    });
}
