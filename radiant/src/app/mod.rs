use anyhow::Result;
use gpui::{
    App, Application, Bounds, Context, Entity, FocusHandle, QuitMode, TitlebarOptions, Window,
    WindowBounds, WindowOptions, bounds, div, point, prelude::*, px, size,
};
use rui::{
    Button, Icon, IconSize, IconVariant, PoolTile, Root, TITLE_BAR_HEIGHT, TileGrid, TileGridState,
    TitleBar, h_flex,
};

use crate::{
    app::ui::tiles::{FixturesTile, GroupsPoolTile},
    showfile::Showfile,
};

mod settings;
mod state;
mod ui;

pub mod action {
    use gpui::{App, KeyBinding, TitlebarOptions, WindowOptions, prelude::*};
    use rui::{AppExt, Root};

    use super::settings::SettingsView;

    gpui::actions!([OpenSettings]);

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

        let tile_grid_state = cx.new(|cx| {
            let mut tile_grid_state = TileGridState::new();
            let cell_size = px(80.0);
            tile_grid_state
                .add_tile(FixturesTile::new(window, cx), bounds(point(0, 0), size(8, 12)));
            let groups_pool_delegate = cx.new(|_cx| GroupsPoolTile::new());
            tile_grid_state.add_tile(
                PoolTile::new(groups_pool_delegate, cell_size),
                bounds(point(8, 0), size(10, 8)),
            );
            tile_grid_state
        });

        Ok(Self { focus_handle, tile_grid_state })
    }

    fn render_title_bar_content(
        &mut self,
        window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        h_flex().size_full().justify_between().child(window.window_title()).child(
            Button::new("settings")
                .icon(Icon::new(IconVariant::Settings, IconSize::ExtraSmall))
                .on_click(|_, window, cx| {
                    window.dispatch_action(Box::new(action::OpenSettings), cx);
                }),
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
