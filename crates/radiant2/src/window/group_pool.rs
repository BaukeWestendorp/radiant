use gpui::prelude::*;
use gpui::{div, SharedString, ViewContext};

use super::WindowViewDelegate;

pub struct GroupPoolWindowViewDelegate {}

impl GroupPoolWindowViewDelegate {
    pub fn new() -> Self {
        Self {}
    }
}

impl WindowViewDelegate for GroupPoolWindowViewDelegate {
    fn title(&self, cx: &mut ViewContext<super::WindowView<Self>>) -> Option<SharedString> {
        Some("Groups".into())
    }

    fn render_content(
        &mut self,
        _cx: &mut ViewContext<super::WindowView<Self>>,
    ) -> impl IntoElement {
        div().child("FIXME: This is a group pool")
    }
}
