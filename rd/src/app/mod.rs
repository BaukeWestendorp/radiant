use std::path::PathBuf;

use anyhow::Result;
use gpui::{Context, Entity, FocusHandle, Window, div, prelude::*, px, size};
use rd_engine::{Engine, EngineHandle, Project, cmd::Command};
use rd_ui::{
    ActiveTheme, Button, Icon, IconSize, IconVariant, TITLE_BAR_HEIGHT, TITLE_BAR_RIGHT_PADDING,
    h_flex,
};

use crate::{app::ui::LayoutViewer, engine::EngineManager};

mod settings;
mod ui;

pub mod action {
    use gpui::{App, KeyBinding, prelude::*};
    use rd_engine::cmd::Command;
    use rd_ui::{Root, SETTINGS_WINDOW_OPTIONS, SettingsAppExt as _};

    use crate::{app::settings::SettingsView, engine::EngineManager};

    gpui::actions!([SettingsOpen, Save, HighlightToggle]);

    pub(crate) fn init(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("secondary-,", SettingsOpen, None),
            KeyBinding::new("secondary-s", Save, None),
            KeyBinding::new("h", HighlightToggle, None),
        ]);

        cx.on_action::<SettingsOpen>(|_, cx| {
            cx.open_settings(Some(SETTINGS_WINDOW_OPTIONS), |window, cx| {
                cx.new(|cx| Root::new(cx.new(|cx| SettingsView::new(window, cx)), window, cx))
                    .into()
            });
        });

        cx.on_action::<HighlightToggle>(|_, cx| {
            EngineManager::execute(cx, Command::HighlightToggle);
        });

        cx.on_action::<Save>(|_, cx| match EngineManager::snapshot(cx).showfile_path() {
            Some(_path) => {
                todo!();
            }
            None => {
                log::error!("FIXME: implement saving new showfiles");
            }
        });
    }
}

pub fn run(showfile_path: Option<PathBuf>) -> Result<()> {
    rd_ui::build_simple_app()
        .window_title(if cfg!(debug_assertions) { "Radiant [debug-mode]" } else { "Radiant" })
        .window_size(size(px((18.0 + 1.5) * 80.0), px(12.0 * 80.0) + TITLE_BAR_HEIGHT * 2.0))
        .title_bar_content(|_, cx| cx.new(|_| TitleBarContent).into())
        .run(|window, cx| {
            crate::app::action::init(cx);

            let project = match showfile_path {
                Some(showfile_path) => match Project::load_from_folder(showfile_path) {
                    Ok(project) => project,
                    Err(err) => {
                        log::error!("Could not load showfile: {err}");
                        Project::new()
                    }
                },
                None => Project::new(),
            };

            let rd_engine = match Engine::new(project) {
                Ok(rd_engine) => rd_engine,
                Err(err) => {
                    log::error!("Could not load engine: {err}");
                    Engine::new(Project::new()).expect("should create new showfile")
                }
            };

            let rd_engine_handle = EngineHandle::new(rd_engine);

            let engine_handle = EngineManager::new(rd_engine_handle, cx);
            cx.set_global(engine_handle);

            cx.new(|cx| RadiantApp::new(window, cx).expect("should create app"))
        });

    Ok(())
}

struct RadiantApp {
    focus_handle: FocusHandle,

    layout_viewer: Entity<LayoutViewer>,
}

impl RadiantApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Result<Self> {
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);

        let layout_viewer = cx.new(|cx| LayoutViewer::new(window, cx));

        Ok(Self { focus_handle, layout_viewer })
    }

    fn render_content(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(self.layout_viewer.clone())
            .child(self.render_status_bar(window, cx))
    }

    fn render_status_bar(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let render_indicator = |label: &str, on: bool| {
            let color = if on { cx.theme().accent } else { cx.theme().fg_primary };

            h_flex()
                .text_xs()
                .border_1()
                .border_color(color.opacity(0.5))
                .rounded(cx.theme().radius)
                .text_color(color)
                .bg(color.opacity(0.1))
                .px_2()
                .child(label.to_owned())
        };

        let version =
            div().text_sm().text_color(cx.theme().fg_tertiary).child(crate::version_string());

        let indicators = h_flex().gap_2().child(
            render_indicator("highlight", EngineManager::snapshot(cx).highlight())
                .cursor_pointer()
                .on_any_mouse_down(|_, _, cx| {
                    EngineManager::execute(cx, Command::HighlightToggle);
                }),
        );

        h_flex()
            .bg(cx.theme().title_bar)
            .border_t_1()
            .border_color(cx.theme().title_bar_border)
            .justify_between()
            .w_full()
            .h(TITLE_BAR_HEIGHT)
            .pr(TITLE_BAR_RIGHT_PADDING)
            .pl(TITLE_BAR_RIGHT_PADDING)
            .child(indicators)
            .child(version)
    }
}

impl Render for RadiantApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().track_focus(&self.focus_handle).child(self.render_content(window, cx))
    }
}

struct TitleBarContent;

impl Render for TitleBarContent {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex().flex_row_reverse().size_full().gap_2().child(
            Button::new("settings")
                .icon(Icon::new(IconVariant::Settings, IconSize::ExtraSmall))
                .on_click(|_, window, cx| {
                    window.dispatch_action(Box::new(action::SettingsOpen), cx);
                }),
        )
    }
}
