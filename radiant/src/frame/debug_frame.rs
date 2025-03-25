use flow::ProcessingContext;
use gpui::*;

use crate::effect_graph::EffectGraph;

pub struct DebugFrame {
    effect_graph: Entity<EffectGraph>,
}

impl DebugFrame {
    pub fn build(
        effect_graph: Entity<EffectGraph>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|_cx| Self { effect_graph })
    }
}

impl Render for DebugFrame {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let value = self.effect_graph.update(cx, |effect_graph, _cx| {
            let mut pcx = ProcessingContext::new();
            effect_graph.process(&mut pcx);
            pcx.value
        });

        div().size_full().flex().flex_col().gap_2().p_2().child(format!("graph value: {}", value))
    }
}
