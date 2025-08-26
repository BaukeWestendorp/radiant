use gpui::prelude::*;
use gpui::{Entity, Size, Window, div};
use ui::misc::dot_grid;
use ui::theme::ActiveTheme;
use ui::utils::z_stack;

use crate::main_window::CELL_SIZE;
use crate::panel::Panel;

pub struct PanelGrid {
    size: Size<u32>,
    panels: Vec<Entity<Panel>>,
}

impl PanelGrid {
    pub fn new(size: Size<u32>, _window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { size, panels: Vec::new() }
    }

    pub fn add_panel(&mut self, panel: Entity<Panel>) {
        self.panels.push(panel)
    }
}

impl Render for PanelGrid {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let panels = z_stack(self.panels.clone()).size_full().into_any();

        div().w(CELL_SIZE * self.size.width as f32).h(CELL_SIZE * self.size.height as f32).child(
            z_stack([dot_grid(CELL_SIZE, cx.theme().colors.grid).size_full().into_any(), panels])
                .size_full(),
        )
    }
}
