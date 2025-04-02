use app::RadiantApp;
use gpui::*;

mod app;
mod effect_graph;
mod frame;

fn main() {
    env_logger::init();

    Application::new().run(|cx: &mut App| {
        ui::init(cx);
        ui::actions::init(cx);
        flow::gpui::actions::init(cx);
        actions::init(cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(1600.0), px(960.0)),
                cx,
            ))),
            app_id: Some("radiant".to_string()),

            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            window.set_rem_size(px(14.0));
            cx.new(|cx| RadiantApp::new(window, cx))
        })
        .expect("should open window");
    });
}

mod actions {
    use gpui::*;

    actions!(app, [Quit]);

    pub fn init(cx: &mut App) {
        bind_global_keys(cx);
        handle_global_actions(cx);
    }

    fn bind_global_keys(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("secondary-q", Quit, None)]);
    }

    fn handle_global_actions(cx: &mut App) {
        cx.on_action::<Quit>(|_, cx| cx.quit());
    }
}
