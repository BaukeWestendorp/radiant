use flow::ProcessingContext;
use gpui::*;
use show::assets::EffectGraphAsset;

pub struct Debugger {
    effect_graph: Entity<EffectGraphAsset>,
}

impl Debugger {
    pub fn new(effect_graph: Entity<EffectGraphAsset>) -> Self {
        Self { effect_graph }
    }
}

impl Render for Debugger {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let value = self.effect_graph.update(cx, |effect_graph, _cx| {
            let mut pcx = ProcessingContext::new();
            effect_graph.data.process(&mut pcx);
            pcx.value
        });

        div().size_full().flex().flex_col().gap_2().p_2().child(format!("graph value: {}", value))
    }
}
