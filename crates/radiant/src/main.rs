use assets::Assets;
use gpui::{
    actions, point, size, App, AppContext, AssetSource, Bounds, Context, KeyBinding, Menu,
    MenuItem, VisualContext, WindowOptions,
};

use workspace::Workspace;

use crate::{
    cmd::{Command, CommandList},
    show::Show,
};

pub mod cmd;
pub mod color;
pub mod dmx;
pub mod dmx_protocols;
pub mod show;
pub mod ui;
pub mod workspace;

actions!(app, [Quit]);

fn main() {
    env_logger::init();
    log::info!("Starting Radiant");

    App::new().run(|cx: &mut AppContext| {
        let show = cx.new_model(|_cx| Show::default());
        cx.set_global(CommandList::default());

        cx.text_system()
            .add_fonts(vec![
                Assets.load("fonts/zed-sans/zed-sans-extended.ttf").unwrap(),
                Assets
                    .load("fonts/zed-sans/zed-sans-extendedbold.ttf")
                    .unwrap(),
                Assets
                    .load("fonts/zed-sans/zed-sans-extendeditalic.ttf")
                    .unwrap(),
            ])
            .unwrap();

        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-o", workspace::actions::OpenShow, Some("Workspace")),
            KeyBinding::new("cmd-s", workspace::actions::SaveShow, Some("Workspace")),
            KeyBinding::new("enter", cmd::ExecuteCommandList, Some("Workspace")),
            KeyBinding::new("backspace", cmd::RemoveCommand, Some("Workspace")),
            KeyBinding::new("g", Command::Group, Some("Workspace")),
        ]);

        cx.on_action(|_action: &Quit, cx: &mut AppContext| cx.quit());

        cx.set_menus(vec![
            Menu {
                name: "Radiant",
                items: vec![MenuItem::action("Quit", Quit)],
            },
            Menu {
                name: "Show",
                items: vec![
                    MenuItem::action("Open", workspace::actions::OpenShow),
                    MenuItem::action("Save", workspace::actions::SaveShow),
                ],
            },
            Menu {
                name: "Commands",
                items: vec![MenuItem::action("Group", Command::Group)],
            },
        ]);

        cx.open_window(
            WindowOptions {
                bounds: Some(Bounds {
                    origin: point(0.0.into(), 0.0.into()),
                    size: size(1200.0.into(), 1000.0.into()),
                }),
                ..Default::default()
            },
            |cx| {
                let workspace = cx.new_view(|cx| Workspace::new(show.clone(), cx));

                workspace.update(cx, |workspace, cx| {
                    workspace.open_show(&workspace::actions::OpenShow, cx);
                });

                cx.focus_view(&workspace);

                workspace
            },
        );
    })
}
