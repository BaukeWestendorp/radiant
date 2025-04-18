use super::MainFrame;
use crate::app::APP_ID;
use anyhow::Context as _;
use frames::FrameContainer;
use gpui::*;
use show::Show;
use ui::ActiveTheme as _;

const FRAME_CELL_SIZE: Pixels = px(80.0);
pub const DEFAULT_REM_SIZE: Pixels = px(14.0);

pub struct MainWindow {
    frame_container: Entity<FrameContainer<MainFrame>>,
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

        cx.open_window(window_options, |window, cx| {
            window.set_rem_size(DEFAULT_REM_SIZE);
            cx.new(|cx| Self {
                frame_container: cx.new(|cx| frame_container_from_showfile(window, cx)),
            })
        })
        .context("open window")
    }
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().text_primary)
            .child(self.frame_container.clone())
    }
}

fn frame_container_from_showfile(
    window: &mut Window,
    cx: &mut Context<FrameContainer<MainFrame>>,
) -> FrameContainer<MainFrame> {
    let main_window = Show::global(cx).layout.main_window.clone();

    let mut container = FrameContainer::new(main_window.size.into(), FRAME_CELL_SIZE);

    for frame in &main_window.frames {
        container.add_frame(
            MainFrame::from_show(&frame, window, cx),
            frame.bounds.clone().into(),
            cx,
        );
    }

    container
}
