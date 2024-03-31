use anyhow::Result;
use assets::Assets;

use backstage::command::Command;
use gpui::{actions, App, AppContext, KeyBinding, Menu, MenuItem};
use gpui::{AssetSource, Global};

use crate::workspace::action::{ExecuteCommand, SetCurrentCommand};
use crate::workspace::Workspace;
use theme::ThemeSettings;
use ui::text_input;

pub mod layout;
pub mod workspace;

actions!(app, [Quit]);

fn main() {
    env_logger::init();
    dotenv::dotenv().ok();

    log::info!("Starting Radiant...");

    App::new().run(|cx: &mut AppContext| {
        cx.set_global(ThemeSettings::default());
        cx.set_global(AppState::default());

        if let Err(err) = init_fonts(cx) {
            log::error!("{}", err);
        }

        if let Err(err) = init_keybinds(cx) {
            log::error!("{}", err);
        }

        init_menu(cx);

        cx.on_action(quit);

        cx.spawn(move |cx| async move {
            match Workspace::new(&cx).await {
                Err(err) => {
                    log::error!("Failed to open workspace: {err}");
                    cx.update(|cx| cx.quit()).unwrap();
                }
                Ok(mut workspace) => {
                    workspace.start_dmx_output_loop(&cx).await;
                }
            }
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

    set_default_keybinds(cx);
    if cx.global::<AppState>().use_command_shortcuts {
        set_command_shortcuts(cx);
    }

    cx.observe_global::<AppState>(|cx| {
        if cx.global::<AppState>().use_command_shortcuts {
            set_command_shortcuts(cx);
        } else {
            cx.clear_key_bindings();
            set_default_keybinds(cx);
        }
    })
    .detach();

    log::info!("Initialized keybindings");

    Ok(())
}

fn set_default_keybinds(cx: &mut AppContext) {
    cx.bind_keys([
        KeyBinding::new("cmd-q", Quit, None),
        KeyBinding::new("escape", ExecuteCommand(Command::Clear), None),
        // FIXME: This should be moved to the ui crate.
        KeyBinding::new("enter", text_input::Enter, Some("TextInput")),
        KeyBinding::new("backspace", text_input::Backspace, Some("TextInput")),
        KeyBinding::new("delete", text_input::Delete, Some("TextInput")),
    ]);

    log::info!("Set default keybindings");
}

fn set_command_shortcuts(cx: &mut AppContext) {
    cx.bind_keys([
        KeyBinding::new("s", SetCurrentCommand(Some(Command::Store(None))), None),
        KeyBinding::new("S", SetCurrentCommand(Some(Command::Select(None))), None),
        KeyBinding::new("backspace", SetCurrentCommand(None), None),
    ]);

    log::info!("Set command shortcuts");
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

pub struct AppState {
    pub use_command_shortcuts: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            use_command_shortcuts: true,
        }
    }
}

impl Global for AppState {}
