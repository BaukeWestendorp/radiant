use backstage::Command;
use gpui::{
    actions, impl_actions, point, size, AppContext, Bounds, KeyBinding, Menu, MenuItem,
    VisualContext, WindowOptions,
};
use theme::ThemeSettings;

use crate::assets::Assets;
use crate::output::DmxOutputManager;
use crate::showfile::ShowfileManager;
use crate::workspace::Workspace;

actions!(app, [Quit]);
impl_actions!(app, [ExecuteCommand]);

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct ExecuteCommand(pub Command);

pub fn run_app(app: gpui::App, showfile_path: Option<String>) {
    app.with_assets(Assets).run(move |cx: &mut AppContext| {
        let window_size = size(1719.into(), 960.into());
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

        init_keybinds(cx);
        init_menu(cx);

        cx.on_action(|_action: &Quit, cx| {
            log::info!("Quitting Radiant...");
            cx.quit();
            log::info!("Quit Radiant");
        });

        cx.open_window(window_options, |cx| {
            ThemeSettings::init(cx);
            DmxOutputManager::init(cx);
            ShowfileManager::init(showfile_path, cx);

            let view = Workspace::build(cx);
            cx.focus_view(&view);
            view
        });
    });
}

fn init_keybinds(cx: &mut AppContext) {
    cx.bind_keys([
        KeyBinding::new("cmd-q", Quit, None),
        KeyBinding::new("escape", ExecuteCommand(Command::Clear), None),
    ]);
}

fn init_menu(cx: &mut AppContext) {
    cx.set_menus(vec![Menu {
        name: "Radiant",
        items: vec![MenuItem::action("Quit", Quit)],
    }]);

    log::info!("Initialized menu");
}
