use gpui::{
    canvas, div, px, Bounds, DragMoveEvent, InteractiveElement, IntoElement, Model, ParentElement,
    Pixels, Render, SharedString, StatefulInteractiveElement, Styled, ViewContext, VisualContext,
    WindowContext,
};

use crate::{
    button::{Button, ButtonStyle},
    container::Container,
};

pub struct Slider<D: SliderDelegate> {
    pub delegate: D,
    id: SharedString,
    pub value: Model<f32>,
}

impl<D: SliderDelegate + 'static> Slider<D> {
    pub fn new(id: &str, delegate: D, value: Model<f32>) -> Self {
        Self {
            delegate,
            id: id.to_string().into(),
            value,
        }
    }

    fn handle_drag_thumb(
        &mut self,
        bounds: &Bounds<Pixels>,
        event: &DragMoveEvent<DraggedSliderThumb>,
        cx: &mut ViewContext<Self>,
    ) {
        if event.drag(cx).id != self.id {
            return;
        }

        let mouse_y = cx.mouse_position().y - self.delegate.thumb_height() / 2.0;
        let h = mouse_y - bounds.top();
        self.value.update(cx, |value, cx| {
            *value = (1.0 - (h / self.movement_height(bounds))).clamp(0.0, 1.0);
            cx.notify();
        });
    }

    fn movement_height(&self, bounds: &Bounds<Pixels>) -> Pixels {
        bounds.size.height - self.delegate.thumb_height()
    }

    fn calculate_thumb_position(&self, bounds: &Bounds<Pixels>, cx: &mut WindowContext) -> Pixels {
        px(*self.value.read(cx)) * self.movement_height(bounds)
    }
}

impl<D: SliderDelegate + 'static> Render for Slider<D> {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        canvas(
            {
                let this = cx.view().clone();
                move |bounds, cx| {
                    let mut slider = this.update(cx, |this, cx| {
                        div()
                            .id(this.id.clone())
                            .size_full()
                            .relative()
                            // Background
                            .child(
                                div()
                                    .absolute()
                                    .size_full()
                                    .child(this.delegate.render_background(cx)),
                            )
                            // Thumb
                            .child(
                                div()
                                    .absolute()
                                    .bottom(this.calculate_thumb_position(&bounds, cx))
                                    .child(this.delegate.render_thumb(cx)),
                            )
                            .on_drag(
                                DraggedSliderThumb {
                                    id: this.id.clone(),
                                },
                                |slider, cx| {
                                    cx.stop_propagation();
                                    cx.new_view(|_cx| slider.clone())
                                },
                            )
                            .on_drag_move(cx.listener(move |this, event, cx| {
                                this.handle_drag_thumb(&bounds, event, cx)
                            }))
                            .into_any_element()
                    });
                    slider.layout(bounds.origin, bounds.size.into(), cx);
                    slider
                }
            },
            |_bounds, mut element, cx| element.paint(cx),
        )
        .size_full()
    }
}

#[derive(Debug, Clone, Render)]
struct DraggedSliderThumb {
    id: SharedString,
}

pub trait SliderDelegate: Sized {
    fn render_background(&self, cx: &mut ViewContext<Slider<Self>>) -> impl IntoElement {
        Container::new(cx).size_full()
    }

    fn render_thumb(&self, cx: &mut ViewContext<Slider<Self>>) -> impl IntoElement {
        Button::new(ButtonStyle::Secondary, "thumb", cx)
            .h(self.thumb_height())
            .w_full()
    }

    fn thumb_height(&self) -> Pixels {
        px(16.0)
    }
}
