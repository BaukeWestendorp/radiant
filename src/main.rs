use assets::Assets;
use gpui::{
    actions, App, AppContext, AssetSource, Bounds, KeyBinding, Point, Size, WindowBounds,
    WindowOptions,
};
use show::{cmd, Show, ShowView};

pub mod color;
pub mod dmx;
pub mod layout;
pub mod presets;
pub mod screen;
pub mod show;
pub mod ui;
pub mod window;

actions!(app, [Quit]);

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

        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("s", cmd::Store, Some("Show")),
            KeyBinding::new("escape", cmd::Clear, Some("Show")),
            KeyBinding::new("t", cmd::Test, Some("Show")),
        ]);

        cx.on_action(|_action: &Quit, cx: &mut AppContext| cx.quit());

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
                cx.set_global(Show::new());

                cx.update_global::<Show, _>(|show, _cx| {
                    show.presets.add_color_preset(presets::ColorPreset::new(
                        "White",
                        dmx::color::DmxColor::new(255, 255, 255),
                    ));

                    show.presets.add_color_preset(presets::ColorPreset::new(
                        "Red",
                        dmx::color::DmxColor::new(255, 0, 0),
                    ));

                    show.presets.add_color_preset(presets::ColorPreset::new(
                        "Orange",
                        dmx::color::DmxColor::new(255, 128, 0),
                    ));

                    show.presets.add_color_preset(presets::ColorPreset::new(
                        "Yellow",
                        dmx::color::DmxColor::new(255, 255, 0),
                    ));

                    show.presets.add_color_preset(presets::ColorPreset::new(
                        "Lime",
                        dmx::color::DmxColor::new(128, 255, 0),
                    ));

                    show.presets.add_color_preset(presets::ColorPreset::new(
                        "Green",
                        dmx::color::DmxColor::new(0, 255, 0),
                    ));

                    show.presets.add_color_preset(presets::ColorPreset::new(
                        "Sea Green",
                        dmx::color::DmxColor::new(0, 255, 128),
                    ));

                    show.presets.add_color_preset(presets::ColorPreset::new(
                        "Cyan",
                        dmx::color::DmxColor::new(0, 255, 255),
                    ));

                    show.presets.add_color_preset(presets::ColorPreset::new(
                        "Sky Blue",
                        dmx::color::DmxColor::new(0, 128, 255),
                    ));

                    show.presets.add_color_preset(presets::ColorPreset::new(
                        "Blue",
                        dmx::color::DmxColor::new(0, 0, 255),
                    ));

                    show.presets.add_color_preset(presets::ColorPreset::new(
                        "Purple",
                        dmx::color::DmxColor::new(128, 0, 255),
                    ));

                    show.presets.add_color_preset(presets::ColorPreset::new(
                        "Magenta",
                        dmx::color::DmxColor::new(255, 0, 255),
                    ));

                    show.presets.add_color_preset(presets::ColorPreset::new(
                        "Pink",
                        dmx::color::DmxColor::new(255, 0, 128),
                    ));
                });
                ShowView::build(cx)
            },
        );
    })
}
