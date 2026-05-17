use gpui::prelude::*;
use gpui::{EmptyView, Entity, Keystroke, Window, div, point, px};
use rd_ui::{
    ActiveTheme as _, Binding, Button, Icon, IconSize, IconVariant, SettingsAppExt as _, Tab, Tabs,
    TabsState, TabsVariant, TitleBar, dot_grid, line_grid, scrollable_line_grid, section,
};

pub struct MiscPreview {
    tabs: Entity<TabsState>,

    binding: Entity<BindingPreview>,
    grid: Entity<GridPreview>,
    icon: Entity<IconPreview>,
    org: Entity<OrgPreview>,
    settings: Entity<SettingsPreview>,
    title_bar: Entity<TitleBarPreview>,
}

impl MiscPreview {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            tabs: cx.new(|_| TabsState::new().with_selected("binding")),

            binding: cx.new(|cx| BindingPreview::new(window, cx)),
            grid: cx.new(|cx| GridPreview::new(window, cx)),
            icon: cx.new(|cx| IconPreview::new(window, cx)),
            org: cx.new(|cx| OrgPreview::new(window, cx)),
            settings: cx.new(|cx| SettingsPreview::new(window, cx)),
            title_bar: cx.new(|cx| TitleBarPreview::new(window, cx)),
        }
    }
}

impl Render for MiscPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(Tabs::new("misc-tabs", self.tabs.clone(), TabsVariant::Top).tabs([
            Tab::new("binding", "Bindings", self.binding.clone().into_any_element()),
            Tab::new("grid", "Grid", self.grid.clone().into_any_element()),
            Tab::new("icon", "Icon", self.icon.clone().into_any_element()),
            Tab::new("org", "Organization", self.org.clone().into_any_element()),
            Tab::new("settings", "Settings", self.settings.clone().into_any_element()),
            Tab::new("title_bar", "Title Bar", self.title_bar.clone().into_any_element()),
        ]))
    }
}

pub struct BindingPreview {}

impl BindingPreview {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for BindingPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let examples: Vec<(&'static str, &'static str)> = vec![
            ("Copy", "cmd-c"),
            ("Palette", "cmd-shift-p"),
            ("Escape", "escape"),
            ("Enter", "enter"),
            ("Arrow Left", "left"),
            ("Arrow Right", "right"),
            ("Arrow Up", "up"),
            ("Arrow Down", "down"),
            ("Page Up", "pageup"),
            ("Page Down", "pagedown"),
            ("Backspace", "backspace"),
            ("Delete", "delete"),
        ];

        let bindings_row = div().flex().gap_2().flex_wrap().children(
            examples
                .iter()
                .map(|(label, stroke)| {
                    let parsed = Keystroke::parse(stroke)
                        .unwrap_or_else(|_| panic!("invalid keystroke in preview: {stroke}"));

                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(div().w_24().child((*label).to_string()))
                        .child(Binding::new(parsed))
                })
                .collect::<Vec<_>>(),
        );

        div()
            .p_2()
            .size_full()
            .flex()
            .flex_col()
            .gap_2()
            .child(section("Keystrokes").size_full().child(bindings_row))
    }
}

pub struct GridPreview {}

impl GridPreview {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for GridPreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p_2()
            .size_full()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                section("Dot grid")
                    .w_full()
                    .h_48()
                    .overflow_hidden()
                    .child(dot_grid(px(12.0), cx.theme().accent.opacity(0.75)).size_full()),
            )
            .child(
                section("Line grid")
                    .w_full()
                    .h_48()
                    .overflow_hidden()
                    .child(line_grid(px(16.0), cx.theme().accent.opacity(0.75)).size_full()),
            )
            .child(
                section("Scrollable grid").w_full().h_48().child(
                    div().size_full().relative().overflow_hidden().child(
                        scrollable_line_grid(
                            &point(px(3.0), px(3.0)),
                            px(16.0),
                            cx.theme().accent.opacity(0.75),
                        )
                        .size_full(),
                    ),
                ),
            )
    }
}

pub struct IconPreview {}

impl IconPreview {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for IconPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().p_2().size_full().flex().flex_col().gap_2().child(
            section("Sizes").w_full().h_1_4().child(
                div()
                    .flex()
                    .gap_2()
                    .flex_wrap()
                    .child(Icon::new(IconVariant::Album, IconSize::ExtraSmall))
                    .child(Icon::new(IconVariant::Album, IconSize::Small))
                    .child(Icon::new(IconVariant::Album, IconSize::Regular))
                    .child(Icon::new(IconVariant::Album, IconSize::Large)),
            ),
        )
    }
}

pub struct OrgPreview {}

impl OrgPreview {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for OrgPreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let section_child = div()
            .size_24()
            .border_1()
            .border_color(cx.theme().accent)
            .bg(cx.theme().accent.opacity(0.2))
            .child(div().size_full().flex().justify_center().items_center().child("Content"));

        div()
            .p_2()
            .size_full()
            .flex()
            .gap_2()
            .child(
                section("Section With Child")
                    .child(section("Section Title").w_48().child(section_child)),
            )
            .child(section("Section Without Child"))
    }
}

pub struct SettingsPreview {}

impl SettingsPreview {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for SettingsPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().p_2().size_full().flex().flex_col().gap_2().child(
            section("Settings window").size_full().child(
                div()
                    .flex()
                    .gap_2()
                    .flex_wrap()
                    .child(Button::new("open-settings").child("Open Settings").on_click(
                        |_, _window, cx| {
                            cx.open_settings(None, |_window, cx| cx.new(|_| EmptyView).into());
                        },
                    ))
                    .child(Button::new("close-settings").child("Close Settings").on_click(
                        |_, _window, cx| {
                            cx.close_settings();
                        },
                    )),
            ),
        )
    }
}

pub struct TitleBarPreview {}

impl TitleBarPreview {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for TitleBarPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let title_bar_no_children = TitleBar::new();
        let title_bar_children = TitleBar::new().child(
            div().flex().gap_2().child("Hello World").child(Button::new("button").child("Button")),
        );

        div()
            .size_full()
            .p_2()
            .flex()
            .gap_2()
            .child(section("Without Children").child(title_bar_no_children))
            .child(section("With Children").child(title_bar_children))
    }
}
