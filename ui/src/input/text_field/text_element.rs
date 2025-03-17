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

        // Cursor.
        let cursor_x_offset = line.x_for_index(field.cursor_char_offset());
        let cursor_origin = bounds.origin + point(cursor_x_offset, px(0.0));
        let cursor_bounds =
            gpui::bounds(cursor_origin, size(cx.theme().cursor_width, window.line_height()));

        // Selection.
        let char_selection = field.char_selection();
        let start = line.x_for_index(char_selection.start);
        let end = line.x_for_index(char_selection.end);
        let selection_bounds = gpui::bounds(
            bounds.origin + point(start, px(0.0)),
            size(end - start, window.line_height()),
        );

        let prepaint_state = PrepaintState { bounds, line, cursor_bounds, selection_bounds };
        self.field
            .update(cx, |field, _cx| field.last_prepaint_state = Some(prepaint_state.clone()));
        prepaint_state
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
        let Self::PrepaintState { bounds, line, cursor_bounds, selection_bounds } = prepaint;

        let cursor_px_offset = cursor_bounds.right() - bounds.left();
        if cursor_px_offset >= bounds.size.width {
            self.field.update(cx, |field, _cx| {
                let new_offset = cursor_px_offset - bounds.size.width;
                if new_offset > field.scroll_offset {
                    field.scroll_offset = new_offset;
                }
            });
        }

        let scroll_offset = self.field.read(cx).scroll_offset;
        if cursor_px_offset < scroll_offset {
            self.field.update(cx, |field, _cx| {
                let new_offset = cursor_px_offset - cursor_bounds.size.width;
                if new_offset < field.scroll_offset {
                    field.scroll_offset = new_offset;
                }
            });
        }

        let field = self.field.read(cx);
        let should_show_cursor = field.blink_cursor.read(cx).visible();
        let focus_handle = field.focus_handle.clone();

        // Handle Input.
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(*bounds, self.field.clone()),
            cx,
        );

        let text_offset = point(-field.scroll_offset, px(0.0));

        // Paint text.
        _ = line.paint(bounds.origin + text_offset, window.line_height(), window, cx);

        // Paint selection.
        _ = window.paint_quad(fill(*selection_bounds + text_offset, cx.theme().highlight));

        // Paint cursor if visible.
        if should_show_cursor {
            _ = window.paint_quad(fill(*cursor_bounds + text_offset, cx.theme().cursor));
        }
    }
}

impl IntoElement for TextElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

#[derive(Debug, Clone)]
pub(super) struct PrepaintState {
    pub bounds: Bounds<Pixels>,
    pub line: ShapedLine,
    pub cursor_bounds: Bounds<Pixels>,
    pub selection_bounds: Bounds<Pixels>,
}
