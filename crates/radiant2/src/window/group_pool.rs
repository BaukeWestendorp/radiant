use gpui::{IntoElement, Model, ParentElement, Styled, WindowContext};
use ui::button::{Button, ButtonStyle};

use super::pool::PoolWindowDelegate;
use super::WindowView;
use crate::showfile::{ShowfileManager, Window};

pub struct GroupPoolWindowDelegate {
    window: Model<Window>,
}

impl GroupPoolWindowDelegate {
    pub fn new(window: Model<Window>) -> Self {
        Self { window }
    }
}

impl PoolWindowDelegate for GroupPoolWindowDelegate {
    fn label(&self) -> String {
        "Groups".to_string()
    }

    fn window(&self) -> &Model<Window> {
        &self.window
    }

    fn render_item_for_id(&self, id: usize, cx: &mut WindowContext) -> Option<impl IntoElement> {
        if let Some(group) = ShowfileManager::show(cx).group(id) {
            Some(
                Button::new(ButtonStyle::Secondary, id, cx)
                    .size_full()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(group.label.clone()),
            )
        } else {
            None
        }
    }

    fn handle_click_item(&mut self, _id: usize, _cx: &mut gpui::ViewContext<WindowView<Self>>) {
        todo!("Handle clicking group pool item");
    }
}
