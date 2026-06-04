use gpui::{AnyElement, App, Bounds, SharedString, Window, div, prelude::*};
use rd_ui::TileDelegate;

pub struct AttributeEditorTile {}

impl AttributeEditorTile {
    pub fn new(_window: &mut Window, _cx: &mut App) -> Self {
        Self {}
    }
}

impl TileDelegate for AttributeEditorTile {
    fn title(&self, _cx: &App) -> SharedString {
        "Attribute Editor".into()
    }

    fn render_content(&self, _bounds: Bounds<u32>, _window: &mut Window, cx: &App) -> AnyElement {
        div().size_full().child(rd_ui::todo(cx)).into_any_element()
    }
}
