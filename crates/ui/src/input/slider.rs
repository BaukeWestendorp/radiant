use std::ops::RangeInclusive;

use gpui::*;

use crate::{bounds_updater, theme::ActiveTheme, ContainerKind, InteractiveContainer, StyledExt};

use super::{NumberField, NumberFieldEvent};

pub struct Slider {
    id: ElementId,
    number_field: View<NumberField>,

    slider_bounds: Bounds<Pixels>,

    range: RangeInclusive<f64>,
    step: Option<f64>,
    strict: bool,
}

impl Slider {
    pub fn new(id: impl Into<ElementId>, value: f64, cx: &mut ViewContext<Self>) -> Self {
        let id: ElementId = id.into();
        Self {
            id: id.clone(),
            number_field: {
                let field = cx.new_view(|cx| NumberField::new(id, value, cx));

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

    pub fn value(&self, cx: &AppContext) -> f64 {
        self.number_field.read(cx).value(cx)
    }

    pub fn set_value(&mut self, value: f64, cx: &mut ViewContext<Self>) {
        let stepped_value = self
            .step
            .map_or(value, |step| (value / step).round() * step);
        let strict_value = self.range.start().max(self.range.end().min(stepped_value));
        self.number_field
            .update(cx, |field, cx| field.set_value(strict_value, cx));
        cx.emit(SliderEvent::Change(value));
    }

    pub fn range(&self) -> &RangeInclusive<f64> {
        &self.range
    }

    pub fn set_range(&mut self, range: RangeInclusive<f64>) {
        self.range = range;
    }

    pub fn step(&self) -> Option<f64> {
        self.step
    }

    pub fn set_step(&mut self, step: Option<f64>) {
        self.step = step;
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
        let value = self.range.start() + (self.range.end() - self.range.start()) * relative as f64;

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
        let bar = div()
            .h_full()
            .w(relative(relative_value as f32))
            .bg(cx.theme().accent.opacity(0.2));

        let slider = InteractiveContainer::new(
            ContainerKind::Custom {
                bg: cx.theme().background,
                border_color: ContainerKind::Element.border_color(cx),
            },
            self.id.clone(),
            false,
            focused,
        )
        .w_2_3()
        .h_full()
        .cursor_ew_resize()
        .child(bar)
        .child(bounds_updater(cx.view().clone(), |this, bounds, _cx| {
            this.slider_bounds = bounds
        }))
        .on_drag(
            Drag {
                id: self.id.clone(),
            },
            |_, _point, cx| cx.new_view(|_cx| EmptyView),
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
    Change(f64),
}
