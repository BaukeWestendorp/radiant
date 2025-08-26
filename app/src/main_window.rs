use eyre::Context as _;
use gpui::prelude::*;
use gpui::{
    App, Bounds, Context, Entity, FocusHandle, Focusable, KeyContext, Pixels, TitlebarOptions,
    Window, WindowBounds, WindowHandle, WindowOptions, bounds, div, point, px, size,
};
use ui::interactive::input::TEXT_INPUT_KEY_CONTEXT;
use ui::misc::{TRAFFIC_LIGHT_POSITION, titlebar};
use ui::org::root;
use ui::theme::{ActiveTheme, InteractiveColor};

use crate::error::Result;
use crate::panel::grid::PanelGrid;
use crate::panel::pool::{ObjectPool, PoolPanel, PoolPanelKind};
use crate::panel::window::{
    AttributeEditorPanel, CommandLinePanel, ExecutorsPanel, FixturesTablePanel, WindowPanel,
    WindowPanelKind,
};
use crate::panel::{Panel, PanelKind};
use crate::state::with_show;

pub const CELL_SIZE: Pixels = px(80.0);

pub const MAIN_WINDOW_KEY_CONTEXT: &str = "MainWindow";

pub struct MainWindow {
    panel_grid: Entity<PanelGrid>,
    focus_handle: FocusHandle,
    has_rendered: bool,
}

impl MainWindow {
    pub fn open(cx: &mut App) -> Result<WindowHandle<MainWindow>> {
        let window_bounds = Bounds::centered(None, size(px(1600.0), px(994.0)), cx);
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(window_bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some("Radiant".into()),
                appears_transparent: true,
                traffic_light_position: Some(TRAFFIC_LIGHT_POSITION),
            }),
            app_id: Some("radiant".to_string()),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| Self {
                panel_grid: temporary_panel_grid(window, cx),
                focus_handle: cx.focus_handle(),
                has_rendered: false,
            })
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

        let root = root().flex().flex_col().size_full().child(titlebar).child(content);

        let mut key_context = KeyContext::parse(MAIN_WINDOW_KEY_CONTEXT).unwrap();
        if self.has_rendered {
            // FIXME: This is kind of cursed, but I can't figure out why I'm unable to do
            // this with key contexts.
            if !window.context_stack().iter().any(|ctx| ctx.contains(TEXT_INPUT_KEY_CONTEXT)) {
                key_context.add("cmd_allowed");
            }
        } else {
            self.has_rendered = true;
        }

        div()
            .id("main_window")
            .track_focus(&self.focus_handle(cx))
            .key_context(key_context)
            .size_full()
            .child(root)
    }
}

impl Focusable for MainWindow {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

fn temporary_panel_grid(window: &mut Window, cx: &mut App) -> Entity<PanelGrid> {
    cx.new(|cx| {
        let mut grid = PanelGrid::new(size(20, 12), window, cx);

        grid.add_panel(cx.new(|cx| {
            Panel::new(PanelKind::Pool(PoolPanelKind::Group(
                cx.new(|_| PoolPanel::new(bounds(point(0, 0), size(6, 3)), ObjectPool::new())),
            )))
        }));
        grid.add_panel(cx.new(|cx| {
            Panel::new(PanelKind::Pool(PoolPanelKind::PresetDimmer(
                cx.new(|_| PoolPanel::new(bounds(point(0, 3), size(6, 3)), ObjectPool::new())),
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
                let bounds = bounds(point(0, 10), size(15, 2));
                WindowPanel::new(bounds, ExecutorsPanel::new(bounds.size.width, window, cx))
            }))))
        }));

        grid.add_panel(cx.new(|cx| {
            Panel::new(PanelKind::Window(WindowPanelKind::CommandLine(cx.new(|cx| {
                WindowPanel::new(
                    bounds(point(15, 10), size(5, 2)),
                    CommandLinePanel::new(window, cx),
                )
            }))))
        }));

        grid
    })
}
