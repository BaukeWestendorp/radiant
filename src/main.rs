use assets::Assets;
use gpui::{
    App, AppContext, AssetSource, Bounds, Context, KeyBinding, Point, Size, WindowBounds,
    WindowOptions,
};
use show::Show;
use ui::show::ShowView;

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

        cx.bind_keys([KeyBinding::new("s", show::cmd::Store, Some("Show"))]);

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
                let show_model = cx.new_model(|_cx| Show::new());

                ShowView::build(show_model, cx)
            },
        );
    })
}
