use flow::graph::Graph;
use gpui::*;

pub struct EffectGraphEditor {
    _graph: Graph,
}

impl EffectGraphEditor {
    pub fn new() -> Self {
        let graph = crate::graph::get_graph();
        eprintln!("processing...");
        let cx = graph.process();
        dbg!(cx);

        Self { _graph: graph }
    }
}

impl Render for EffectGraphEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        div().child("effect graph editor")
    }
}
