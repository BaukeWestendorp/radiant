use gpui::prelude::*;
use gpui::{Bounds, Entity, Window, div};
use ui::ActiveTheme;
use ui::utils::z_stack;

mod attribute_editor;
mod command_line;
mod executors;
mod fixtures_table;

pub use attribute_editor::AttributeEditorPanel;
pub use command_line::CommandLinePanel;
pub use executors::ExecutorsPanel;
pub use fixtures_table::FixturesTablePanel;

#[derive(Clone)]
pub enum WindowPanelKind {
    Executors(Entity<WindowPanel<ExecutorsPanel>>),
    AttributeEditor(Entity<WindowPanel<AttributeEditorPanel>>),
    FixturesTable(Entity<WindowPanel<FixturesTablePanel>>),
    CommandLine(Entity<WindowPanel<CommandLinePanel>>),
}

pub struct WindowPanel<D: WindowPanelDelegate> {
    bounds: Bounds<u32>,
    delegate: D,
}

impl<D: WindowPanelDelegate> WindowPanel<D> {
    pub fn new(bounds: Bounds<u32>, delegate: D) -> Self {
        Self { bounds, delegate }
    }

    pub fn bounds(&self) -> Bounds<u32> {
        self.bounds
    }
}

impl<D: WindowPanelDelegate + 'static> Render for WindowPanel<D> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.delegate.render(window, cx)
    }
}

pub trait WindowPanelDelegate {
    fn render_content(
        &mut self,
        window: &mut Window,
        cx: &mut Context<WindowPanel<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized;

    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut Context<WindowPanel<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized,
    {
        z_stack([
            div()
                .size_full()
                .border_1()
                .border_color(cx.theme().colors.border)
                .rounded(cx.theme().radius)
                .into_any_element(),
            self.render_content(window, cx).into_any_element(),
        ])
        .size_full()
    }
}
