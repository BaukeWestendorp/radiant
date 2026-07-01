use gpui::{App, Entity, FocusHandle, Focusable, Window, prelude::*};

mod event;
mod state;
mod value;

pub use event::*;
pub use state::*;
pub use value::*;

#[derive(IntoElement)]
pub struct Field<T: FieldValue + 'static> {
    state: Entity<FieldState<T>>,
}

impl<T: FieldValue + 'static> Field<T> {
    pub fn new(state: Entity<FieldState<T>>) -> Self {
        Self { state }
    }
}

impl<T: FieldValue + 'static> Focusable for Field<T> {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.state.read(cx).text_input.focus_handle(cx)
    }
}

impl<T: FieldValue + 'static> RenderOnce for Field<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        T::render(self, window, cx)
    }
}
