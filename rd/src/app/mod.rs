use std::path::PathBuf;

use anyhow::Result;
use gpui::{Context, Entity, FocusHandle, Window, div, prelude::*, px, size};
use rd_ui::{Button, Icon, IconSize, IconVariant, TITLE_BAR_HEIGHT, h_flex};

use crate::{app::ui::LayoutViewer, engine::Engine};

mod settings;
mod ui;

pub mod action {
    use gpui::{App, KeyBinding, prelude::*};
    use rd_ui::{Root, SETTINGS_WINDOW_OPTIONS, SettingsAppExt as _};

    use crate::app::settings::SettingsView;

    gpui::actions!([OpenSettings, Save]);

    pub(crate) fn init(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("secondary-,", OpenSettings, None),
            KeyBinding::new("secondary-s", Save, None),
        ]);

        cx.on_action::<OpenSettings>(|_, cx| {
            cx.open_settings(Some(SETTINGS_WINDOW_OPTIONS), |window, cx| {
                cx.new(|cx| Root::new(cx.new(|cx| SettingsView::new(window, cx)), window, cx))
                    .into()
            });
        });

        cx.on_action::<Save>(|_, cx| {
            todo!();
        });
    }
}

pub fn run(showfile_path: Option<PathBuf>) -> Result<()> {
    rd_ui::build_simple_app()
        .window_title("Radiant")
        .window_size(size(px((18.0 + 1.5) * 80.0), px(12.0 * 80.0) + TITLE_BAR_HEIGHT))
        .title_bar_content(|_, cx| cx.new(|_| TitleBarContent).into())
        .run(|window, cx| {
            crate::app::action::init(cx);

            let engine = Engine::new(showfile_path, cx).expect("should create engine");
            engine.start();
            cx.set_global(engine);

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

    fn render_content(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div().size_full().child(self.layout_viewer.clone())
    }
}

impl Render for RadiantApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .overflow_hidden()
            .child(self.render_content(window, cx))
    }
}

struct TitleBarContent;

impl Render for TitleBarContent {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex().flex_row_reverse().size_full().gap_2().child(
            Button::new("settings")
                .icon(Icon::new(IconVariant::Settings, IconSize::ExtraSmall))
                .on_click(|_, window, cx| {
                    window.dispatch_action(Box::new(action::OpenSettings), cx);
                }),
        )
    }
}
