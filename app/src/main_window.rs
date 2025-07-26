use eyre::Context as _;
use gpui::prelude::*;
use gpui::{
    App, Bounds, Context, Entity, ReadGlobal, TitlebarOptions, Window, WindowBounds, WindowHandle,
    WindowOptions, div, px, size,
};

use crate::app::AppState;
use crate::attribute_editor::AttributeEditor;
use crate::error::Result;
use ui::{ActiveTheme, InteractiveColor, root, titlebar};

pub struct MainWindow {
    attribute_editor: Entity<AttributeEditor>,
}

impl MainWindow {
    pub fn open(cx: &mut App) -> Result<WindowHandle<MainWindow>> {
        let bounds = Bounds::centered(None, size(px(1080.0), px(720.0)), cx);
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some("Radiant".into()),
                appears_transparent: true,
                traffic_light_position: Some(ui::TRAFFIC_LIGHT_POSITION),
            }),
            app_id: Some("radiant".to_string()),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| Self { attribute_editor: cx.new(|cx| AttributeEditor::new(window, cx)) })
        })
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
            .flex()
            .justify_between()
            .child(div().text_sm().text_color(cx.theme().colors.text.muted()).child(showfile_path))
            .pr(ui::TRAFFIC_LIGHT_POSITION.x);

        let content = div().size_full().child(self.attribute_editor.clone());

        root(cx)
            .flex()
            .flex_col()
            .size_full()
            .bg(cx.theme().colors.bg_primary)
            .child(titlebar)
            .child(content)
    }
}
