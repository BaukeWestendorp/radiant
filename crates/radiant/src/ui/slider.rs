use gpui::{
    canvas, div, prelude::*, px, AvailableSpace, Bounds, DragMoveEvent, ElementId, Model, Pixels,
    ViewContext,
};

pub struct Slider {
    id: ElementId,
    value: Model<f32>,
}

impl Slider {
    pub fn new(id: impl Into<ElementId>, value: Model<f32>) -> Self {
        Self {
            id: id.into(),
            value,
        }
    }

    fn handle_drag(
        &mut self,
        bounds: &Bounds<Pixels>,
        event: &DragMoveEvent<DraggedSlider>,
        cx: &mut ViewContext<Self>,
    ) {
        if event.drag(cx).id != self.id {
            return;
        }

        let mouse_y = cx.mouse_position().y;
        let h = mouse_y - bounds.top();
        self.value.update(cx, |value, cx| {
            *value = (1.0 - (h / bounds.size.height)).clamp(0.0, 1.0);
            cx.notify();
        });
    }
}

impl Render for Slider {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let value = *self.value.read(cx);

        canvas(
            {
                let this = cx.view().clone();
                move |bounds, cx| {
                    let h = bounds.size.height * px(value);

                    let mut slider = this.update(cx, |this, cx| {
                        div()
                            .id(this.id.clone())
                            .size_full()
                            .relative()
                            .on_drag(
                                DraggedSlider {
                                    id: this.id.clone(),
                                },
                                |dragged_slider, cx| cx.new_view(|_cx| dragged_slider.clone()),
                            )
                            .on_drag_move(cx.listener(move |this, event, cx| {
                                this.handle_drag(&bounds, event, cx)
                            }))
                            .child(
                                div()
                                    .absolute()
                                    .w_full()
                                    .h(h)
                                    .rounded_md()
                                    .bottom_0()
                                    .bg(gpui::green()),
                            )
                            .into_any_element()
                    });

                    slider.layout(bounds.origin, bounds.size.map(AvailableSpace::Definite), cx);

                    slider
                }
            },
            |_bounds, mut element, cx| element.paint(cx),
        )
        .border()
        .border_color(gpui::white())
        .rounded_md()
        .h_full()
    }
}

#[derive(Debug, Clone, Render)]
struct DraggedSlider {
    id: ElementId,
}
