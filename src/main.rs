use assets::Assets;
use gpui::{
    App, AppContext, AssetSource, Bounds, Point, Size, VisualContext, WindowBounds, WindowOptions,
};
use show::Show;
use ui::{
    layout::Layout,
    window::{ColorPresetWindow, Window, WindowKind},
};

pub mod dmx;
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

        let mut show = Show::new();
        let main_layout_id = show.add_layout(Layout::new());
        let main_layout = show.layout_mut(main_layout_id);
        main_layout.add_window(Window::new(WindowKind::ColorPreset(
            ColorPresetWindow::new(),
        )));
        cx.set_global(show);

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
                    let main_layout = cx.global::<Show>().layout(main_layout_id);
                    main_layout.clone()
                })
            },
        );
    })
}
