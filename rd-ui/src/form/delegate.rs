use gpui::{AnyElement, App, IntoElement, SharedString};

pub struct FormField {
    pub label: SharedString,
    pub input: AnyElement,
}

impl FormField {
    pub fn new(label: impl Into<SharedString>, input: impl IntoElement) -> Self {
        Self { label: label.into(), input: input.into_any_element() }
    }
}

pub trait FormDelegate {
    type Data;

    fn fields(&self) -> Vec<FormField>;

    fn extract_data(&self, cx: &App) -> Option<Self::Data>;
}
