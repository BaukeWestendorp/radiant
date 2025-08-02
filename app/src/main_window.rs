use eyre::Context as _;
use gpui::prelude::*;
use gpui::{
    App, Bounds, Context, Entity, Pixels, TitlebarOptions, Window, WindowBounds, WindowHandle,
    WindowOptions, bounds, div, point, px, size,
};
use ui::{ActiveTheme, InteractiveColor, root, titlebar};

use crate::app::with_show;
use crate::error::Result;
use crate::panel::{
    AttributeEditorPanel, ExecutorsPanel, Panel, PanelGrid, PanelKind, WindowPanelKind,
};

pub const CELL_SIZE: Pixels = px(80.0);

pub struct MainWindow {
    panel_grid: Entity<PanelGrid>,
}

impl MainWindow {
    pub fn open(cx: &mut App) -> Result<WindowHandle<MainWindow>> {
        let window_bounds = Bounds::centered(None, size(px(1600.0), px(994.0)), cx);
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(window_bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some("Radiant".into()),
                appears_transparent: true,
                traffic_light_position: Some(ui::TRAFFIC_LIGHT_POSITION),
            }),
            app_id: Some("radiant".to_string()),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| Self {
                panel_grid: cx.new(|cx| {
                    let mut grid = PanelGrid::new(size(20, 12), window, cx);
                    grid.add_panel(cx.new(|cx| {
                        Panel::new(
                            PanelKind::Window(WindowPanelKind::AttributeEditor(
                                cx.new(|cx| AttributeEditorPanel::new(window, cx)),
                            )),
                            bounds(point(0, 4), size(20, 4)),
                        )
                    }));
                    grid.add_panel(cx.new(|cx| {
                        Panel::new(
                            PanelKind::Window(WindowPanelKind::Executors(
                                cx.new(|cx| ExecutorsPanel::new(window, cx)),
                            )),
                            bounds(point(0, 9), size(20, 3)),
                        )
                    }));
                    grid
                }),
            })
        })
        .map_err(|err| eyre::eyre!(err))
        .context("failed to open main window")
    }
}

impl Render for MainWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let showfile_path = match with_show(cx, |show| show.path().cloned()) {
            Some(path) => path.display().to_string(),
            None => "[unsaved showfile]".to_string(),
        };

        let titlebar = titlebar(window, cx)
            .flex()
            .justify_between()
            .child(div().text_sm().text_color(cx.theme().colors.text.muted()).child(showfile_path))
            .pr(ui::TRAFFIC_LIGHT_POSITION.x);

        let content = self.panel_grid.clone();

        root(cx)
            .flex()
            .flex_col()
            .size_full()
            .bg(cx.theme().colors.bg_primary)
            .child(titlebar)
            .child(content)
    }
}
