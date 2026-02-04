use anyhow::Result;
use gpui::{
    App, Application, Bounds, Context, Entity, FocusHandle, ReadGlobal, TitlebarOptions, Window,
    WindowBounds, WindowOptions, div, prelude::*, px, size,
};
use rui::{Root, Table, TableState, TitleBar, h_flex};
use zeevonk::project::file::ProjectFile;

use crate::{app::state::AppState, fixture_table::FixtureTableDelegate};

pub mod action {
    use gpui::{App, KeyBinding, TitlebarOptions, WindowOptions, prelude::*};
    use rui::{AppExt, Root};

    use crate::settings::SettingsView;

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

pub mod state {
    use anyhow::Result;
    use gpui::{App, Global};
    use zeevonk::{Zeevonk, project::file::ProjectFile};

    pub(crate) fn init(zv_project_file: ProjectFile, cx: &mut App) -> Result<()> {
        cx.set_global(AppState::new(zv_project_file)?);
        Ok(())
    }

    pub struct AppState {
        zeevonk: Zeevonk,
    }

    impl AppState {
        pub fn new(zv_project_file: ProjectFile) -> Result<Self> {
            let zeevonk = Zeevonk::new(zv_project_file)?;
            zeevonk.start();

            Ok(Self { zeevonk })
        }

        pub fn zeevonk(&self) -> &Zeevonk {
            &self.zeevonk
        }
    }

    impl Global for AppState {}
}

pub fn run(zv_project_file: ProjectFile) -> Result<()> {
    Application::new().run(|cx: &mut App| {
        rui::init(cx);

        action::init(cx);

        state::init(zv_project_file, cx).expect("should initialize app state");

        cx.activate(true);

        let bounds = Bounds::centered(None, size(px(1080.0), px(720.0)), cx);
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

    fixture_table_state: Entity<TableState<FixtureTableDelegate>>,
}

impl RadiantApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Result<Self> {
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);

        let fixtures = cx.new(|cx| {
            AppState::global(cx).zeevonk().project().stage().fixtures().values().cloned().collect()
        });
        let fixture_table_state =
            cx.new(|cx| TableState::new(FixtureTableDelegate::new(fixtures), window, cx));

        Ok(Self { focus_handle, fixture_table_state })
    }

    fn render_title_bar_content(
        &mut self,
        window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        h_flex().size_full().justify_between().child(window.window_title())
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
            .child(div().overflow_hidden().child(Table::new(self.fixture_table_state.clone())))
    }
}
