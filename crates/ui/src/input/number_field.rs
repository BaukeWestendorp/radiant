use std::ops::RangeInclusive;

use gpui::*;
use prelude::FluentBuilder;

use crate::{theme::ActiveTheme, z_stack, StyledExt};

const DRAG_MULTIPLIER: f32 = 0.5;

pub struct NumberField {
    id: ElementId,
    focus_handle: FocusHandle,
    value: f32,
    range: Option<RangeInclusive<f32>>,
    step: Option<f32>,
    strict: bool,

    bounds: Bounds<Pixels>,
}

impl NumberField {
    pub fn new(id: impl Into<ElementId>, cx: &mut ViewContext<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        Self {
            id: id.into(),
            focus_handle: focus_handle.clone(),
            value: 0.0,
            range: None,
            step: None,
            strict: false,
            bounds: Bounds::default(),
        }
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn set_value(&mut self, value: f32, cx: &mut ViewContext<Self>) {
        let stepped_value = self
            .step
            .map_or(value, |step| (value / step).round() * step);
        let strict_value = self.range.as_ref().map_or(stepped_value, |range| {
            range.start().max(range.end().min(stepped_value))
        });
        self.value = strict_value;
        cx.emit(NumberFieldEvent::ChangeValue(value));
    }

    pub fn range(&self) -> Option<&RangeInclusive<f32>> {
        self.range.as_ref()
    }

    pub fn set_range(&mut self, range: RangeInclusive<f32>) {
        self.range = Some(range);
    }

    pub fn step(&self) -> Option<f32> {
        self.step
    }

    pub fn set_step(&mut self, step: f32) {
        self.step = Some(step);
    }

    pub fn strict(&self) -> bool {
        self.strict
    }

    pub fn set_strict(&mut self, strict: bool) {
        self.strict = strict;
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
        let value = match &self.range {
            Some(range) => {
                let relative =
                    (cx.mouse_position().x - self.bounds.left()) / self.bounds.size.width;
                range.start() + (range.end() - range.start()) * relative
            }
            None => drag.start_value + delta_x * DRAG_MULTIPLIER,
        };

        self.set_value(value, cx);
        cx.notify();
    }
}

impl Render for NumberField {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let focused = self.focus_handle.is_focused(cx);

        let bounds_updater = div()
            .size_full()
            .child({
                let view = cx.view().clone();
                canvas(
                    move |bounds, cx| view.update(cx, |view, _cx| view.bounds = bounds),
                    |_, _, _| {},
                )
                .absolute()
                .size_full()
            })
            .into_any_element();

        let background =
            div()
                .h_full()
                .rounded(cx.theme().radius)
                .when_some(self.range(), |e, range| {
                    let diff = *range.end() - *range.start();
                    let relative_value = self.value() / diff;
                    e.w(relative(relative_value)).bg(cx.theme().tertriary)
                });

        let field = div()
            .id(self.id.clone())
            .h_flex()
            .size_full()
            .px_1()
            .border_color(cx.theme().border)
            .border_1()
            .when(focused, |e| e.border_color(cx.theme().accent))
            .rounded(cx.theme().radius)
            .child(self.value.to_string())
            .child(bounds_updater)
            .on_drag(
                Drag {
                    id: self.id.clone(),
                    start_value: self.value,
                    start_mouse_position: cx.mouse_position(),
                },
                |_, cx| cx.new_view(|_cx| EmptyView),
            )
            .on_drag_move(cx.listener(Self::on_drag_move));

        z_stack([background.into_any_element(), field.into_any_element()])
            .track_focus(&self.focus_handle)
            .h(cx.theme().input_height)
            .w_full()
            .bg(cx.theme().primary)
            .hover(|e| e.bg(cx.theme().primary_hover))
            .rounded(cx.theme().radius)
            .overflow_hidden()
            .cursor_ew_resize()
    }
}

struct Drag {
    pub id: ElementId,
    pub start_value: f32,
    pub start_mouse_position: Point<Pixels>,
}

impl FocusableView for NumberField {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<NumberFieldEvent> for NumberField {}

#[derive(Debug, Clone, Copy)]
pub enum NumberFieldEvent {
    ChangeValue(f32),
}
