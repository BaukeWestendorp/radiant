use gpui::{
    canvas, div, px, rgb, rgba, Bounds, DragMoveEvent, InteractiveElement, IntoElement, Model,
    ParentElement, Pixels, Render, SharedString, StatefulInteractiveElement, Styled, ViewContext,
    VisualContext, WindowContext,
};

pub struct Slider<D: SliderDelegate> {
    delegate: D,
    id: SharedString,
    bounds: Bounds<Pixels>,
    value: Model<f32>,
}

impl<D: SliderDelegate + 'static> Slider<D> {
    pub fn new(id: &str, delegate: D, value: Model<f32>) -> Self {
        Self {
            delegate,
            id: id.to_string().into(),
            bounds: Bounds::default(),
            value,
        }
    }

    fn handle_drag_thumb(
        &mut self,
        event: &DragMoveEvent<DraggedSliderThumb>,
        cx: &mut ViewContext<Self>,
    ) {
        if event.drag(cx).id != self.id {
            return;
        }

        let mouse_y = cx.mouse_position().y - self.delegate.thumb_height() / 2.0;
        let h = mouse_y - self.bounds.top();
        self.value.update(cx, |value, cx| {
            *value = (1.0 - (h / self.movement_height())).clamp(0.0, 1.0);
            cx.notify();
        });
    }

    fn movement_height(&self) -> Pixels {
        self.height() - self.delegate.thumb_height()
    }

    fn height(&self) -> Pixels {
        self.bounds.size.height
    }

    fn calculate_thumb_position(&self, cx: &mut WindowContext) -> Pixels {
        px(*self.value.read(cx)) * self.movement_height()
    }
}

impl<D: SliderDelegate + 'static> Render for Slider<D> {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let thumb_position = self.calculate_thumb_position(cx);

        div()
            .id(self.id.clone())
            .relative()
            .size_full()
            // Background
            .child(
                div()
                    .absolute()
                    .size_full()
                    .child(self.delegate.render_background(cx)),
            )
            // Thumb
            .child(
                div()
                    .absolute()
                    .bottom(thumb_position)
                    .bg(rgba(0x00ff0080))
                    .child(self.delegate.render_thumb(cx)),
            )
            // Bounds reader
            .child(
                canvas({
                    let this = cx.view().clone();
                    move |bounds, cx| {
                        this.update(cx, |this, _cx| {
                            this.bounds = *bounds;
                        })
                    }
                })
                .absolute()
                .size_full(),
            )
            .on_drag(
                DraggedSliderThumb {
                    id: self.id.clone(),
                },
                |slider, cx| {
                    cx.stop_propagation();
                    cx.new_view(|_cx| slider.clone())
                },
            )
            .on_drag_move(cx.listener(Self::handle_drag_thumb))
    }
}

#[derive(Debug, Clone, Render)]
struct DraggedSliderThumb {
    id: SharedString,
}

pub trait SliderDelegate: Sized {
    fn render_background(&self, _cx: &mut ViewContext<Slider<Self>>) -> impl IntoElement {
        div().bg(rgb(0x303030))
    }

    fn render_thumb(&self, _cx: &mut ViewContext<Slider<Self>>) -> impl IntoElement {
        div()
            .bg(rgb(0x505050))
            .h(self.thumb_height())
            .w_full()
            .hover(|this| this.bg(rgb(0x606060)))
    }

    fn thumb_height(&self) -> Pixels {
        px(16.0)
    }
}
