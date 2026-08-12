use gpui::ScrollHandle;
use rd_ui::{
    ActiveTheme, StyledExt, Theme,
    comp::{Section, sub},
    gpui::{App, Hsla, Window, div, prelude::*},
    h_flex, v_flex,
};

use crate::layout::PreviewStory;

pub struct ThemePreview {
    scroll_handle: ScrollHandle,
}

impl ThemePreview {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { scroll_handle: ScrollHandle::new() }
    }
}

impl Render for ThemePreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let light_theme = Theme::light();
        let dark_theme = Theme::dark();

        // FIXME: Scrolling is a bit broken
        v_flex()
            .id("theme_preview")
            .track_scroll(&self.scroll_handle)
            .p_2()
            .overflow_y_scroll()
            .size_full()
            .child(PreviewStory::new(
                "Built-in themes",
                h_flex()
                    .p_2()
                    .gap_2()
                    .items_start()
                    .child(
                        Section::new(
                            "Light",
                            v_flex()
                                .gap_2()
                                .p_2()
                                .child(theme_metrics(&light_theme, cx))
                                .child(theme_palette(&light_theme, cx)),
                        )
                        .w_full(),
                    )
                    .child(
                        Section::new(
                            "Dark",
                            v_flex()
                                .gap_2()
                                .p_2()
                                .child(theme_metrics(&dark_theme, cx))
                                .child(theme_palette(&dark_theme, cx)),
                        )
                        .w_full(),
                    ),
            ))
    }
}

fn theme_metrics(theme: &Theme, cx: &App) -> impl IntoElement {
    v_flex()
        .w_full()
        .gap_1()
        .child(metric_row("Font size", format!("{:?}", theme.font_size), cx))
        .child(metric_row("Radius", format!("{:?}", theme.radius), cx))
        .child(metric_row("Cursor width", format!("{:?}", theme.cursor_width), cx))
        .child(metric_row(
            h_flex().gap_2().child("Button depression").child(sub("poor button :(", cx)),
            format!("{:?}", theme.button_depression),
            cx,
        ))
        .child(metric_row("Shadow", theme.shadow.to_string(), cx))
}

fn theme_palette(theme: &Theme, cx: &App) -> impl IntoElement {
    v_flex()
        .w_full()
        .gap_3()
        .child(color_group(
            "Background",
            vec![
                ("Primary", theme.bg_primary),
                ("Secondary", theme.bg_secondary),
                ("Tertiary", theme.bg_tertiary),
                ("Selected", theme.bg_selected),
                ("Focused", theme.bg_focused),
                ("Table", theme.bg_table),
                ("Table odd", theme.bg_table_odd),
                ("Tile header", theme.bg_tile_header),
            ],
            cx,
        ))
        .child(color_group(
            "Foreground",
            vec![
                ("Primary", theme.fg_primary),
                ("Secondary", theme.fg_secondary),
                ("Tertiary", theme.fg_tertiary),
                ("Selected", theme.fg_selected),
                ("Focused", theme.fg_focused),
                ("Tile header", theme.fg_tile_header),
                ("Contrast", theme.contrast),
            ],
            cx,
        ))
        .child(color_group(
            "Border",
            vec![
                ("Primary", theme.border_primary),
                ("Secondary", theme.border_secondary),
                ("Tertiary", theme.border_tertiary),
                ("Selected", theme.border_selected),
                ("Focused", theme.border_focused),
                ("Tile header", theme.border_tile_header),
            ],
            cx,
        ))
        .child(color_group(
            "Feedback",
            vec![
                ("Accent", theme.accent),
                ("Danger", theme.indicate.danger),
                ("Warning", theme.indicate.warning),
                ("Info", theme.indicate.info),
                ("Success", theme.indicate.success),
                ("Title bar", theme.title_bar),
                ("Title bar border", theme.title_bar_border),
            ],
            cx,
        ))
}

fn color_group(
    title: &'static str,
    colors: Vec<(&'static str, Hsla)>,
    cx: &App,
) -> impl IntoElement {
    v_flex()
        .w_full()
        .gap_1()
        .child(div().font_bold().child(title))
        .children(colors.into_iter().map(|(label, color)| color_row(label, color, cx)))
}

fn metric_row(label: impl IntoElement, value: String, cx: &App) -> impl IntoElement {
    h_flex()
        .w_full()
        .justify_between()
        .gap_2()
        .child(label)
        .child(div().text_color(cx.theme().fg_secondary).child(value))
}

fn color_row(label: &'static str, color: Hsla, cx: &App) -> impl IntoElement {
    h_flex().w_full().items_center().justify_between().gap_3().child(label).child(
        h_flex()
            .items_center()
            .child(div().w_12().h_6().bg(color).border_1().border_color(cx.theme().border_primary)),
    )
}
