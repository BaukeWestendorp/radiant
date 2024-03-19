use gpui::{div, rgb, IntoElement, Model, ParentElement, Styled, WindowContext};

use crate::{
    cmd::{Command, CommandList},
    show::{data_pools::DataPool, Show},
    workspace::layout::{window::Window, LayoutBounds},
};

use super::PoolWindowDelegate;

pub struct GroupPoolWindowDelegate {
    scroll_offset: i32,
    bounds: LayoutBounds,
    show: Model<Show>,
    window_id: usize,
}

impl GroupPoolWindowDelegate {
    pub fn new(
        window_id: usize,
        scroll_offset: i32,
        bounds: LayoutBounds,
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

    fn bounds(&self) -> &LayoutBounds {
        &self.bounds
    }

    fn scroll_offset(&self) -> i32 {
        self.scroll_offset
    }

    fn render_item_for_id(&self, id: usize, cx: &mut WindowContext) -> Option<impl IntoElement> {
        let group = self.show.read(cx).data_pools.fixture_group(id);

        match group {
            Some(group) => {
                let label = group.label().to_string();

                Some(
                    div()
                        .bg(rgb(0x303030))
                        .size_full()
                        .flex()
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

    fn handle_click_item(&mut self, id: usize, cx: &mut gpui::ViewContext<Window<Self>>)
    where
        Self: Sized,
    {
        CommandList::extend([Command::Group, Command::Id(id)], cx);
        CommandList::execute(self.show.clone(), cx);
    }
}
