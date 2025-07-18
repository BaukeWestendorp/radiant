use gpui::prelude::*;
use gpui::{
    App, Application, Bounds, Context, TitlebarOptions, Window, WindowBounds, WindowOptions, div,
    px, size,
};

use crate::ui::{ActiveTheme, root, titlebar};

struct MainWindowView {}

impl Render for MainWindowView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let titlebar = titlebar(window, cx);
        let content = div().size_full();

        root(cx)
            .flex()
            .flex_col()
            .size_full()
            .bg(cx.theme().colors.bg_primary)
            .child(titlebar)
            .child(content)
    }
}

pub fn run() {
    Application::new().run(|cx: &mut App| {
        crate::ui::init(cx);

        cx.activate(true);

        let bounds = Bounds::centered(None, size(px(500.0), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Radiant".into()),
                    appears_transparent: true,
                    traffic_light_position: Some(crate::ui::TRAFFIC_LIGHT_POSITION),
                }),

                app_id: Some("radiant".to_string()),
                ..Default::default()
            },
            |_, cx| cx.new(|_| MainWindowView {}),
        )
        .unwrap();
    });
}
