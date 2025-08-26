use gpui::prelude::*;
use gpui::{App, Context, Entity, Focusable, Window, div};
use ui::interactive::input::{Field, FieldEvent};
use ui::interactive::modal::{Modal, ModalDelegate};

use crate::main_window::MainWindow;

pub struct StringModal {
    field: Entity<Field<String>>,
    on_submit: Option<Box<dyn Fn(&str, &mut App)>>,
}

impl StringModal {
    pub fn new(window: &mut Window, cx: &mut Context<Modal<Self>>) -> Self {
        let field = cx.new(|cx| Field::new("string_field", cx.focus_handle(), window, cx));
        window.focus(&field.focus_handle(cx));
        cx.subscribe_in(&field, window, Self::handle_field_event).detach();
        Self { field, on_submit: None }
    }

    pub fn on_submit<F: Fn(&str, &mut App) + 'static>(mut self, on_submit: F) -> Self {
        self.on_submit = Some(Box::new(on_submit));
        self
    }

    fn handle_field_event(
        modal: &mut Modal<Self>,
        _field: &Entity<Field<String>>,
        event: &FieldEvent<String>,
        window: &mut Window,
        cx: &mut Context<Modal<Self>>,
    ) {
        match event {
            FieldEvent::Submit(value) => {
                if let Some(on_submit) = &modal.delegate.on_submit {
                    match window.root::<MainWindow>() {
                        Some(Some(main_window)) => main_window.focus_handle(cx).focus(window),
                        _ => {}
                    }
                    on_submit(value, cx);
                }
            }

            _ => {}
        }
    }
}

impl ModalDelegate for StringModal {
    fn render_content(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut Context<Modal<Self>>,
    ) -> impl gpui::IntoElement {
        div().size_full().child(div().w_full().child(self.field.clone()))
    }
}
