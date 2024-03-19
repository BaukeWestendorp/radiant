use gpui::{div, IntoElement, Model, ParentElement, Rgba, Styled, WindowContext};

use crate::{
    color,
    show::{presets::Preset, Show},
    workspace::layout::LayoutBounds,
};

use super::PoolWindowDelegate;

pub struct ColorPoolWindowDelegate {
    scroll_offset: usize,
    bounds: LayoutBounds,
    show: Model<Show>,
}

impl ColorPoolWindowDelegate {
    pub fn new(scroll_offset: usize, bounds: LayoutBounds, show: Model<Show>) -> Self {
        Self {
            scroll_offset,
            bounds,
            show,
        }
    }
}

impl PoolWindowDelegate for ColorPoolWindowDelegate {
    fn label(&self) -> String {
        "Colors".to_string()
    }

    fn bounds(&self, _cx: &mut WindowContext) -> &LayoutBounds {
        &self.bounds
    }

    fn scroll_offset(&self, _cx: &mut WindowContext) -> usize {
        self.scroll_offset
    }

    fn render_item_for_id(&self, id: usize, cx: &mut WindowContext) -> Option<impl IntoElement> {
        let color = self.show.read(cx).presets.color_preset(id);

        match color {
            Some(color) => {
                let rgba: Rgba = color.color.clone().into();
                let label = color.label().to_string();

                Some(
                    div()
                        .size_full()
                        .flex()
                        .flex_col_reverse()
                        .bg(color::opacify(rgba, 0.8))
                        .rounded_md()
                        .child(
                            div()
                                .bg(rgba)
                                .h_1_3()
                                .border_t()
                                .rounded_b_md()
                                .border_color(color::darken(rgba, 0.4)),
                        )
                        .child(div().flex().justify_center().text_xs().child(label)),
                )
            }
            None => None,
        }
    }
}
