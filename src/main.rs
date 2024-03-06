use assets::Assets;
use gpui::{
    App, AppContext, AssetSource, Bounds, KeyBinding, Point, Size, WindowBounds, WindowOptions,
};
use show::{cmd, Show, ShowView};

pub mod dmx;
pub mod layout;
pub mod presets;
pub mod screen;
pub mod show;
pub mod window;

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
            KeyBinding::new("s", cmd::Store, Some("Show")),
            KeyBinding::new("escape", cmd::Clear, Some("Show")),
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
                cx.set_global(Show::new());
                ShowView::build(cx)
            },
        );
    })
}
