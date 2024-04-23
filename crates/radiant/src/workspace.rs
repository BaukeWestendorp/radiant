use gpui::{
    div, Context, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use crate::ui::slider::Slider;

pub struct Workspace {
    slider1: View<Slider>,
    slider2: View<Slider>,
    slider3: View<Slider>,
}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
            slider1: cx.new_view(|cx| Slider::new("slider1", cx.new_model(|_cx| 0.3))),
            slider2: cx.new_view(|cx| Slider::new("slider2", cx.new_model(|_cx| 0.5))),
            slider3: cx.new_view(|cx| Slider::new("slider3", cx.new_model(|_cx| 0.8))),
        })
    }
}

impl Render for Workspace {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let sliders = [
            div()
                .w_10()
                .h_56()
                .bg(gpui::blue())
                .child(self.slider1.clone()),
            div()
                .w_10()
                .h_56()
                .bg(gpui::blue())
                .child(self.slider2.clone()),
            div()
                .w_10()
                .h_56()
                .bg(gpui::blue())
                .child(self.slider3.clone()),
        ];
        div()
            .text_color(gpui::white())
            .flex()
            .gap_2()
            .children(sliders)
    }
}
