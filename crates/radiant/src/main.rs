use anyhow::Result;
use assets::Assets;

use backstage::command::{Command, Instruction};
use gpui::AssetSource;
use gpui::{actions, App, AppContext, KeyBinding, Menu, MenuItem};

use crate::workspace::Workspace;
use theme::ThemeSettings;
use ui::text_input;

pub mod workspace;

actions!(app, [Quit]);

fn main() {
    env_logger::init();
    dotenv::dotenv().ok();

    log::info!("Starting Radiant...");

    App::new().run(|cx: &mut AppContext| {
        cx.set_global(ThemeSettings::default());

        if let Err(err) = init_fonts(cx) {
            log::error!("{}", err);
        }

        if let Err(err) = init_keybinds(cx) {
            log::error!("{}", err);
        }

        init_menu(cx);

        cx.on_action(quit);

        cx.spawn(move |mut cx| async move {
            let mut workspace = Workspace::new(&mut cx).await.unwrap();
            workspace.start_dmx_output_loop(&cx).await;
        })
        .detach();

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

    cx.bind_keys([
        KeyBinding::new("cmd-q", Quit, None),
        KeyBinding::new(
            "escape",
            workspace::action::Command(Command::new([Instruction::Clear])),
            Some("Screen"),
        ),
        KeyBinding::new("enter", text_input::Enter, Some("TextInput")),
        KeyBinding::new("backspace", text_input::Backspace, Some("TextInput")),
        KeyBinding::new("delete", text_input::Delete, Some("TextInput")),
    ]);

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
