use crate::app::APP_ID;
use anyhow::Context as _;
use gpui::{prelude::FluentBuilder, *};
use ui::{ActiveTheme as _, InteractiveColor as _};

use super::DEFAULT_REM_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Patch,
    DmxIo,
}

impl Tab {
    pub fn label(&self) -> &str {
        match self {
            Tab::Patch => "Patch",
            Tab::DmxIo => "DMX IO",
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Tab::Patch => "patch",
            Tab::DmxIo => "dmx_io",
        }
    }
}

pub struct SettingsWindow {
    active_tab: Tab,
}

impl SettingsWindow {
    pub fn open(cx: &mut App) -> anyhow::Result<WindowHandle<Self>> {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(800.0), px(600.0)),
                cx,
            ))),
            app_id: Some(APP_ID.to_string()),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            window.set_rem_size(DEFAULT_REM_SIZE);
            cx.new(|_cx| Self { active_tab: Tab::Patch })
        })
        .context("open settings window")
    }

    fn render_sidebar(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        const TABS: [Tab; 2] = [Tab::Patch, Tab::DmxIo];

        div()
            .children(TABS.iter().map(|tab| {
                div()
                    .id(ElementId::Name(format!("settings-tab-{}", tab.id()).into()))
                    .bg(cx.theme().element_background)
                    .p_1()
                    .border_1()
                    .border_color(cx.theme().border)
                    .hover(|e| {
                        e.bg(cx.theme().background.hovered())
                            .border_color(cx.theme().border.hovered())
                    })
                    .when(self.active_tab == *tab, |e| {
                        e.bg(cx.theme().element_background_selected)
                            .border_color(cx.theme().border_selected)
                            .hover(|e| {
                                e.bg(cx.theme().element_background_selected.hovered())
                                    .border_color(cx.theme().border_selected.hovered())
                            })
                    })
                    .rounded(cx.theme().radius)
                    .cursor_pointer()
                    .on_click(cx.listener(|view, _, _window, _cx| view.active_tab = *tab))
                    .child(tab.label())
            }))
            .flex()
            .flex_col()
            .gap_2()
            .p_2()
            .border_r_1()
            .border_color(cx.theme().border)
            .w_56()
            .h_full()
    }

    fn render_content(&self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(match self.active_tab {
            Tab::Patch => div().child("PATCH"),
            Tab::DmxIo => div().child("DMX IO"),
        })
    }
}

impl Render for SettingsWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .bg(cx.theme().background)
            .text_color(cx.theme().text_primary)
            .child(self.render_sidebar(window, cx))
            .child(self.render_content(window, cx))
    }
}
