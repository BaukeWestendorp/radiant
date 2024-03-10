use gpui::{div, IntoElement, ParentElement, Render, Rgba, Styled, ViewContext};

use crate::color;
use crate::show::presets::{ColorPreset, Preset};

pub struct PoolItem {
    id: usize,
}

impl PoolItem {
    pub fn build(id: usize) -> Self {
        Self { id }
    }

    fn render_color_pool_item(&self, color_preset: &ColorPreset) -> impl IntoElement {
        let color: Rgba = color_preset.color.clone().into();
        let label = color_preset.label().to_string();

        div()
            .size_full()
            .flex()
            .flex_col_reverse()
            .bg(color::opacify(color, 0.8))
            .rounded_md()
            .child(
                div()
                    .bg(color)
                    .h_1_3()
                    .border_t()
                    .rounded_b_md()
                    .border_color(color::darken(color, 0.4)),
            )
            .child(div().flex().justify_center().text_xs().child(label))
    }

    fn render_empty_pool_item(&self) -> impl IntoElement {
        div().size_full()
    }
}

impl Render for PoolItem {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        // let pool_color = ();

        // let mut has_content = false;
        // let content = match &pool_window.kind {
        //     PoolWindowKind::Color => {
        //         let color_preset = ();

        //         match color_preset {
        //             Some(color_preset) => {
        //                 has_content = true;
        //                 self.render_color_pool_item(color_preset).into_any_element()
        //             }
        //             None => self.render_empty_pool_item().into_any_element(),
        //         }
        //     }
        // };

        // div()
        //     .bg(rgb(0x202020))
        //     .border_color(color::darken(pool_color, 0.7))
        //     .border_1()
        //     .rounded_md()
        //     .size_full()
        //     .relative()
        //     .child(div().size_full().absolute().inset_0().child(content))
        //     .child(
        //         div()
        //             .absolute()
        //             .size_full()
        //             .text_sm()
        //             .text_color(rgb(0x808080))
        //             .when(has_content, |this| this.text_color(rgb(0xffffff)))
        //             .pl(px(4.0))
        //             .child(format!("{}", self.id)),
        //     )
        div()
        // TODO:
    }
}
