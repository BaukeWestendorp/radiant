use gpui::{IntoElement, Model, ParentElement, Styled, WindowContext};
use theme::ActiveTheme;
use ui::container::{Container, ContainerStyle};

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
                Container::new(cx)
                    .container_style(ContainerStyle {
                        background: cx.theme().colors().element_background_secondary,
                        border: cx.theme().colors().border,
                    })
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
