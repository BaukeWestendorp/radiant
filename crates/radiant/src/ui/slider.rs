use gpui::{
    canvas, div, prelude::*, px, AvailableSpace, Bounds, DragMoveEvent, ElementId, Model, Pixels,
    View, ViewContext, WindowContext,
};

use crate::theme::THEME;

pub struct Slider {
    id: ElementId,
    value: Model<f32>,
    markers: Option<Vec<f32>>,
}

impl Slider {
    pub fn build(
        id: impl Into<ElementId>,
        value: Model<f32>,
        markers: Option<Model<Vec<f32>>>,
        cx: &mut WindowContext,
    ) -> View<Self> {
        cx.new_view(|cx| {
            markers.clone().map(|markers| {
                cx.observe(&markers, |this: &mut Self, markers, cx| {
                    this.markers = Some(markers.read(cx).clone());
                    cx.notify();
                })
                .detach();
            });

            Self {
                id: id.into(),
                value,
                markers: markers.map(|markers| markers.read(cx).clone()),
            }
        })
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
                        let markers = this
                            .markers
                            .as_ref()
                            .map(|markers| {
                                markers
                                    .iter()
                                    .map(|marker| {
                                        div()
                                            .absolute()
                                            .w_full()
                                            .bottom(bounds.size.height * px(*marker))
                                            .border_t_4()
                                            .border_color(THEME.accent)
                                    })
                                    .collect()
                            })
                            .unwrap_or_else(|| vec![]);

                        div()
                            .id(this.id.clone())
                            .relative()
                            .size_full()
                            .border()
                            .border_color(THEME.border)
                            .rounded_md()
                            .child(
                                div()
                                    .absolute()
                                    .w_full()
                                    .h(h)
                                    .rounded_md()
                                    .bottom_0()
                                    .bg(THEME.fill_selected),
                            )
                            .children(markers)
                            .on_drag(
                                DraggedSlider {
                                    id: this.id.clone(),
                                },
                                |dragged_slider, cx| cx.new_view(|_cx| dragged_slider.clone()),
                            )
                            .on_drag_move(cx.listener(move |this, event, cx| {
                                this.handle_drag(&bounds, event, cx)
                            }))
                            .into_any_element()
                    });

                    slider.layout(bounds.origin, bounds.size.map(AvailableSpace::Definite), cx);

                    slider
                }
            },
            |_bounds, mut element, cx| element.paint(cx),
        )
        .h_full()
    }
}

#[derive(Debug, Clone, Render)]
struct DraggedSlider {
    id: ElementId,
}
