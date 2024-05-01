use std::path::PathBuf;

use backstage::cmd::Command;
use gpui::{
    actions, point, size, AppContext, Bounds, KeyBinding, Menu, MenuItem, VisualContext,
    WindowOptions,
};

actions!(app, [Quit]);

use crate::{
    output::{artnet::ArtnetDmxProtocol, OutputManager},
    showfile::Showfile,
    workspace::{self, Workspace},
};

pub fn run_app(app: gpui::App, showfile_path: Option<PathBuf>) {
    app.run(move |cx: &mut AppContext| {
        Showfile::init(showfile_path, cx)
            .map_err(|err| log::error!("Failed to initialize showfile: {err}"))
            .ok();

        OutputManager::init(cx);
        OutputManager::register_protocol(ArtnetDmxProtocol::new("0.0.0.0", 0, 0).unwrap(), cx);
        OutputManager::register_protocol(ArtnetDmxProtocol::new("0.0.0.0", 1, 1).unwrap(), cx);

        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new(
                "escape",
                workspace::action::ExecuteCommand(Command::Clear),
                Some("Workspace"),
            ),
        ]);

        cx.set_menus(vec![Menu {
            name: "radiant",
            items: vec![MenuItem::action("Quit", Quit)],
        }]);

        cx.on_action::<Quit>(|_action, cx| {
            log::info!("Quitting Radiant...");
            cx.quit();
            log::info!("Exited Radiant. Goodbye!")
        });

        let window_size = size(1712.into(), 998.into());
        let window_options = WindowOptions {
            bounds: Some(Bounds {
                origin: cx
                    .primary_display()
                    .map(|display| {
                        display.bounds().center()
                            - point(window_size.width / 2, window_size.height / 2)
                    })
                    .unwrap_or(point(1920.into(), 1080.into())),
                size: window_size,
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |cx| {
            let view = Workspace::build(cx);
            cx.focus_view(&view);
            view
        });
    });
}
