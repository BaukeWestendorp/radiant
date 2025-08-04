use gpui::prelude::*;
use gpui::{Bounds, Entity, Window, div};
use ui::ActiveTheme;
use ui::utils::z_stack;

use crate::main_window::CELL_SIZE;

pub use attribute_editor::*;
pub use executors::*;
pub use fixtures_table::*;
pub use grid::*;
pub use groups_pool::*;

pub mod attribute_editor;
pub mod executors;
pub mod fixtures_table;
pub mod grid;
pub mod groups_pool;

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
            WindowPanelKind::AttributeEditor(attribute_editor) => {
                attribute_editor.into_any_element()
            }
            WindowPanelKind::Executors(executors) => executors.into_any_element(),
            WindowPanelKind::FixturesTable(table) => table.into_any_element(),
        };

        z_stack([
            div()
                .size_full()
                .border_1()
                .border_color(cx.theme().colors.border)
                .rounded(cx.theme().radius)
                .into_any_element(),
            content.into_any_element(),
        ])
        .size_full()
    }

    fn render_pool(
        &mut self,
        kind: PoolPanelKind,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let content = match kind {
            PoolPanelKind::Groups(pool) => pool.into_any_element(),
        };

        z_stack([
            div()
                .size_full()
                .border_1()
                .border_color(cx.theme().colors.border)
                .rounded(cx.theme().radius)
                .into_any_element(),
            content.into_any_element(),
        ])
        .size_full()
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
    AttributeEditor(Entity<AttributeEditorPanel>),
    Executors(Entity<ExecutorsPanel>),
    FixturesTable(Entity<FixturesTablePanel>),
}

#[derive(Debug, Clone)]
pub enum PoolPanelKind {
    Groups(Entity<GroupsPool>),
}
