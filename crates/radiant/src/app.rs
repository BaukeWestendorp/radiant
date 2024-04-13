use backstage::{Command, Object};
use gpui::{
    actions, point, size, AppContext, Bounds, Global, KeyBinding, Menu, MenuItem, VisualContext,
    WindowOptions,
};
use theme::ThemeSettings;
use ui::text_input;

use crate::assets::Assets;
use crate::output::{artnet, DmxOutputManager};
use crate::showfile::ShowfileManager;
use crate::workspace::actions::{
    ExecuteCommand, ExecuteCurrentCommand, SetCurrentCommand, SetCurrentObject,
};
use crate::workspace::Workspace;

actions!(app, [Quit, Save]);

pub fn run_app(app: gpui::App, showfile_path: Option<String>) {
    app.with_assets(Assets).run(move |cx: &mut AppContext| {
        AppState::init(cx);
        ThemeSettings::init(cx);
        ShowfileManager::init(showfile_path, cx);
        DmxOutputManager::init(cx);

        register_io_protocols_from_showfile(cx);

        init_menu(cx);

        if let Err(err) = init_keybinds(cx) {
            log::error!("Failed to initialize keybinds: {err}");
        }

        cx.on_action(handle_quit);

        cx.on_action(handle_save);

        let window_size = size(1719.into(), 998.into());
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

fn register_io_protocols_from_showfile(cx: &mut AppContext) {
    let io = ShowfileManager::io(cx).clone();
    for protocol in io.artnet.into_iter() {
        match artnet::ArtnetDmxProtocol::new(protocol.target_ip.as_str()) {
            Ok(protocol) => DmxOutputManager::register_protocol(protocol, cx),
            Err(err) => log::error!("Failed to initialize protocol: {err}"),
        }
    }
}

fn init_keybinds(cx: &mut AppContext) -> anyhow::Result<()> {
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
        KeyBinding::new("cmd-s", Save, None),
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
        KeyBinding::new(
            "l",
            SetCurrentCommand(Some(Command::Label {
                object: None,
                label: None,
            })),
            None,
        ),
        KeyBinding::new("f", SetCurrentObject(Some(Object::Fixture(None))), None),
        KeyBinding::new("g", SetCurrentObject(Some(Object::Group(None))), None),
        KeyBinding::new("e", SetCurrentObject(Some(Object::Executor(None))), None),
        KeyBinding::new(
            "c",
            SetCurrentObject(Some(Object::Cue {
                sequence_id: None,
                cue_ix: None,
            })),
            None,
        ),
        KeyBinding::new("enter", ExecuteCurrentCommand, None),
        KeyBinding::new("backspace", SetCurrentCommand(None), None),
    ]);

    log::info!("Set command shortcuts");
}

fn init_menu(cx: &mut AppContext) {
    cx.set_menus(vec![
        Menu {
            name: "Radiant",
            items: vec![MenuItem::action("Quit", Quit)],
        },
        Menu {
            name: "Programmer",
            items: vec![MenuItem::action("Clear", ExecuteCommand(Command::Clear))],
        },
    ]);

    log::info!("Initialized menu");
}

fn handle_quit(_action: &Quit, cx: &mut AppContext) {
    cx.quit();
}

fn handle_save(_action: &Save, cx: &mut AppContext) {
    if let Err(err) = ShowfileManager::save(cx) {
        log::error!("Failed to save showfile: {err}")
    }
}

pub struct AppState {
    pub use_command_shortcuts: bool,
}

impl AppState {
    pub fn init(cx: &mut AppContext) {
        cx.set_global(Self::default());
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            use_command_shortcuts: true,
        }
    }
}

impl Global for AppState {}
