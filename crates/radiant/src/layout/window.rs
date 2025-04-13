use gpui::*;

pub struct LayoutWindow {}

pub struct MainWindow {}

impl MainWindow {
    pub fn open(cx: &mut App) -> WindowHandle<Self> {
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
            cx.new(|cx| RadiantApp::new(showfile, window, cx))
        })
        .expect("should open window")
    }
}
