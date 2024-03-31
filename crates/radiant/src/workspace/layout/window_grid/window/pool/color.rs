use backstage::command::{Command, Object};
use backstage::show::{Preset, Show};
use gpui::{div, InteractiveElement, IntoElement, Model, ParentElement, Styled, WindowContext};

use crate::workspace::layout::window_grid::window::WindowView;
use crate::workspace::layout::window_grid::GridBounds;
use theme::ActiveTheme;

use super::PoolWindowDelegate;

pub struct ColorPoolWindowDelegate {
    scroll_offset: i32,
    bounds: GridBounds,
    show: Model<Show>,
    window_id: usize,
}

impl ColorPoolWindowDelegate {
    pub fn new(
        window_id: usize,
        scroll_offset: i32,
        bounds: GridBounds,
        show: Model<Show>,
    ) -> Self {
        Self {
            scroll_offset,
            bounds,
            show,
            window_id,
        }
    }
}

impl PoolWindowDelegate for ColorPoolWindowDelegate {
    fn label(&self) -> String {
        "Colors".to_string()
    }

    fn bounds(&self) -> &GridBounds {
        &self.bounds
    }

    fn scroll_offset(&self) -> i32 {
        self.scroll_offset
    }

    fn render_item_for_id(&self, id: usize, cx: &mut WindowContext) -> Option<impl IntoElement> {
        let color_preset = self.show.read(cx).color_preset(id);

        match color_preset {
            Some(color_preset) => {
                let label = color_preset.label().to_string();

                Some(
                    div()
                        .bg(cx.theme().colors().background)
                        .cursor_pointer()
                        .hover(|this| this.bg(cx.theme().colors().element_background_hover))
                        .size_full()
                        .flex()
                        .flex_col()
                        .justify_center()
                        .items_center()
                        .text_sm()
                        .child(label),
                )
            }
            None => None,
        }
    }

    fn window_id(&self) -> usize {
        self.window_id
    }

    fn handle_click_item(&mut self, id: usize, cx: &mut gpui::ViewContext<WindowView<Self>>) {
        self.show.update(cx, |show, cx| {
            if let Err(err) = show.execute_command(&Command::Select(Some(Object::PresetColor(id))))
            {
                log::error!("Failed to Select Group {id}: {}", err.to_string())
            }
            cx.notify();
        });
    }
}
