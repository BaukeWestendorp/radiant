use crate::{app::APP_ID, layout::MainFrame};
use anyhow::Context as _;
use frames::FrameContainer;
use gpui::*;
use show::{Show, layout::Layout};
use ui::{ActiveTheme as _, root, utils::z_stack};

use super::{DEFAULT_REM_SIZE, VirtualWindow, settings::SettingsWindow};

pub const FRAME_CELL_SIZE: Pixels = px(80.0);

pub struct MainWindow {
    frame_container: Entity<FrameContainer<MainFrame>>,
    settings_window: Option<Entity<VirtualWindow<SettingsWindow>>>,

    focus_handle: FocusHandle,
}

impl MainWindow {
    pub fn open(cx: &mut App) -> anyhow::Result<WindowHandle<Self>> {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(1600.0), px(960.0)),
                cx,
            ))),
            app_id: Some(APP_ID.to_string()),
            titlebar: Some(TitlebarOptions { title: Some("Radiant".into()), ..Default::default() }),
            ..Default::default()
        };

        cx.open_window(window_options, |w, cx| {
            w.set_rem_size(DEFAULT_REM_SIZE);

            cx.new(|cx| {
                let frame_container = cx.new(|cx| {
                    let layout = Show::global(cx).layout.clone();

                    cx.observe_in(&layout, w, |this, layout, w, cx| {
                        *this = frame_container_from_showfile(layout, w, cx);
                        log::debug!("Updating main frame container");
                        cx.notify();
                    })
                    .detach();

                    frame_container_from_showfile(layout, w, cx)
                });

                Self { frame_container, settings_window: None, focus_handle: cx.focus_handle() }
            })
        })
        .context("open main window")
    }

    pub fn open_settings_window(&mut self, w: &mut Window, cx: &mut Context<Self>) {
        if self.settings_window.is_none() {
            let this = cx.entity();
            let vw = cx.new(|cx| VirtualWindow::new(SettingsWindow::new(this, w, cx)));
            self.settings_window = Some(vw);
            cx.notify();
        }
    }

    pub fn close_settings_window(&mut self, cx: &mut Context<Self>) {
        self.settings_window.take();
        cx.notify();
    }
}

impl MainWindow {
    fn handle_open_settings(
        &mut self,
        _: &actions::OpenSettings,
        w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_settings_window(w, cx);
    }
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let main_layout = div()
            .size_full()
            .bg(cx.theme().colors.bg_primary)
            .text_color(cx.theme().colors.text)
            .child(self.frame_container.clone());

        let settings_window = match &self.settings_window {
            Some(settings_window) => div().size_full().p_2().child(settings_window.clone()),
            None => div(),
        };

        root(cx)
            .track_focus(&self.focus_handle)
            .key_context(actions::KEY_CONTEXT)
            .size_full()
            .on_action(cx.listener(Self::handle_open_settings))
            .child(z_stack([main_layout, settings_window]).size_full())
    }
}

// We can't really put this in the `frames` crate,
// so let's just add a helper function here.
fn frame_container_from_showfile(
    layout: Entity<Layout>,
    window: &mut Window,
    cx: &mut Context<FrameContainer<MainFrame>>,
) -> FrameContainer<MainFrame> {
    let main_window = layout.read(cx).main_window.clone();

    let mut container = FrameContainer::new(main_window.size, FRAME_CELL_SIZE);

    for frame in &main_window.frames {
        container.add_frame(MainFrame::from_show(frame, window, cx), frame.bounds, cx);
    }

    container
}

pub mod actions {
    use gpui::*;

    pub const KEY_CONTEXT: &str = "MainWindow";

    actions!(main_window, [OpenSettings]);

    pub fn init(cx: &mut App) {
        bind_keys(cx);
    }

    fn bind_keys(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("secondary-,", OpenSettings, Some(KEY_CONTEXT))]);
    }
}
