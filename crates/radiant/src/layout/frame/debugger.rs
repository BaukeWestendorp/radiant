use flow::ProcessingContext;
use gpui::*;
use show::assets::EffectGraphAsset;

use crate::dmx_io::DmxIo;

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
        let value = self.effect_graph.update(cx, |effect_graph, cx| {
            let mut pcx = ProcessingContext::new();
            effect_graph.data.process(&mut pcx);

            let dmx_value = dmx::Value(pcx.value as u8);

            let address =
                dmx::Address::new(dmx::UniverseId::new(1).unwrap(), dmx::Channel::new(1).unwrap());
            DmxIo::update_global(cx, |dmx_io, _cx| {
                dmx_io.multiverse.set_value(&address, dmx_value);
            });

            pcx.value
        });

        div().size_full().flex().flex_col().gap_2().p_2().child(format!("graph value: {}", value))
    }
}
