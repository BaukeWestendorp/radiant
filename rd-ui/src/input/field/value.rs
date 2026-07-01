use gpui::{App, ElementId, Focusable, SharedString, Window, div, prelude::*};

use crate::{Field, interactive_container};

pub trait FieldValue {
    fn from_str(s: &str) -> Option<Self>
    where
        Self: Sized;

    fn to_shared_string(&self) -> impl Into<SharedString>;

    fn validator(s: &str) -> bool;

    fn submit_validator(s: &str) -> bool;

    fn render_overlay(_window: &mut Window, _cx: &mut App) -> Option<impl IntoElement> {
        Option::<gpui::Empty>::None
    }

    fn render(field: Field<Self>, window: &mut Window, cx: &mut App) -> impl IntoElement
    where
        Self: Sized,
    {
        let id = ElementId::View(field.state.entity_id());
        let focus_handle = field.state.read(cx).text_input.read(cx).focus_handle(cx);
        let disabled = field.state.read(cx).disabled(cx);

        let overlay = Self::render_overlay(window, cx).map(|e| e.into_any_element());

        interactive_container(id, Some(focus_handle))
            .relative()
            .w_full()
            .disabled(disabled)
            .child(div().size_full().px_1().py_0p5().child(field.state.read(cx).text_input.clone()))
            .when_some(overlay, |e, overlay| e.child(div().absolute().inset_0().child(overlay)))
    }
}

impl FieldValue for f64 {
    fn from_str(s: &str) -> Option<Self> {
        s.parse().ok()
    }

    fn to_shared_string(&self) -> impl Into<SharedString> {
        self.to_string()
    }

    fn validator(s: &str) -> bool {
        s.parse::<f64>().is_ok()
    }

    fn submit_validator(s: &str) -> bool {
        s.parse::<f64>().is_ok()
    }
}

impl FieldValue for SharedString {
    fn from_str(s: &str) -> Option<Self> {
        Some(s.into())
    }

    fn to_shared_string(&self) -> impl Into<SharedString> {
        self.clone()
    }

    fn validator(s: &str) -> bool {
        !s.trim().is_empty()
    }

    fn submit_validator(s: &str) -> bool {
        !s.trim().is_empty()
    }
}
