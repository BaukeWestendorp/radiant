use gpui::prelude::*;
use gpui::{App, Window, div, rgba};

use rd_ui::{ActiveTheme, HslaExt, section};

pub struct ThemePreview {}

impl ThemePreview {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

fn row(label: impl Into<String>, color: gpui::Hsla, fg: gpui::Hsla, cx: &App) -> impl IntoElement {
    let label = label.into();

    div()
        .flex()
        .items_center()
        .gap_2()
        .child(
            div()
                .w_12()
                .h_6()
                .rounded(cx.theme().radius)
                .border_1()
                .border_color(rgba(0xFFFFFF1F))
                .bg(color),
        )
        .child(div().text_color(fg).child(label))
}

impl Render for ThemePreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = cx.theme();

        let content = div()
            .p_2()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                section("Backgrounds").child(
                    div()
                        .flex_col()
                        .gap_2()
                        .child(row("bg_primary", t.bg_primary, t.fg_primary, cx))
                        .child(row("bg_secondary", t.bg_secondary, t.fg_primary, cx))
                        .child(row("bg_tertiary", t.bg_tertiary, t.fg_primary, cx))
                        .child(row("bg_selected", t.bg_selected, t.fg_primary, cx))
                        .child(row("bg_selected_extra", t.bg_selected_extra, t.fg_primary, cx))
                        .child(row("bg_table", t.bg_table, t.fg_primary, cx))
                        .child(row("bg_table_odd", t.bg_table_odd, t.fg_primary, cx))
                        .child(row("bg_tile_header", t.bg_tile_header, t.fg_primary, cx)),
                ),
            )
            .child(
                section("Foregrounds").child(
                    div()
                        .flex_col()
                        .gap_2()
                        .child(row("fg_primary", t.fg_primary, t.fg_primary, cx))
                        .child(row("fg_secondary", t.fg_secondary, t.fg_secondary, cx))
                        .child(row("fg_tertiary", t.fg_tertiary, t.fg_tertiary, cx))
                        .child(row("fg_selected", t.fg_selected, t.fg_selected, cx))
                        .child(row("fg_tile_header", t.fg_tile_header, t.fg_tile_header, cx)),
                ),
            )
            .child(
                section("Borders").child(
                    div()
                        .flex_col()
                        .gap_2()
                        .child(row("border_primary", t.border_primary, t.fg_primary, cx))
                        .child(row("border_secondary", t.border_secondary, t.fg_primary, cx))
                        .child(row("border_tertiary", t.border_tertiary, t.fg_primary, cx))
                        .child(row("border_selected", t.border_selected, t.fg_primary, cx))
                        .child(row("border_tile_header", t.border_tile_header, t.fg_primary, cx)),
                ),
            )
            .child(
                section("Accents & States").child(
                    div()
                        .flex_col()
                        .gap_2()
                        .child(row("accent", t.accent, t.fg_primary, cx))
                        .child(
                            div()
                                .flex()
                                .gap_3()
                                .flex_wrap()
                                .child(row("accent (hover)", t.accent.hover(), t.fg_primary, cx))
                                .child(row("accent (active)", t.accent.active(), t.fg_primary, cx))
                                .child(row(
                                    "accent (disabled)",
                                    t.accent.disabled(),
                                    t.fg_primary,
                                    cx,
                                )),
                        )
                        .child(row("warning", t.indicate.warning, t.fg_primary, cx))
                        .child(row("error", t.indicate.error, t.fg_primary, cx))
                        .child(row("success", t.indicate.success, t.fg_primary, cx)),
                ),
            )
            .child(
                section("Title bar").child(
                    div()
                        .flex_col()
                        .gap_2()
                        .child(row("title_bar", t.title_bar, t.fg_primary, cx))
                        .child(row("title_bar_border", t.title_bar_border, t.fg_primary, cx)),
                ),
            );

        div().id("theme-preview").size_full().overflow_scroll().child(content)
    }
}
