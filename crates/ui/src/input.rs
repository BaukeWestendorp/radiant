use gpui::*;
use prelude::FluentBuilder;

use crate::{theme::ActiveTheme, StyledExt, INPUT_HEIGHT};

const DRAG_MULTIPLIER: f32 = 0.5;

pub struct IntField {
    id: ElementId,
    focus_handle: FocusHandle,
    value: i32,
}

impl IntField {
    pub fn new(id: impl Into<ElementId>, cx: &mut ViewContext<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        Self {
            id: id.into(),
            focus_handle: focus_handle.clone(),
            value: 0,
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn set_value(&mut self, value: i32, cx: &mut ViewContext<Self>) {
        self.value = value;
        cx.emit(InputEvent::ChangeValue(value));
    }

    pub fn focus(&self, cx: &mut ViewContext<Self>) {
        self.focus_handle.focus(cx);
    }

    fn on_drag_move(&mut self, event: &DragMoveEvent<Drag>, cx: &mut ViewContext<Self>) {
        let drag = event.drag(cx);

        if drag.id != self.id {
            return;
        }

        let delta_x = cx.mouse_position().x.0 - drag.start_mouse_position.x.0;
        let value = drag.start_value as f32 + delta_x / DRAG_MULTIPLIER;

        cx.stop_propagation();
        self.focus(cx);
        self.set_value(value as i32, cx);
        cx.notify();
    }
}

impl Render for IntField {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let focused = self.focus_handle.is_focused(cx);

        div()
            .id(self.id.clone())
            .track_focus(&self.focus_handle)
            .h_flex()
            .w_full()
            .bg(cx.theme().tertriary)
            .px_1()
            .h(INPUT_HEIGHT)
            .border_1()
            .border_color(cx.theme().border)
            .rounded(cx.theme().radius)
            .child(self.value.to_string())
            .hover(|v| v.bg(cx.theme().tertriary_hover))
            .when(focused, |e| {
                e.bg(cx.theme().tertriary_active)
                    .border_color(cx.theme().accent)
            })
            .cursor_ew_resize()
            .on_drag(
                Drag {
                    id: self.id.clone(),
                    start_value: self.value,
                    start_mouse_position: cx.mouse_position(),
                },
                |_, cx| cx.new_view(|_cx| EmptyView),
            )
            .on_drag_move(cx.listener(Self::on_drag_move))
    }
}

struct Drag {
    pub id: ElementId,
    pub start_value: i32,
    pub start_mouse_position: Point<Pixels>,
}

impl FocusableView for IntField {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<InputEvent> for IntField {}

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    ChangeValue(i32),
}
