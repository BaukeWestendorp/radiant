use flow::ProcessingContext;
use gpui::{App, AsyncApp, Entity, ReadGlobal, Timer};
use show::{
    Show,
    assets::{EffectGraphDef, EffectGraphId, EffectGraphState, FixtureGroupId},
};
use std::time::Duration;

const INTERVAL: Duration = Duration::from_millis(16);

pub fn start(multiverse: Entity<dmx::Multiverse>, cx: &mut App) {
    cx.spawn(async move |cx: &mut AsyncApp| {
        const EFFECT_GRAPH_ID: EffectGraphId = EffectGraphId::new(1);
        const FIXTURE_GROUP_ID: FixtureGroupId = FixtureGroupId::new(101);

        loop {
            cx.update(|cx| {
                let show = Show::global(cx);
                let Some(effect_graph) = show.assets.effect_graphs.get(&EFFECT_GRAPH_ID).cloned()
                else {
                    log::warn!("No effect graph to process!");
                    return;
                };

                let Some(fixture_group) =
                    show.assets.fixture_groups.get(&FIXTURE_GROUP_ID).cloned()
                else {
                    log::warn!("No fixture group to process!");
                    return;
                };
                let fixture_group_len = fixture_group.read(cx).data.fixtures.len();

                for ix in 0..fixture_group_len {
                    let mut pcx = ProcessingContext::<EffectGraphDef>::new(EffectGraphState {
                        multiverse: multiverse.clone(),
                        fixture_group: fixture_group.clone(),
                        fixture_id_index: Some(ix),
                    });

                    effect_graph.update(cx, |effect_graph, cx| {
                        effect_graph.data.process(&mut pcx, cx);
                    });
                }
            })
            .expect("should update context");

            Timer::after(INTERVAL).await;
        }
    })
    .detach();
}
