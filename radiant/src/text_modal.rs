use gpui::prelude::*;
use gpui::{App, Entity, FocusHandle, SharedString, Window, div};
use ui::interactive::input::{FieldEvent, TextField};
use ui::interactive::modal::{Modal, ModalDelegate, ModalExt};

pub struct TextModal {
    field: Entity<TextField>,
}

impl TextModal {
    pub fn new(
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<Modal<Self>>,
    ) -> Self {
        let field = cx.new(|cx| TextField::new("modal_field", focus_handle, window, cx));

        cx.subscribe(&field, |_, _, event, cx| match event {
            FieldEvent::Submit => cx.close_modal(),
            _ => {}
        })
        .detach();

        Self { field }
    }

    pub fn field(&self) -> &Entity<TextField> {
        &self.field
    }

    pub fn with_value(self, value: SharedString, cx: &mut App) -> Self {
        self.field.update(cx, |field, cx| field.set_value(value, cx));
        self
    }
}

impl ModalDelegate for TextModal {
    fn render_content(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Modal<Self>>,
    ) -> impl IntoElement {
        div().w_full().child(self.field.clone())
    }
}
