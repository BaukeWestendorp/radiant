use flow::gpui::GraphEditorView;
use gpui::*;
use show::{AssetPool, EffectGraph, EffectGraphDefinition};
use ui::{container, theme::ActiveTheme, ContainerKind};

use super::{FrameDelegate, FrameView};

pub struct EffectGraphEditorFrameDelegate {
    graph_editor: View<GraphEditorView<EffectGraphDefinition>>,
}

impl EffectGraphEditorFrameDelegate {
    pub fn new(
        window: Model<show::Window>,
        effect_graph_asset_pool: Model<AssetPool<EffectGraph>>,
        cx: &mut WindowContext,
    ) -> Self {
        let graph_model = cx.new_model(|cx| {
            effect_graph_asset_pool
                .read(cx)
                .get(&window.read(cx).selected_effect_graph.unwrap())
                .unwrap()
                .clone()
                .graph
        });

        cx.observe(&window, {
            let graph_model = graph_model.clone();
            move |window, cx| {
                graph_model.update(cx, |graph, cx| {
                    *graph = effect_graph_asset_pool
                        .read(cx)
                        .get(&window.read(cx).selected_effect_graph.unwrap())
                        .unwrap()
                        .clone()
                        .graph;
                })
            }
        })
        .detach();

        Self {
            graph_editor: GraphEditorView::build(graph_model, cx),
        }
    }
}

impl FrameDelegate for EffectGraphEditorFrameDelegate {
    fn title(&mut self, _cx: &mut ViewContext<FrameView<Self>>) -> &str {
        "Effect Graph Editor"
    }

    fn render_content(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> impl IntoElement {
        container(ContainerKind::Element, cx)
            .size_full()
            .border_color(cx.theme().frame_header_border)
            .child(self.graph_editor.clone())
    }
}
