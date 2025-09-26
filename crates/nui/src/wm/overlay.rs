use gpui::prelude::*;
use gpui::{AnyView, App, Entity, KeyBinding, SharedString, Window, div};

use crate::input::TextField;

mod actions {
    pub const KEY_CONTEXT: &str = "Overlay";

    gpui::actions!(text_input, [Close]);
}

pub(super) fn init(cx: &mut App) {
    cx.bind_keys([KeyBinding::new("escape", actions::Close, Some(actions::KEY_CONTEXT))]);
}

#[derive(Debug, Clone)]
pub struct Overlay {
    id: String,
    title: SharedString,
    content: AnyView,
    is_modal: bool,
}

impl Overlay {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<SharedString>,
        content: impl Into<AnyView>,
    ) -> Self {
        Self { id: id.into(), title: title.into(), content: content.into(), is_modal: false }
    }

    pub fn as_modal(mut self) -> Self {
        self.is_modal = true;
        self
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &AnyView {
        &self.content
    }

    pub fn is_modal(&self) -> bool {
        self.is_modal
    }
}

pub(super) struct TextModal {
    pub field: Entity<TextField>,
}

impl TextModal {
    pub fn new(initial_value: SharedString, window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            field: cx.new(|cx| {
                let focus_handle = cx.focus_handle();
                focus_handle.focus(window);
                TextField::new("modal_text_field", focus_handle, window, cx)
                    .with_value(initial_value, cx)
            }),
        }
    }
}

impl Render for TextModal {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().w_96().flex().justify_center().items_center().p_2().child(self.field.clone())
    }
}
