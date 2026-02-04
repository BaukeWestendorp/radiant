use gpui::{Pixels, SharedString, px};
pub struct Column {
    id: SharedString,
    label: SharedString,
    min_width: Pixels,
}

impl Column {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self { id: id.into(), label: label.into(), min_width: px(100.0) }
    }

    pub fn id(&self) -> &SharedString {
        &self.id
    }

    pub fn label(&self) -> &SharedString {
        &self.label
    }

    pub fn min_width(&self) -> Pixels {
        self.min_width
    }

    pub fn with_min_width(mut self, min_width: Pixels) -> Self {
        self.min_width = min_width;
        self
    }
}
