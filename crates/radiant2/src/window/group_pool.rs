use gpui::prelude::*;
use gpui::{div, SharedString, ViewContext};

use super::{WindowView, WindowViewDelegate};

pub struct GroupPoolWindowViewDelegate {}

impl GroupPoolWindowViewDelegate {
    pub fn new() -> Self {
        Self {}
    }
}

impl WindowViewDelegate for GroupPoolWindowViewDelegate {
    fn title(&self, _cx: &mut ViewContext<WindowView<Self>>) -> Option<SharedString> {
        Some("Groups".into())
    }

    fn render_content(&mut self, _cx: &mut ViewContext<WindowView<Self>>) -> impl IntoElement {
        div().child("FIXME: This is a group pool")
    }

    fn render_header(
        &mut self,
        _cx: &mut ViewContext<WindowView<Self>>,
    ) -> Option<impl IntoElement> {
        Option::<gpui::Empty>::None
    }
}
