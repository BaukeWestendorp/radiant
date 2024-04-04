use gpui::{IntoElement, Model, WindowContext};

use super::pool::PoolWindowDelegate;
use super::WindowView;
use crate::showfile::Window;

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

    fn render_item_for_id(&self, _id: usize, _cx: &mut WindowContext) -> Option<impl IntoElement> {
        Option::<gpui::Empty>::None
    }

    fn handle_click_item(&mut self, _id: usize, _cx: &mut gpui::ViewContext<WindowView<Self>>) {
        todo!("Handle clicking group pool item");
    }
}
