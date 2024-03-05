use assets::Assets;
use dmx::color::DmxColor;
use gpui::{
    App, AppContext, AssetSource, Bounds, KeyBinding, Point, Size, WindowBounds, WindowOptions,
};
use presets::ColorPreset;
use show::{ShowModel, ShowView};
use ui::window::{ColorPresetWindow, Window, WindowKind};

pub mod dmx;
pub mod presets;
pub mod show;
pub mod ui;

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
            KeyBinding::new("s", show::cmd::Store, Some("Show")),
            KeyBinding::new("escape", show::cmd::Clear, Some("Show")),
        ]);

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
                ShowModel::init(cx);

                // ShowModel::update(cx, |model, cx| {
                //     model.inner.update(cx, |show, cx| {
                //         show.screen.update(cx, |screen, _cx| {
                //             screen
                //                 .layout_mut()
                //                 .add_window(Window::new(WindowKind::ColorPreset(
                //                     ColorPresetWindow::new(),
                //                 )))
                //         });

                //         show.presets.add_color_preset(ColorPreset::new(
                //             "Magenta",
                //             DmxColor::new(255, 0, 255),
                //         ));
                //     })
                // });

                ShowView::build(cx)
            },
        );
    })
}
