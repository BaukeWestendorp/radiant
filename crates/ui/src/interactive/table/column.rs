use gpui::{Pixels, SharedString, px};

#[derive(Debug, Clone)]
pub struct Column {
    pub id: SharedString,
    pub label: SharedString,
    pub width: Pixels,
}

impl Column {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self { id: id.into(), label: label.into(), width: px(100.0) }
    }

    pub fn with_width(mut self, width: Pixels) -> Self {
        self.width = width;
        self
    }
}
