use flow::gpui::GraphEditorView;
use gpui::*;
use show::{EffectGraph, EffectGraphDefinition};
use ui::{container, theme::ActiveTheme, ContainerKind};

use super::{FrameDelegate, FrameView};

pub struct EffectGraphEditorFrameDelegate {
    graph: Model<EffectGraph>,
    editor: View<GraphEditorView<EffectGraphDefinition>>,
}

impl EffectGraphEditorFrameDelegate {
    pub fn new(graph: Model<EffectGraph>, cx: &mut WindowContext) -> Self {
        let editor = cx.new_view(|cx| {
            // FIXME: We could create a helper for these 'model mappings'.
            let flow_graph = cx.new_model(|cx| graph.read(cx).graph.clone());
            cx.observe(&graph, {
                let flow_graph = flow_graph.clone();
                move |editor, graph, cx| {
                    log::debug!("Updating effect graph editor with new graph");
                    flow_graph.update(cx, |flow_graph, cx| {
                        *flow_graph = graph.read(cx).graph.clone();
                        cx.notify();
                    });

                    *editor = GraphEditorView::new(flow_graph.clone(), cx);
                    cx.notify();
                }
            })
            .detach();

            GraphEditorView::new(flow_graph.clone(), cx)
        });

        Self { graph, editor }
    }

    fn save_graph(&self, cx: &mut WindowContext) {
        let new_graph = self.graph.read(cx).clone();

        log::info!(
            "Saving effect graph '{}' ({})",
            new_graph.label,
            new_graph.id
        );
        todo!("Impelment saving");
    }
}

impl FrameDelegate for EffectGraphEditorFrameDelegate {
    fn title(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> String {
        format!(
            "Effect Graph Editor ({} [{}])",
            self.graph.read(cx).label,
            self.graph.read(cx).id,
        )
    }

    fn render_header_content(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> impl IntoElement {
        div()
            .id("save-button")
            .on_click(cx.listener(|this, _, cx| this.delegate.save_graph(cx)))
            .border_1()
            .p_2()
            .child("Save")
    }

    fn render_content(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> impl IntoElement {
        container(ContainerKind::Element, cx)
            .size_full()
            .border_color(cx.theme().frame_header_border)
            .child(self.editor.clone())
    }
}
