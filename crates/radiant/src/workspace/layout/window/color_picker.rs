use gpui::{
    div, rgb, Context, IntoElement, Model, ParentElement, Styled, View, ViewContext, VisualContext,
    WindowContext,
};

use crate::ui::slider::{Slider, SliderDelegate};

use super::{Window, WindowDelegate};

pub struct ColorPickerWindowDelegate {
    red_slider: View<Slider<ColorComponentSliderDelegate>>,
    red: Model<f32>,

    green_slider: View<Slider<ColorComponentSliderDelegate>>,
    green: Model<f32>,

    blue_slider: View<Slider<ColorComponentSliderDelegate>>,
    blue: Model<f32>,
}

impl ColorPickerWindowDelegate {
    pub fn new(cx: &mut WindowContext) -> Self {
        let red = cx.new_model(|_cx| 0.0);
        let red_slider = cx.new_view(|_cx| {
            Slider::new(
                "red_slider",
                ColorComponentSliderDelegate {
                    component: ColorComponent::Red,
                },
                red.clone(),
            )
        });

        let green = cx.new_model(|_cx| 0.0);
        let green_slider = cx.new_view(|_cx| {
            Slider::new(
                "green_slider",
                ColorComponentSliderDelegate {
                    component: ColorComponent::Green,
                },
                green.clone(),
            )
        });

        let blue = cx.new_model(|_cx| 0.0);
        let blue_slider = cx.new_view(|_cx| {
            Slider::new(
                "blue_slider",
                ColorComponentSliderDelegate {
                    component: ColorComponent::Blue,
                },
                blue.clone(),
            )
        });

        Self {
            red_slider,
            red,
            green_slider,
            green,
            blue_slider,
            blue,
        }
    }
}

impl WindowDelegate for ColorPickerWindowDelegate {
    fn title(&self) -> String {
        "ColorPicker".to_string()
    }

    fn render_content(&self, cx: &mut ViewContext<Window<Self>>) -> impl IntoElement {
        div()
            .size_full()
            .p_4()
            .flex()
            .gap_2()
            .child(div().w_8().h_full().child(self.red_slider.clone()))
            .child(div().w_8().h_full().child(self.green_slider.clone()))
            .child(div().w_8().h_full().child(self.blue_slider.clone()))
            .child(format!(
                "R: {:.2}, G: {:.2}, B: {:.2}",
                self.red.read(cx),
                self.green.read(cx),
                self.blue.read(cx)
            ))
    }
}

struct ColorComponentSliderDelegate {
    pub component: ColorComponent,
}

impl SliderDelegate for ColorComponentSliderDelegate {
    fn render_background(&self, _cx: &mut ViewContext<Slider<Self>>) -> impl IntoElement {
        let background = match self.component {
            ColorComponent::Red => rgb(0xff0000),
            ColorComponent::Green => rgb(0x00ff00),
            ColorComponent::Blue => rgb(0x0000ff),
        };

        div().bg(background).size_full()
    }
}

enum ColorComponent {
    Red,
    Green,
    Blue,
}
