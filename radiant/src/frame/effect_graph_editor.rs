use flow::ProcessingContext;

use gpui::*;

use crate::effect_graph::{self, EffectGraph};

pub struct EffectGraphEditor {
    _graph: EffectGraph,
}

impl EffectGraphEditor {
    pub fn new() -> Self {
        let graph = effect_graph::get_graph();
        eprintln!("processing...");
        let mut cx = ProcessingContext::default();
        graph.process(&mut cx);
        dbg!(cx);

        Self { _graph: graph }
    }
}

impl Render for EffectGraphEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        div().child("effect graph editor")
    }
}
