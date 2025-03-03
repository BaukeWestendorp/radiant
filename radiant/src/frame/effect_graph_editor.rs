use flow::{Graph, frontend::gpui::GraphEditorView};

use gpui::*;

use crate::effect_graph;

pub struct EffectGraphEditor {
    graph_editor_view: Entity<GraphEditorView<effect_graph::GraphDef>>,
}

impl EffectGraphEditor {
    pub fn build(effect_graph: Entity<effect_graph::EffectGraph>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let graph_editor_view = GraphEditorView::build(effect_graph, cx);
            Self { graph_editor_view }
        })
    }

    pub fn graph(&self, cx: &mut App) -> Entity<Graph<effect_graph::GraphDef>> {
        self.graph_editor_view.read(cx).graph(cx).clone()
    }
}

impl Render for EffectGraphEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        div().child(self.graph_editor_view.clone())
    }
}
