use std::path::PathBuf;

use anyhow::{Context as _, Result};
use gpui::{Context, Entity, FocusHandle, Window, div, prelude::*, px, size};
use rd_core::Engine;
use rd_ui::{
    Button, Icon, IconSize, IconVariant, TITLE_BAR_HEIGHT, TileGrid, TileGridState, h_flex,
};

use crate::state::AppState;

pub mod action {
    use gpui::{App, KeyBinding, prelude::*};
    use rd_ui::{Root, SETTINGS_WINDOW_OPTIONS, SettingsAppExt as _};

    use crate::{settings::SettingsView, state::AppState};

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
            if let Err(err) = AppState::save(cx) {
                log::error!("failed to save project: {err}");
            }
        });
    }
}

pub fn run(showfile_path: Option<PathBuf>) -> Result<()> {
    let engine = Engine::new(showfile_path).context("failed to load showfile for directory")?;

    rd_ui::build_simple_app()
        .window_title("Radiant")
        .window_size(size(px(18.0 * 80.0), px(12.0 * 80.0) + TITLE_BAR_HEIGHT))
        .title_bar_content(|_, cx| cx.new(|_| TitleBarContent).into())
        .run(|window, cx| {
            crate::app::action::init(cx);
            crate::state::init(engine, cx).expect("failed to initialize application state");
            crate::state::action::init(cx);
            cx.new(|cx| RadiantApp::new(window, cx).expect("should create app"))
        });

    Ok(())
}

struct RadiantApp {
    focus_handle: FocusHandle,

    tile_grid_state: Entity<TileGridState>,
}

impl RadiantApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Result<Self> {
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);

        let layout = AppState::layout(cx).clone();
        let tile_grid_state = cx.new(|cx| layout.read(cx).clone().to_tile_grid_state(window, cx));
        cx.observe_in(&layout, window, |app, layout, window, cx| {
            let next_state = layout.read(cx).clone().to_tile_grid_state(window, cx);
            app.tile_grid_state.update(cx, |state, cx| {
                *state = next_state;
                cx.notify();
            })
        })
        .detach();

        Ok(Self { focus_handle, tile_grid_state })
    }

    fn render_content(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div().size_full().child(TileGrid::new(self.tile_grid_state.clone()))
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
