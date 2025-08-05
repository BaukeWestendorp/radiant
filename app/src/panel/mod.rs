use gpui::prelude::*;
use gpui::{App, Bounds, Window, div};
use ui::ActiveTheme;

use crate::main_window::CELL_SIZE;
use crate::panel::pool::PoolPanelKind;
use crate::panel::window::WindowPanelKind;

pub mod grid;
pub mod pool;
pub mod window;

pub struct Panel {
    kind: PanelKind,
}

impl Panel {
    pub fn new(kind: PanelKind) -> Self {
        Self { kind }
    }
}

impl Render for Panel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bounds = self.kind.bounds(cx);
        div()
            .left(CELL_SIZE * bounds.origin.x as f32)
            .top(CELL_SIZE * bounds.origin.y as f32)
            .w(CELL_SIZE * bounds.size.width as f32)
            .h(CELL_SIZE * bounds.size.height as f32)
            .bg(cx.theme().colors.bg_primary)
            .child(match self.kind.clone() {
                PanelKind::Window(kind) => match kind {
                    WindowPanelKind::Executors(panel) => panel.into_any_element(),
                    WindowPanelKind::AttributeEditor(panel) => panel.into_any_element(),
                    WindowPanelKind::FixturesTable(panel) => panel.into_any_element(),
                },
                PanelKind::Pool(kind) => match kind {
                    PoolPanelKind::Group(panel) => panel.into_any_element(),
                    PoolPanelKind::Sequence(panel) => panel.into_any_element(),
                },
            })
    }
}

#[derive(Clone)]
pub enum PanelKind {
    Window(WindowPanelKind),
    Pool(PoolPanelKind),
}

impl PanelKind {
    pub fn bounds(&self, cx: &App) -> Bounds<u32> {
        match self {
            PanelKind::Window(window_panel) => match window_panel {
                WindowPanelKind::Executors(panel) => panel.read(cx).bounds(),
                WindowPanelKind::AttributeEditor(panel) => panel.read(cx).bounds(),
                WindowPanelKind::FixturesTable(panel) => panel.read(cx).bounds(),
            },
            PanelKind::Pool(pool_panel) => match pool_panel {
                PoolPanelKind::Group(panel) => panel.read(cx).bounds(),
                PoolPanelKind::Sequence(panel) => panel.read(cx).bounds(),
            },
        }
    }
}
