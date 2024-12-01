use gpui::*;

use crate::theme::ActiveTheme;

use super::TextField;

pub struct TextElement {
    pub field: View<TextField>,
}

impl TextElement {
    fn paint_mouse_listeners(&mut self, cx: &mut WindowContext) {
        cx.on_mouse_event({
            let field = self.field.clone();
            move |event: &MouseMoveEvent, _, cx| {
                if event.pressed_button == Some(MouseButton::Left) {
                    field.update(cx, |field, cx| {
                        field.on_drag_move(event, cx);
                    })
                }
            }
        });
    }
}

impl IntoElement for TextElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

pub struct PrepaintState {
    scroll_offset: Point<Pixels>,
    line: Option<ShapedLine>,
    cursor: Option<PaintQuad>,
    selection: Option<PaintQuad>,
    bounds: Bounds<Pixels>,
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
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.0).into();
        style.size.height = cx.line_height().into();
        (cx.request_layout(style, []), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Self::PrepaintState {
        let field = self.field.read(cx);
        let value = field.value.clone();
        let placeholder = field.placeholder.clone();
        let selected_range = field.selected_range.clone();
        let cursor = field.cursor_offset();
        let style = cx.text_style();

        let (display_text, text_color) = if value.is_empty() {
            (placeholder, cx.theme().text_placeholder)
        } else {
            (value, cx.theme().text)
        };

        let run = TextRun {
            len: display_text.len(),
            font: style.font(),
            color: text_color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };

        let runs = if let Some(marked_range) = field.marked_range.as_ref() {
            vec![
                TextRun {
                    len: marked_range.start,
                    ..run.clone()
                },
                TextRun {
                    len: marked_range.end - marked_range.start,
                    underline: Some(UnderlineStyle {
                        color: Some(run.color),
                        thickness: px(1.0),
                        wavy: false,
                    }),
                    ..run.clone()
                },
                TextRun {
                    len: display_text.len() - marked_range.end,
                    ..run.clone()
                },
            ]
            .into_iter()
            .filter(|run| run.len > 0)
            .collect()
        } else {
            vec![run]
        };

        let font_size = style.font_size.to_pixels(cx.rem_size());
        let line = cx
            .text_system()
            .shape_line(display_text, font_size, &runs)
            .unwrap();

        // Calculate the scroll offset to keep the cursor in view
        let mut scroll_offset = field.scroll_offset;
        let mut bounds = bounds;
        let right_margin = px(5.);
        let cursor_pos = line.x_for_index(cursor);
        let cursor_start = line.x_for_index(selected_range.start);
        let cursor_end = line.x_for_index(selected_range.end);

        scroll_offset.x = if scroll_offset.x + cursor_pos > (bounds.size.width - right_margin) {
            // cursor is out of right
            bounds.size.width - right_margin - cursor_pos
        } else if scroll_offset.x + cursor_pos < px(0.) {
            // cursor is out of left
            scroll_offset.x - cursor_pos
        } else {
            scroll_offset.x
        };

        if field.selection_reversed {
            if scroll_offset.x + cursor_start < px(0.) {
                // selection start is out of left
                scroll_offset.x = -cursor_start;
            }
        } else if scroll_offset.x + cursor_end <= px(0.) {
            // selection end is out of left
            scroll_offset.x = -cursor_end;
        }

        bounds.origin += scroll_offset;

        let inset = px(0.5);
        let cursor_pos = line.x_for_index(cursor);
        let (selection, cursor) = if selected_range.is_empty() && field.show_cursor(cx) {
            // Blink the cursor
            (
                None,
                Some(fill(
                    Bounds::new(
                        point(bounds.left() + cursor_pos, bounds.top() + inset),
                        size(
                            cx.theme().cursor_width,
                            bounds.bottom() - bounds.top() - inset * 2,
                        ),
                    ),
                    cx.theme().accent,
                )),
            )
        } else {
            // Selection (No blinking cursor)
            (
                Some(fill(
                    Bounds::from_corners(
                        point(
                            bounds.left() + line.x_for_index(selected_range.start) + inset,
                            bounds.top() + inset,
                        ),
                        point(
                            bounds.left() + line.x_for_index(selected_range.end) - inset,
                            bounds.bottom() - inset,
                        ),
                    ),
                    cx.theme().text_accent,
                )),
                None,
            )
        };

        PrepaintState {
            scroll_offset,
            line: Some(line),
            cursor,
            selection,
            bounds,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        let focus_handle = self.field.read(cx).focus_handle.clone();
        let focused = focus_handle.is_focused(cx);
        let bounds = prepaint.bounds;

        cx.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.field.clone()),
        );

        if let Some(selection) = prepaint.selection.take() {
            cx.paint_quad(selection);
        }

        let line = prepaint.line.take().unwrap();
        line.paint(bounds.origin, cx.line_height(), cx).unwrap();

        if focused {
            if let Some(cursor) = prepaint.cursor.take() {
                cx.paint_quad(cursor);
            }
        }

        self.field.update(cx, |field, _cx| {
            field.scroll_offset = prepaint.scroll_offset;
            field.last_layout = Some(line);
            field.last_bounds = Some(bounds);
        });

        self.paint_mouse_listeners(cx);
    }
}
