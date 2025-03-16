use super::TextField;
use crate::theme::ActiveTheme;
use gpui::*;

pub(super) struct TextElement {
    field: Entity<TextField>,
}

impl TextElement {
    pub fn new(field: Entity<TextField>) -> Self {
        Self { field }
    }
}

impl Element for TextElement {
    type RequestLayoutState = ();
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.height = window.line_height().into();

        let layout_id = window.request_layout(style, [], cx);
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let field = self.field.read(cx);
        let style = window.text_style();

        // Get text to show.
        let (display_text, text_color) = if field.text().is_empty() {
            (field.placeholder(), cx.theme().text_muted)
        } else {
            (field.text(), cx.theme().text_primary)
        };

        // Line.
        let font_size = style.font_size.to_pixels(window.rem_size());
        let line = window
            .text_system()
            .shape_line(
                display_text.into(),
                font_size,
                &[TextRun {
                    len: display_text.len(),
                    font: style.font(),
                    color: text_color,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                }],
            )
            .unwrap();

        let cursor_x_offset = line.x_for_index(field.cursor_char_offset());
        let cursor_origin = bounds.origin + point(cursor_x_offset, px(0.0));
        let cursor_bounds = gpui::bounds(cursor_origin, size(px(1.0), window.line_height()));

        PrepaintState { bounds, line, cursor_bounds }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.field.read(cx).focus_handle.clone();
        let Self::PrepaintState { bounds, line, cursor_bounds } = prepaint;

        // Handle Input.
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(*bounds, self.field.clone()),
            cx,
        );

        // Paint text.
        _ = line.paint(bounds.origin, window.line_height(), window, cx);

        // Paint cursor
        _ = window.paint_quad(fill(*cursor_bounds, cx.theme().accent));
    }
}

impl IntoElement for TextElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

pub(super) struct PrepaintState {
    bounds: Bounds<Pixels>,
    line: ShapedLine,
    cursor_bounds: Bounds<Pixels>,
}
