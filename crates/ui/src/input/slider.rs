use std::ops::RangeInclusive;

use gpui::*;
use prelude::FluentBuilder;

use crate::{bounds_updater, theme::ActiveTheme, StyledExt};

use super::{NumberField, NumberFieldEvent};

pub struct Slider {
    id: ElementId,
    number_field: View<NumberField>,

    slider_bounds: Bounds<Pixels>,

    range: RangeInclusive<f32>,
    step: Option<f32>,
    strict: bool,
}

impl Slider {
    pub fn new(id: impl Into<ElementId>, cx: &mut ViewContext<Self>) -> Self {
        Self {
            id: id.into(),
            number_field: {
                let field = cx.new_view(NumberField::new);

                cx.subscribe(&field, |_, _, event: &NumberFieldEvent, cx| {
                    let NumberFieldEvent::Change(value) = event;
                    cx.emit(SliderEvent::Change(*value));
                })
                .detach();

                field
            },
            range: 0.0..=1.0,
            step: None,
            strict: false,
            slider_bounds: Bounds::default(),
        }
    }

    pub fn value(&self, cx: &AppContext) -> f32 {
        self.number_field.read(cx).value(cx)
    }

    pub fn set_value(&mut self, value: f32, cx: &mut ViewContext<Self>) {
        let stepped_value = self
            .step
            .map_or(value, |step| (value / step).round() * step);
        let strict_value = self.range.start().max(self.range.end().min(stepped_value));
        self.number_field
            .update(cx, |field, cx| field.set_value(strict_value, cx));
        cx.emit(SliderEvent::Change(value));
    }

    pub fn range(&self) -> &RangeInclusive<f32> {
        &self.range
    }

    pub fn set_range(&mut self, range: RangeInclusive<f32>, cx: &mut ViewContext<Self>) {
        self.range = range;
        self.set_value(self.value(cx), cx);
    }

    pub fn step(&self) -> Option<f32> {
        self.step
    }

    pub fn set_step(&mut self, step: Option<f32>, cx: &mut ViewContext<Self>) {
        self.step = step;
        self.set_value(self.value(cx), cx);
    }

    pub fn strict(&self) -> bool {
        self.strict
    }

    pub fn set_strict(&mut self, strict: bool) {
        self.strict = strict;
    }

    pub fn focus(&self, cx: &mut ViewContext<Self>) {
        self.number_field.focus_handle(cx).focus(cx);
    }

    fn on_drag_move(&mut self, event: &DragMoveEvent<Drag>, cx: &mut ViewContext<Self>) {
        let drag = event.drag(cx);

        if drag.id != self.id {
            return;
        }

        let relative =
            (cx.mouse_position().x - self.slider_bounds.left()) / self.slider_bounds.size.width;
        let value = self.range.start() + (self.range.end() - self.range.start()) * relative;

        self.set_value(value, cx);
        cx.notify();
    }
}

impl Render for Slider {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let focus_handle = self.number_field.focus_handle(cx);
        let focused = focus_handle.is_focused(cx);
        let number_field = self.number_field.clone();

        let diff = *self.range.end() - *self.range.start();
        let relative_value = (self.value(cx) / diff).clamp(0.0, 1.0);
        let slider_background = div()
            .h_full()
            .w(relative(relative_value))
            .bg(cx.theme().tertriary);

        let slider = div()
            .track_focus(&focus_handle)
            .id(self.id.clone())
            .w_2_3()
            .h_full()
            .bg(cx.theme().primary)
            .border_1()
            .border_color(cx.theme().border)
            .when(focused, |s| s.border_color(cx.theme().accent))
            .rounded(cx.theme().radius)
            .cursor_ew_resize()
            .child(slider_background)
            .child(bounds_updater(cx.view().clone(), |this, bounds, _cx| {
                this.slider_bounds = bounds
            }))
            .on_drag(
                Drag {
                    id: self.id.clone(),
                },
                |_, cx| cx.new_view(|_cx| EmptyView),
            )
            .on_drag_move(cx.listener(Self::on_drag_move));

        div()
            .h_flex()
            .gap_1()
            .w_full()
            .h(cx.theme().input_height)
            .child(slider)
            .child(div().w_1_3().h_full().child(number_field))
    }
}

struct Drag {
    pub id: ElementId,
}

impl EventEmitter<SliderEvent> for Slider {}

#[derive(Debug, Clone, Copy)]
pub enum SliderEvent {
    Change(f32),
}
