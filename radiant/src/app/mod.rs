use anyhow::Result;
use gpui::{
    App, Application, Bounds, Context, Entity, FocusHandle, QuitMode, TitlebarOptions, Window,
    WindowBounds, WindowOptions, div, prelude::*, px, size,
};
use rui::{
    ActiveTheme, Button, Icon, IconSize, IconVariant, Root, TITLE_BAR_HEIGHT, TileGrid,
    TileGridState, TitleBar, h_flex,
};

use crate::{app::state::AppState, showfile::Showfile};

pub mod state;
pub mod ui;

mod settings;

pub mod action {
    use gpui::{App, KeyBinding, TitlebarOptions, WindowOptions, prelude::*};
    use rui::{AppExt, Root};

    use super::settings::SettingsView;

    gpui::actions!([OpenSettings, Debug]);

    pub(crate) fn init(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("secondary-,", OpenSettings, None)]);

        cx.on_action::<OpenSettings>(|_, cx| {
            cx.open_settings(
                Some(WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Settings".into()),
                        appears_transparent: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                |window, cx| {
                    cx.new(|cx| Root::new(cx.new(|cx| SettingsView::new(window, cx)), window, cx))
                        .into()
                },
            );
        });
    }
}

pub fn run(showfile: Showfile) -> Result<()> {
    Application::new()
        .with_assets(rui::Assets::default())
        .with_quit_mode(QuitMode::LastWindowClosed)
        .run(move |cx: &mut App| {
            rui::init(cx);
            action::init(cx);
            state::init(showfile, cx).expect("should initialize app state");
            state::action::init(cx);

            cx.activate(true);

            let bounds = Bounds::centered(
                None,
                size(px(18.0 * 80.0), px(12.0 * 80.0) + TITLE_BAR_HEIGHT),
                cx,
            );
            let options = WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Radiant".into()),
                    appears_transparent: true,
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            };

            cx.open_window(options, |window, cx| {
                let view = cx.new(|cx| RadiantApp::new(window, cx).expect("should create app"));
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("should open main window");
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

        let layout = AppState::show(cx).layout();
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

    fn render_title_bar_content(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let highlight = AppState::show(cx).modes().read(cx).highlight;
        let highlight_status = h_flex()
            .justify_center()
            .px_1()
            .bg(cx.theme().bg_tertiary)
            .rounded_md()
            .border_1()
            .border_color(cx.theme().border_tertiary)
            .when(highlight, |e| {
                e.border_color(cx.theme().accent).bg(cx.theme().accent.opacity(0.25))
            })
            .text_sm()
            .child("Highlight");

        h_flex().size_full().justify_between().child(window.window_title()).child(
            h_flex().gap_2().child(highlight_status).child(
                Button::new("settings")
                    .icon(Icon::new(IconVariant::Settings, IconSize::ExtraSmall))
                    .on_click(|_, window, cx| {
                        window.dispatch_action(Box::new(action::OpenSettings), cx);
                    }),
            ),
        )
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
            .flex()
            .flex_col()
            .size_full()
            .child(TitleBar::new().child(self.render_title_bar_content(window, cx)))
            .child(div().size_full().overflow_hidden().child(self.render_content(window, cx)))
    }
}
