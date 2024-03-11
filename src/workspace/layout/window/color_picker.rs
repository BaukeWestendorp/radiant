use gpui::{IntoElement, Model, ParentElement, Render, Styled, View, ViewContext, VisualContext};

use crate::show::Show;
use crate::ui::grid_div;

use super::{show_window, Window};

pub struct ColorPickerWindow {
    window_id: usize,
    show: Model<Show>,
}

impl ColorPickerWindow {
    pub fn build(window_id: usize, show: Model<Show>, cx: &mut ViewContext<Window>) -> View<Self> {
        cx.new_view(|_cx| Self { window_id, show })
    }
}

impl Render for ColorPickerWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let show_window = show_window(&self.show, self.window_id, cx).clone();

        grid_div(show_window.bounds.size, None)
            .size_full()
            .child("Hello, window!")
    }
}
