use app::RadiantApp;
use gpui::*;

mod app;
mod effect_graph;
mod frame;

fn main() {
    Application::new().run(|cx: &mut App| {
        ui::theme::Theme::init(cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1600.0), px(960.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |_, cx| RadiantApp::build(cx),
        )
        .expect("should open window");
    });
}
