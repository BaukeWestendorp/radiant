use app::RadiantApp;
use gpui::*;

mod app;
mod effect_graph;
mod frame;

actions!(app, [Quit]);

fn main() {
    Application::new().run(|cx: &mut App| {
        ui::theme::Theme::init(cx);

        cx.bind_keys([KeyBinding::new("ctrl-q", Quit, None)]);

        cx.on_action::<Quit>(|_, cx| cx.quit());

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1600.0), px(960.0)),
                    cx,
                ))),
                app_id: Some("radiant".to_string()),

                ..Default::default()
            },
            |window, cx| RadiantApp::build(window, cx),
        )
        .expect("should open window");
    });
}
