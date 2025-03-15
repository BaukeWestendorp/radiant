use crate::{
    effect_graph::{self},
    frame::{DebugFrame, GraphEditor, MainFrame},
};
use frames::FrameContainer;
use gpui::*;
use ui::theme::ActiveTheme;

pub struct RadiantApp {
    frame_container: Entity<FrameContainer<MainFrame>>,
}

impl RadiantApp {
    pub fn build(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            cx.activate(true);

            let effect_graph = cx.new(|_cx| effect_graph::get_graph());

            Self {
                frame_container: cx.new(|cx| {
                    let mut container = FrameContainer::new(size(20, 12), px(80.0));
                    container.add_frame(
                        MainFrame::EffectGraphEditor(GraphEditor::build(effect_graph, cx)),
                        bounds(point(0, 0), size(15, 12)),
                        cx,
                    );
                    container.add_frame(
                        MainFrame::DebugFrame(DebugFrame::build(window, cx)),
                        bounds(point(15, 0), size(4, 4)),
                        cx,
                    );
                    container
                }),
            }
        })
    }
}

impl Render for RadiantApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(cx.theme().background)
            .text_xs()
            .text_color(cx.theme().text_primary)
            .child(self.frame_container.clone())
    }
}
