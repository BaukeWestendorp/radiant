use gpui::{Hsla, ScrollHandle, Window, div, prelude::*};
use ui::{ActiveTheme, InteractiveColor};

pub struct ColorTab {
    scroll_handle: ScrollHandle,
}

impl ColorTab {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { scroll_handle: ScrollHandle::new() }
    }
}

impl Render for ColorTab {
    fn render(&mut self, _w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let color_swatch = |label, color| {
            div().child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(color)
                    .size_10()
                    .text_xs()
                    .child(label),
            )
        };

        let color_swatches = |label, color: Hsla| {
            div()
                .flex()
                .items_center()
                .gap_1()
                .child(div().w_40().child(label))
                .child(color_swatch("", color))
                .child(color_swatch("muted", color.muted()))
                .child(color_swatch("hover", color.hovered()))
                .child(color_swatch("active", color.active()))
        };

        let bg_colors = div()
            .flex()
            .flex_col()
            .gap_1()
            .child(color_swatches("bg_primary", cx.theme().colors.bg_primary))
            .child(color_swatches("bg_secondary", cx.theme().colors.bg_secondary))
            .child(color_swatches("bg_tertiary", cx.theme().colors.bg_tertiary))
            .child(color_swatches("bg_selected", cx.theme().colors.bg_selected))
            .child(color_swatches("bg_selected_bright", cx.theme().colors.bg_selected_bright))
            .child(color_swatches("bg_focused", cx.theme().colors.bg_focused));

        let text_colors = div().child(color_swatches("text", cx.theme().colors.text));

        let border_colors = div()
            .flex()
            .flex_col()
            .gap_1()
            .child(color_swatches("border", cx.theme().colors.border))
            .child(color_swatches("border_selected", cx.theme().colors.border_selected))
            .child(color_swatches("border_focused", cx.theme().colors.border_focused));

        let header_colors = div()
            .flex()
            .flex_col()
            .gap_1()
            .child(color_swatches("header_background", cx.theme().colors.header_background))
            .child(color_swatches("header_border", cx.theme().colors.header_border));

        let accent_colors = div()
            .flex()
            .flex_col()
            .gap_1()
            .child(color_swatches("accent", cx.theme().colors.accent))
            .child(color_swatches("highlight", cx.theme().colors.highlight))
            .child(color_swatches("cursor", cx.theme().colors.cursor));

        let misc_colors =
            div().flex().flex_col().gap_1().child(color_swatches("grid", cx.theme().colors.grid));

        div()
            .id("colors-tab")
            .track_scroll(&self.scroll_handle)
            .overflow_y_scroll()
            .size_full()
            .p_2()
            .child(ui::section("Background Colors").child(bg_colors))
            .child(ui::section("Text Colors").child(text_colors))
            .child(ui::section("Border Colors").child(border_colors))
            .child(ui::section("Header Colors").child(header_colors))
            .child(ui::section("Accent Colors").child(accent_colors))
            .child(ui::section("Misc Colors").child(misc_colors))
    }
}
