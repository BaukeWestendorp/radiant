use gpui::prelude::*;
use gpui::{Bounds, Entity, Window, div};
use ui::ActiveTheme;

use crate::main_window::CELL_SIZE;

pub use attribute_editor::*;
pub use grid::*;

pub mod attribute_editor;
pub mod grid;

pub struct Panel {
    bounds: Bounds<u32>,
    kind: PanelKind,
}

impl Panel {
    pub fn new(kind: PanelKind, bounds: Bounds<u32>) -> Self {
        Self { kind, bounds }
    }

    fn render_window(
        &mut self,
        kind: WindowPanelKind,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let content = match kind {
            WindowPanelKind::AttributeEditor(attribute_editor) => attribute_editor,
        };

        div()
            .size_full()
            .border_1()
            .border_color(cx.theme().colors.border)
            .rounded(cx.theme().radius)
            .child(content)
    }

    fn render_pool(
        &mut self,
        kind: PoolPanelKind,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .size_full()
            .border_1()
            .border_color(cx.theme().colors.border)
            .rounded(cx.theme().radius)
    }
}

impl Render for Panel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .left(CELL_SIZE * self.bounds.origin.x as f32)
            .top(CELL_SIZE * self.bounds.origin.y as f32)
            .w(CELL_SIZE * self.bounds.size.width as f32)
            .h(CELL_SIZE * self.bounds.size.height as f32)
            .bg(cx.theme().colors.bg_primary)
            .child(match &self.kind {
                PanelKind::Window(kind) => {
                    self.render_window(kind.clone(), window, cx).into_any_element()
                }
                PanelKind::Pool(kind) => {
                    self.render_pool(kind.clone(), window, cx).into_any_element()
                }
            })
    }
}

#[derive(Debug, Clone)]
pub enum PanelKind {
    Window(WindowPanelKind),
    Pool(PoolPanelKind),
}

#[derive(Debug, Clone)]
pub enum WindowPanelKind {
    AttributeEditor(Entity<AttributeEditor>),
}

#[derive(Debug, Clone)]
pub enum PoolPanelKind {}
