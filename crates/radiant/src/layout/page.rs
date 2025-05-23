use super::Frame;
use crate::{show, ui::FRAME_CELL_SIZE};
use gpui::{App, Entity, Size, Window, div, prelude::*};
use ui::{ActiveTheme, utils::z_stack};

pub const GRID_SIZE: Size<u32> = Size { width: 18, height: 12 };

pub struct Page {
    pub label: String,
    frames: Vec<Entity<Frame>>,
}

impl Page {
    pub fn new(label: String) -> Self {
        Self { label, frames: Vec::new() }
    }
}

impl Render for Page {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let grid = ui::dot_grid(FRAME_CELL_SIZE, cx.theme().colors.grid)
            .w(GRID_SIZE.width as f32 * FRAME_CELL_SIZE)
            .h(GRID_SIZE.height as f32 * FRAME_CELL_SIZE)
            .into_any_element();

        let frames = z_stack(self.frames.clone()).size_full().into_any_element();

        div().size_full().child(z_stack([grid, frames]))
    }
}

impl Page {
    pub fn into_show(self, cx: &App) -> show::Page {
        show::Page {
            label: self.label,
            frames: self.frames.into_iter().map(|frame| frame.read(cx).into_show(cx)).collect(),
        }
    }

    pub fn from_show(
        layout: &Entity<show::Layout>,
        w: &mut Window,
        cx: &mut Context<Page>,
    ) -> Self {
        let loaded_page = &layout.read(cx).main_window.loaded_page;
        let mut page = Self::new(loaded_page.label.clone());

        for frame in &loaded_page.frames.clone() {
            page.frames.push(cx.new(|cx| Frame::from_show(frame, w, cx)))
        }

        page
    }
}
