use crate::{app::APP_ID, layout::MainFrame};
use anyhow::Context as _;
use frames::FrameContainer;
use gpui::*;
use show::Show;
use ui::{ActiveTheme as _, utils::z_stack};

use super::{DEFAULT_REM_SIZE, settings::SettingsWindow};

const FRAME_CELL_SIZE: Pixels = px(80.0);

pub struct MainWindow {
    frame_container: Entity<FrameContainer<MainFrame>>,
    settings_window: Option<Entity<SettingsWindow>>,
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
            ..Default::default()
        };

        cx.open_window(window_options, |w, cx| {
            w.set_rem_size(DEFAULT_REM_SIZE);
            cx.new(|cx| Self {
                frame_container: cx.new(|cx| frame_container_from_showfile(w, cx)),
                settings_window: None,
            })
        })
        .context("open main window")
    }

    pub fn open_settings_window(&mut self, w: &mut Window, cx: &mut Context<Self>) {
        if self.settings_window.is_none() {
            let this = cx.entity();
            self.settings_window = Some(cx.new(|cx| SettingsWindow::new(this, w, cx)));
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
            Some(settings_window) => div().size_full().m_2().child(settings_window.clone()),
            None => div(),
        };

        z_stack([main_layout, settings_window])
            .size_full()
            .on_action(cx.listener(Self::handle_open_settings))
    }
}

// We can't really put this in the `frames` crate,
// so let's just add a helper function here.
fn frame_container_from_showfile(
    window: &mut Window,
    cx: &mut Context<FrameContainer<MainFrame>>,
) -> FrameContainer<MainFrame> {
    let main_window = Show::global(cx).layout.main_window.clone();

    let mut container = FrameContainer::new(main_window.size, FRAME_CELL_SIZE);

    for frame in &main_window.frames {
        container.add_frame(MainFrame::from_show(frame, window, cx), frame.bounds, cx);
    }

    container
}

pub mod actions {
    use gpui::*;
    use show::Show;

    actions!(main_window, [OpenSettings]);

    pub fn init(cx: &mut App) {
        bind_global_keys(cx);
        handle_global_actions(cx);
    }

    fn bind_global_keys(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("secondary-,", OpenSettings, None)]);
    }

    fn handle_global_actions(cx: &mut App) {
        cx.on_action::<OpenSettings>(|_, cx| {
            let path = Show::global(cx).path.clone();
            if let Some(path) = &path {
                log::info!("Saving show to '{}'", path.display());
                match Show::update_global(cx, |show, cx| show.save_to_file(path, cx)) {
                    Ok(_) => log::info!("Show saved successfully"),
                    Err(err) => log::error!("Error saving show: '{}'", err),
                }
                return;
            }

            let Some(dir) = dirs::home_dir() else {
                log::error!("Could not determine home directory");
                return;
            };

            let path = cx.prompt_for_new_path(&dir.to_path_buf());
            cx.spawn(async move |cx| {
                let Ok(result) = path.await else {
                    log::error!("Error awaiting path selection");
                    return;
                };

                let Ok(Some(path)) = result else {
                    if let Ok(None) = result {
                        log::info!("Save cancelled - no path selected");
                    } else if let Err(err) = result {
                        log::error!("Error prompting for path: '{}'", err);
                    }
                    return;
                };

                log::info!("Saving show to '{}'", path.display());
                let result = cx.update_global(|show: &mut Show, cx| show.save_to_file(&path, cx));

                match result {
                    Ok(Ok(_)) => log::info!("Show saved successfully"),
                    Ok(Err(err)) => log::error!("Error saving show: '{}'", err),
                    Err(err) => log::error!("Error updating global show: '{}'", err),
                }
            })
            .detach();
        });
    }
}
