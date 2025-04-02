use crate::{
    effect_graph::{self},
    frame::{DebugFrame, GraphEditor, MainFrame},
};
use frames::FrameContainer;
use gpui::*;
use ui::ActiveTheme;

pub struct RadiantApp {
    frame_container: Entity<FrameContainer<MainFrame>>,
}

impl RadiantApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.activate(true);

        let effect_graph = cx.new(|_cx| effect_graph::get_graph());

        Self {
            frame_container: cx.new(|cx| {
                let mut container = FrameContainer::new(size(20, 12), px(80.0));
                container.add_frame(
                    MainFrame::EffectGraphEditor(
                        cx.new(|cx| GraphEditor::new(effect_graph.clone(), window, cx)),
                    ),
                    bounds(point(0, 0), size(15, 12)),
                    cx,
                );
                container.add_frame(
                    MainFrame::DebugFrame(cx.new(|_cx| DebugFrame::new(effect_graph))),
                    bounds(point(15, 0), size(2, 4)),
                    cx,
                );
                container
            }),
        }
    }
}

impl Render for RadiantApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(cx.theme().background)
            .text_color(cx.theme().text_primary)
            .child(self.frame_container.clone())
    }
}
