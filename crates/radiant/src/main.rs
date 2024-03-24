use anyhow::Result;
use assets::Assets;
use gpui::AssetSource;
use gpui::{actions, App, AppContext, KeyBinding, Menu, MenuItem};

use crate::workspace::Workspace;

pub mod ui;
pub mod workspace;

actions!(app, [Quit]);

fn main() {
    env_logger::init();
    log::info!("Starting Radiant...");

    App::new().run(|cx: &mut AppContext| {
        if let Err(err) = init_fonts(cx) {
            log::error!("{}", err);
        }

        if let Err(err) = init_keybinds(cx) {
            log::error!("{}", err);
        }

        init_menu(cx);

        cx.on_action(quit);

        Workspace::open_window(cx);

        log::info!("Radiant initialized");
    });
}

fn init_fonts(cx: &mut AppContext) -> Result<()> {
    cx.text_system().add_fonts(vec![
        Assets.load("fonts/zed-sans/zed-sans-extended.ttf")?,
        Assets.load("fonts/zed-sans/zed-sans-extendedbold.ttf")?,
        Assets.load("fonts/zed-sans/zed-sans-extendeditalic.ttf")?,
    ])
}

fn init_keybinds(cx: &mut AppContext) -> Result<()> {
    // FIXME: Get the keybindings from a keybinds file.

    cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

    log::info!("Initialized keybindings");

    Ok(())
}

fn init_menu(cx: &mut AppContext) {
    cx.set_menus(vec![Menu {
        name: "Radiant",
        items: vec![MenuItem::action("Quit", Quit)],
    }]);

    log::info!("Initialized menu");
}

fn quit(_: &Quit, cx: &mut AppContext) {
    log::info!("Quitting Radiant...");
    cx.quit();
    log::info!("Quit Radiant");
}
