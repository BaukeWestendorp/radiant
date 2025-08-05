use eyre::Context as _;
use gpui::prelude::*;
use gpui::{
    App, Bounds, Context, Entity, Pixels, TitlebarOptions, Window, WindowBounds, WindowHandle,
    WindowOptions, bounds, div, point, px, size,
};
use ui::{ActiveTheme, InteractiveColor, root, titlebar};

use crate::error::Result;
use crate::panel::grid::PanelGrid;
use crate::panel::pool::{ObjectPool, PoolPanel, PoolPanelKind};
use crate::panel::window::{
    AttributeEditorPanel, ExecutorsPanel, FixturesTablePanel, WindowPanel, WindowPanelKind,
};
use crate::panel::{Panel, PanelKind};
use crate::state::with_show;

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
            cx.new(|cx| Self { panel_grid: temporary_panel_grid(window, cx) })
        })
        .map_err(|err| eyre::eyre!(err))
        .context("failed to open main window")
    }
}

impl Render for MainWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut showfile_path = match with_show(cx, |show| show.path().cloned()) {
            Some(path) => path.display().to_string(),
            None => "[unsaved showfile]".to_string(),
        };

        if cfg!(debug_assertions) {
            showfile_path = format!("[DEBUG] {showfile_path}");
        }

        let titlebar = titlebar(window, cx)
            .flex()
            .child(div().text_color(cx.theme().colors.text.muted()).child(showfile_path));

        let content = self.panel_grid.clone();

        root(cx).flex().flex_col().size_full().child(titlebar).child(content)
    }
}

fn temporary_panel_grid(window: &mut Window, cx: &mut App) -> Entity<PanelGrid> {
    cx.new(|cx| {
        let mut grid = PanelGrid::new(size(20, 12), window, cx);

        grid.add_panel(cx.new(|cx| {
            Panel::new(PanelKind::Pool(PoolPanelKind::Group(
                cx.new(|_| PoolPanel::new(bounds(point(0, 0), size(7, 3)), ObjectPool::new())),
            )))
        }));

        grid.add_panel(cx.new(|cx| {
            Panel::new(PanelKind::Pool(PoolPanelKind::Sequence(
                cx.new(|_| PoolPanel::new(bounds(point(0, 3), size(7, 3)), ObjectPool::new())),
            )))
        }));

        grid.add_panel(cx.new(|cx| {
            Panel::new(PanelKind::Window(WindowPanelKind::FixturesTable(cx.new(|cx| {
                WindowPanel::new(
                    bounds(point(12, 0), size(8, 7)),
                    FixturesTablePanel::new(window, cx),
                )
            }))))
        }));

        grid.add_panel(cx.new(|cx| {
            Panel::new(PanelKind::Window(WindowPanelKind::AttributeEditor(cx.new(|cx| {
                WindowPanel::new(
                    bounds(point(0, 7), size(20, 3)),
                    AttributeEditorPanel::new(window, cx),
                )
            }))))
        }));

        grid.add_panel(cx.new(|cx| {
            Panel::new(PanelKind::Window(WindowPanelKind::Executors(cx.new(|cx| {
                let bounds = bounds(point(0, 10), size(20, 2));
                WindowPanel::new(bounds, ExecutorsPanel::new(bounds.size.width, window, cx))
            }))))
        }));

        grid
    })
}
