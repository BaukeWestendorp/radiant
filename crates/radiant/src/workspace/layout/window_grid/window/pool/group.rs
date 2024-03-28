use backstage::command::{Command, Instruction, Object};
use backstage::show::Show;
use gpui::InteractiveElement;
use gpui::{div, prelude::FluentBuilder, IntoElement, Model, ParentElement, Styled, WindowContext};

use crate::workspace::layout::window_grid::window::WindowView;
use crate::workspace::layout::window_grid::GridBounds;
use theme::ActiveTheme;

use super::PoolWindowDelegate;

pub struct GroupPoolWindowDelegate {
    scroll_offset: i32,
    bounds: GridBounds,
    show: Model<Show>,
    window_id: usize,
}

impl GroupPoolWindowDelegate {
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

impl PoolWindowDelegate for GroupPoolWindowDelegate {
    fn label(&self) -> String {
        "Groups".to_string()
    }

    fn bounds(&self) -> &GridBounds {
        &self.bounds
    }

    fn scroll_offset(&self) -> i32 {
        self.scroll_offset
    }

    fn render_item_for_id(&self, id: usize, cx: &mut WindowContext) -> Option<impl IntoElement> {
        let group = self.show.read(cx).group(id);

        match group {
            Some(group) => {
                let label = group.label.to_string();
                let is_in_programmer_selection =
                    self.show.read(cx).are_fixtures_selected(&group.fixtures);

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
                        .child(label)
                        .when(is_in_programmer_selection, |this| {
                            this.child(div().w_full().h_3().bg(gpui::green()))
                        }),
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
            if let Err(err) =
                show.execute_command(&Command::new([Instruction::Select(Object::Group(id))]))
            {
                log::error!("Failed to Select Group {id}: {}", err.to_string())
            }
            cx.notify();
        });
    }
}
