use std::time::Duration;

use crate::{
    effect_graph::{self},
    frame::{GraphEditor, MainFrame},
};
use flow_gpui::{Graph, flow::ProcessingContext};
use frames::FrameContainer;
use gpui::*;
use ui::theme::ActiveTheme;

pub struct RadiantApp {
    value: f64,
    frame_container: Entity<FrameContainer<MainFrame>>,
}

impl RadiantApp {
    pub fn build(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            cx.activate(true);

            let effect_graph = cx.new(|_cx| effect_graph::get_graph());
            start_graph_processor(effect_graph.clone(), cx);

            Self {
                frame_container: cx.new(|cx| {
                    let mut container = FrameContainer::new(size(20, 12), px(80.0));
                    container.add_frame(
                        MainFrame::EffectGraphEditor(GraphEditor::build(effect_graph, cx)),
                        bounds(point(0, 0), size(19, 12)),
                        cx,
                    );
                    container
                }),
                value: 0.0,
            }
        })
    }
}

impl Render for RadiantApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(cx.theme().background)
            .text_color(cx.theme().text_primary)
            .text_xs()
            .child(self.frame_container.clone())
            .child(format!("{}", self.value))
    }
}

fn start_graph_processor(
    graph: Entity<Graph<effect_graph::GraphDef>>,
    cx: &mut Context<RadiantApp>,
) {
    cx.spawn({
        |view, cx| async move {
            loop {
                cx.update({
                    let view = view.clone();
                    let graph = graph.clone();
                    move |cx| {
                        view.update(cx, |view, cx| {
                            let value = graph.update(cx, |graph, _cx| {
                                let mut pcx = ProcessingContext::new();
                                graph.process(&mut pcx);
                                pcx.state().value
                            });
                            view.value = value;
                        })
                        .unwrap()
                    }
                })
                .unwrap();

                Timer::after(Duration::from_micros(16667)).await;
            }
        }
    })
    .detach();
}
