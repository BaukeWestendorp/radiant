use std::time::Duration;

use flow::ProcessingContext;
use gpui::{App, AsyncApp, Entity, ReadGlobal, Timer};
use show::{Show, assets::EffectGraphId};

const INTERVAL: Duration = Duration::from_millis(16);

pub fn start(multiverse: Entity<dmx::Multiverse>, cx: &mut App) {
    cx.spawn(async move |cx: &mut AsyncApp| {
        const EFFECT_GRAPH_ID: EffectGraphId = EffectGraphId::new(1);

        loop {
            cx.update(|cx| {
                let show = Show::global(cx);
                let effect_graph = show.assets.effect_graphs.get(&EFFECT_GRAPH_ID).unwrap().clone();
                let state = effect_graph.update(cx, |effect_graph, _cx| {
                    let mut pcx = ProcessingContext::default();
                    effect_graph.data.process(&mut pcx);
                    pcx.state().clone()
                });

                multiverse.update(cx, |multiverse, cx| {
                    multiverse.set_value(
                        &dmx::Address::new(
                            dmx::UniverseId::new(1).unwrap(),
                            dmx::Channel::new(1).unwrap(),
                        ),
                        dmx::Value(state.value as u8),
                    );
                    cx.notify();
                });
            })
            .expect("should update context");

            Timer::after(INTERVAL).await;
        }
    })
    .detach();
}
