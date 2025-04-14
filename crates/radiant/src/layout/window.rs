use super::MainFrame;
use crate::showfile::{self, EffectGraph, MainFrameKind, effect_graph};
use frames::FrameContainer;
use gpui::*;
use ui::ActiveTheme as _;

const FRAME_CELL_SIZE: Pixels = px(80.0);

pub struct MainWindow {
    frame_container: Entity<FrameContainer<MainFrame>>,
}

impl MainWindow {
    pub fn open(main_window: showfile::MainWindow, cx: &mut App) -> WindowHandle<Self> {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(1600.0), px(960.0)),
                cx,
            ))),
            app_id: Some("radiant".to_string()),

            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            window.set_rem_size(px(14.0));
            cx.new(|cx| Self {
                frame_container: cx
                    .new(|cx| build_frame_container_from_showfile(&main_window, window, cx)),
            })
        })
        .expect("should open window")
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

fn build_frame_container_from_showfile(
    main_window: &showfile::MainWindow,
    window: &mut Window,
    cx: &mut Context<FrameContainer<MainFrame>>,
) -> FrameContainer<MainFrame> {
    let mut container = FrameContainer::new(main_window.size, FRAME_CELL_SIZE);

    for frame in &main_window.frames {
        container.add_frame(build_frame_from_showfile(&frame, window, cx), frame.bounds, cx);
    }

    container
}

fn build_frame_from_showfile(
    frame: &showfile::Frame<MainFrameKind>,
    window: &mut Window,
    cx: &mut App,
) -> MainFrame {
    // FIXME: actually get propper graph
    let graph = cx.new(|_cx| {
        let mut graph = EffectGraph::new();
        effect_graph::insert_templates(&mut graph);
        graph
    });

    match &frame.kind {
        showfile::MainFrameKind::Debugger => {
            MainFrame::Debugger(cx.new(|_cx| super::Debugger::new(graph)))
        }
        showfile::MainFrameKind::EffectGraphEditor {} => {
            MainFrame::EffectGraphEditor(cx.new(|cx| super::GraphEditor::new(graph, window, cx)))
        }
    }
}
