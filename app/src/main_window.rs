use eyre::Context as _;
use gpui::prelude::*;
use gpui::{
    App, Bounds, Context, ReadGlobal, TitlebarOptions, Window, WindowBounds, WindowHandle,
    WindowOptions, div, px, size,
};

use crate::app::AppState;
use crate::error::Result;
use crate::ui::{ActiveTheme, InteractiveColor, root, titlebar};

pub struct MainWindow {}

impl MainWindow {
    pub fn open(cx: &mut App) -> Result<WindowHandle<MainWindow>> {
        let bounds = Bounds::centered(None, size(px(500.0), px(500.0)), cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some("Radiant".into()),
                appears_transparent: true,
                traffic_light_position: Some(crate::ui::TRAFFIC_LIGHT_POSITION),
            }),

            app_id: Some("radiant".to_string()),
            ..Default::default()
        };

        cx.open_window(window_options, |_, cx| cx.new(|_| Self {}))
            .map_err(|err| eyre::eyre!(err))
            .context("failed to open main window")
    }
}

impl Render for MainWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let showfile_path = match AppState::global(cx).engine.show().path() {
            Some(path) => path.display().to_string(),
            None => "[unsaved showfile]".to_string(),
        };

        let titlebar = titlebar(window, cx)
            .child(div().text_sm().text_color(cx.theme().colors.text.muted()).child(showfile_path));

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
