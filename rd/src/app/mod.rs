use std::path::PathBuf;

use anyhow::Result;
use gpui::{Context, Entity, FocusHandle, Hsla, ReadGlobal, Window, div, prelude::*, px, size};
use rd_engine::{Engine, EngineHandle, Project, cmd::Command, event::Event};
use rd_ui::{
    ActiveTheme, Button, Icon, IconSize, IconVariant, TITLE_BAR_HEIGHT, TITLE_BAR_RIGHT_PADDING,
    h_flex,
};

use crate::{
    app::{state::State, ui::LayoutViewer},
    engine::EngineAppExt,
};

mod settings;
mod state;
mod ui;

pub mod action {
    use gpui::{App, KeyBinding, ReadGlobal, prelude::*};
    use rd_engine::cmd::Command;
    use rd_ui::{Root, SETTINGS_WINDOW_OPTIONS, SettingsAppExt as _};

    use crate::{
        app::{
            settings::SettingsView,
            state::{Mode, State},
        },
        engine::EngineAppExt,
    };

    gpui::actions!([SettingsOpen, Save, Clear, Store, HighlightToggle]);

    pub(crate) fn init(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("secondary-,", SettingsOpen, None),
            KeyBinding::new("secondary-s", Save, None),
            KeyBinding::new("escape", Clear, None),
            KeyBinding::new("s", Store, None),
            KeyBinding::new("h", HighlightToggle, None),
        ]);

        cx.on_action::<SettingsOpen>(|_, cx| {
            cx.open_settings(Some(SETTINGS_WINDOW_OPTIONS), |window, cx| {
                cx.new(|cx| Root::new(cx.new(|cx| SettingsView::new(window, cx)), window, cx))
                    .into()
            });
        });

        cx.on_action::<HighlightToggle>(|_, cx| {
            cx.execute_engine_cmd(Command::HighlightToggle);
        });

        cx.on_action::<Clear>(|_, cx| {
            if !cx.engine_snapshot().selection().is_empty() {
                cx.execute_engine_cmd(Command::SelectionClear);
                return;
            }

            if State::global(cx).mode().read(cx) == &Mode::Normal {
                cx.execute_engine_cmd(Command::ProgrammerClear);
                return;
            }

            State::global(cx).mode().write(cx, Mode::Normal);
        });

        cx.on_action::<Store>(|_, cx| {
            State::global(cx).mode().write(cx, Mode::Store);
        });

        cx.on_action::<Save>(|_, cx| match cx.engine_snapshot().showfile_path() {
            Some(path) => {
                cx.execute_engine_cmd(Command::Save { path: path.to_path_buf() });
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

            crate::app::state::init(cx);
            crate::app::action::init(cx);
            crate::engine::init(EngineHandle::new(rd_engine), cx);

            cx.new(|cx| RadiantApp::new(window, cx).expect("should create app"))
        });

    Ok(())
}

struct RadiantApp {
    focus_handle: FocusHandle,

    layout_viewer: Entity<LayoutViewer>,

    highlight: Entity<bool>,
}

impl RadiantApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Result<Self> {
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);

        let layout_viewer = cx.new(|cx| LayoutViewer::new(window, cx));

        let highlight = cx.new(|cx| cx.engine_snapshot().highlight());
        cx.on_engine_event({
            let highlight = highlight.clone();
            move |event, cx| match event {
                Event::HighlightChanged { enabled } => highlight.write(cx, *enabled),
                _ => {}
            }
        })
        .detach();

        Ok(Self { focus_handle, layout_viewer, highlight })
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
        let render_indicator = |label: &str, on: bool, accent: Hsla| {
            let color = if on { accent } else { cx.theme().fg_primary };

            h_flex()
                .justify_center()
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

        let mode = State::global(cx).mode().read(cx);
        let indicators = h_flex()
            .gap_2()
            .child(
                render_indicator(&mode.to_string().to_ascii_lowercase(), true, mode.color(cx))
                    .w_16(),
            )
            .child(
                render_indicator("highlight", *self.highlight.read(cx), cx.theme().accent)
                    .cursor_pointer()
                    .on_any_mouse_down(cx.listener(|_, _, _, cx| {
                        cx.execute_engine_cmd(Command::HighlightToggle);
                        cx.notify();
                    })),
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
