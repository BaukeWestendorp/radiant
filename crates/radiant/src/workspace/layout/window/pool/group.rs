use gpui::{div, rgb, IntoElement, Model, ParentElement, Styled, WindowContext};

use crate::{
    show::{data_pools::DataPool, Show},
    workspace::layout::LayoutBounds,
};

use super::PoolWindowDelegate;

pub struct GroupPoolWindowDelegate {
    scroll_offset: usize,
    bounds: LayoutBounds,
    show: Model<Show>,
}

impl GroupPoolWindowDelegate {
    pub fn new(scroll_offset: usize, bounds: LayoutBounds, show: Model<Show>) -> Self {
        Self {
            scroll_offset,
            bounds,
            show,
        }
    }
}

impl PoolWindowDelegate for GroupPoolWindowDelegate {
    fn label(&self) -> String {
        "Groups".to_string()
    }

    fn bounds(&self, _cx: &mut WindowContext) -> &LayoutBounds {
        &self.bounds
    }

    fn scroll_offset(&self, _cx: &mut WindowContext) -> usize {
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
}
