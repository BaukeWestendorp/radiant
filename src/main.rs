use assets::Assets;
use dmx::color::DmxColor;
use gpui::{
    actions, impl_actions, App, AppContext, AssetSource, Bounds, KeyBinding, Point, Size,
    VisualContext, WindowBounds, WindowOptions,
};
use serde::Deserialize;
use show::{ColorPreset, Show};
use ui::window::{ColorPresetWindow, Window, WindowKind};

pub mod dmx;
pub mod show;
pub mod ui;

actions!(show, [Quit, Save]);

actions!(cmd, [Select, Store, Preset, Into, Clear]);

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct PresetId(pub usize);

impl_actions!(cmd, [PresetId]);

fn main() {
    App::new().run(|cx: &mut AppContext| {
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

        let show =
            match load_show() {
                Ok(show) => show,
                Err(_) => {
                    let mut show = Show::new();

                    show.screen_mut().layout_mut().add_window(Window::new(
                        WindowKind::ColorPreset(ColorPresetWindow::new()),
                    ));

                    show.presets_mut()
                        .add_color_preset(ColorPreset::new("Magenta", DmxColor::new(255, 0, 255)));

                    show
                }
            };

        cx.set_global(show);

        cx.bind_keys([
            KeyBinding::new("cmd-s", Save, None),
            KeyBinding::new("cmd-q", Quit, None),
        ]);

        cx.on_action::<Save>(|_action, cx| {
            let show = cx.global::<Show>();
            let serialized_show = serde_json::to_string(&show).unwrap();
            std::fs::write("show.json", serialized_show).unwrap();
        });

        cx.on_action::<Quit>(|_action, cx| {
            let serialized_show = serde_json::to_string(cx.global::<Show>()).unwrap();
            if serialized_show == std::fs::read_to_string("show.json").unwrap() {
                cx.quit();
            } else {
                eprintln!("Show has unsaved changes");
            }
        });

        cx.bind_keys([
            KeyBinding::new("ctrl-s", Select, None),
            KeyBinding::new("s", Store, None),
            KeyBinding::new("p", Preset, None),
            KeyBinding::new("i", Into, None),
            KeyBinding::new("escape", Clear, None),
        ]);

        cx.on_action::<Select>(|_action, _cx| println!("Select"));
        cx.on_action::<Store>(|_action, cx| {
            println!("Store");
            let show = cx.global_mut::<Show>();
            show.set_programmer_state(show::ProgrammerState::Store);
        });
        cx.on_action::<Preset>(|_action, _cx| println!("Preset"));
        cx.on_action::<PresetId>(|_action, _cx| println!("PresetId"));
        cx.on_action::<Into>(|_action, _cx| println!("Into"));
        cx.on_action::<Clear>(|_action, cx| {
            println!("Clear");
            let show = cx.global_mut::<Show>();
            show.set_programmer_state(show::ProgrammerState::Normal);
        });

        cx.open_window(
            WindowOptions {
                bounds: WindowBounds::Fixed(Bounds {
                    origin: Point {
                        x: 500.0.into(),
                        y: 350.0.into(),
                    },
                    size: Size {
                        width: 1280.0.into(),
                        height: 720.0.into(),
                    },
                }),
                ..Default::default()
            },
            |cx| {
                cx.new_view(|cx| {
                    let show = cx.global::<Show>();
                    let screen = show.screen();
                    screen.clone()
                })
            },
        );
    })
}

fn load_show() -> Result<Show, ()> {
    let serialized_show = std::fs::read_to_string("show.json").map_err(|_| ())?;
    let show = serde_json::from_str(&serialized_show).map_err(|_| ())?;
    Ok(show)
}
