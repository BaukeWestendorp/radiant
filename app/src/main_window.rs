use std::time::{Duration, Instant};

use eyre::Context as _;
use gpui::prelude::*;
use gpui::{
    App, Bounds, Context, Div, Entity, Hsla, ReadGlobal, Timer, TitlebarOptions, Window,
    WindowBounds, WindowHandle, WindowOptions, div, px, size,
};

use crate::app::AppState;
use crate::error::Result;
use crate::ui::{ActiveTheme, InteractiveColor, root, titlebar};

pub struct MainWindow {
    io_status: Entity<IoStatusIndicators>,
}

impl MainWindow {
    pub fn open(cx: &mut App) -> Result<WindowHandle<MainWindow>> {
        let bounds = Bounds::centered(None, size(px(500.0), px(500.0)), cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some("Radiant".into()),
                appears_transparent: true,
                traffic_light_position: Some(crate::ui::TRAFFIC_LIGHT_POSITION),
            }),

            app_id: Some("radiant".to_string()),
            ..Default::default()
        };

        cx.open_window(window_options, |_, cx| {
            cx.new(|cx| Self { io_status: cx.new(|cx| IoStatusIndicators::new(cx)) })
        })
        .map_err(|err| eyre::eyre!(err))
        .context("failed to open main window")
    }
}

impl Render for MainWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let showfile_path = match AppState::global(cx).engine.show().path() {
            Some(path) => path.display().to_string(),
            None => "[unsaved showfile]".to_string(),
        };

        let titlebar = titlebar(window, cx)
            .flex()
            .justify_between()
            .child(div().text_sm().text_color(cx.theme().colors.text.muted()).child(showfile_path))
            .child(self.io_status.clone())
            .pr(crate::ui::TRAFFIC_LIGHT_POSITION.x);

        let content = div().size_full();

        root(cx)
            .flex()
            .flex_col()
            .size_full()
            .bg(cx.theme().colors.bg_primary)
            .child(titlebar)
            .child(content)
    }
}

struct IoStatusIndicators;

impl IoStatusIndicators {
    const INDICATOR_UPDATE_DELAY: Duration = Duration::from_millis(200);

    pub fn new(cx: &mut Context<Self>) -> Self {
        cx.spawn(async move |this, cx| {
            loop {
                cx.update(|cx| cx.notify(this.entity_id())).ok();
                Timer::after(IoStatusIndicators::INDICATOR_UPDATE_DELAY).await;
            }
        })
        .detach();

        Self
    }
}

impl Render for IoStatusIndicators {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        fn recent(elapsed: Option<Instant>) -> bool {
            elapsed
                .map(|t| t.elapsed() <= IoStatusIndicators::INDICATOR_UPDATE_DELAY)
                .unwrap_or(false)
        }

        fn indicator(is_recent: bool, color: Hsla) -> Div {
            let (opacity, border_color) = if is_recent {
                (1.0, gpui::white().opacity(0.15))
            } else {
                (0.5, gpui::black().opacity(0.3))
            };

            div()
                .size_2()
                .rounded_xs()
                .bg(color.opacity(opacity))
                .border_1()
                .border_color(border_color)
        }

        let io = AppState::global(cx).engine.io_status();
        let adapter_input_indicator =
            indicator(recent(io.last_adapter_input()), cx.theme().colors.accent);
        let dmx_output_indicator =
            indicator(recent(io.last_dmx_output()), cx.theme().colors.accent);

        div()
            .flex()
            .flex_col()
            .justify_center()
            .gap_1()
            .h_full()
            .child(adapter_input_indicator)
            .child(dmx_output_indicator)
    }
}
