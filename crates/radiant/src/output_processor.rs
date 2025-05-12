use flow::ProcessingContext;
use gpui::{App, AsyncApp, Entity, ReadGlobal, Timer};
use show::{
    Show,
    assets::{AssetId, EffectGraph, EffectGraphDef, FixtureGroup, State},
};
use std::time::Duration;

const INTERVAL: Duration = Duration::from_millis(16);

pub fn start(multiverse: Entity<dmx::Multiverse>, cx: &mut App) {
    cx.spawn(async move |cx: &mut AsyncApp| {
        loop {
            cx.update(|cx| {
                if let Err(err) = set_default_values(&multiverse, cx) {
                    log::error!("Error occured while setting default DMX values: {err}",);
                };

                process_effect_graphs(&multiverse, cx);
            })
            .expect("should update context");

            Timer::after(INTERVAL).await;
        }
    })
    .detach();
}

fn process_effect_graphs(multiverse: &Entity<dmx::Multiverse>, cx: &mut App) {
    let effect_graph_id: AssetId<EffectGraph> = AssetId::new(1);
    let fixture_group_id: AssetId<FixtureGroup> = AssetId::new(101);

    let show = Show::global(cx);
    let Some(effect_graph) = show.assets.effect_graphs.get(&effect_graph_id).cloned() else {
        log::warn!("No effect graph to process!");
        return;
    };

    let Some(fixture_group) = show.assets.fixture_groups.get(&fixture_group_id).cloned() else {
        log::warn!("No fixture group to process!");
        return;
    };
    let fixture_group_len = fixture_group.read(cx).data.fixtures.len();

    for ix in 0..fixture_group_len {
        let mut pcx = ProcessingContext::<EffectGraphDef>::new(State {
            multiverse: multiverse.clone(),
            fixture_group: fixture_group.clone(),
            fixture_id_index: Some(ix),
        });

        effect_graph.update(cx, |effect_graph, cx| {
            effect_graph.data.process(&mut pcx, cx);
        });
    }
}

fn set_default_values(multiverse: &Entity<dmx::Multiverse>, cx: &mut App) -> anyhow::Result<()> {
    let patch = Show::global(cx).patch.read(cx);

    let mut new_multiverse = dmx::Multiverse::new();

    // Set default DMX values
    for fixture in patch.fixtures() {
        for channel in &fixture.dmx_mode(patch).dmx_channels {
            let Some((_, channel_function)) = channel.initial_function() else {
                continue;
            };

            let Some(offsets) = &channel.offset else { continue };

            let default_bytes = match &channel_function.default.bytes().get() {
                1 => channel_function.default.to_u8().to_be_bytes().to_vec(),
                2 => channel_function.default.to_u16().to_be_bytes().to_vec(),
                _ => panic!("Unsupported default value size"),
            };

            for (i, offset) in offsets.iter().enumerate() {
                let default = dmx::Value(default_bytes[i]);
                let address = fixture.address().with_channel_offset(*offset as u16 - 1)?;
                new_multiverse.set_value(&address, default);
            }
        }
    }

    multiverse.update(cx, |m, cx| {
        *m = new_multiverse;
        cx.notify();
    });

    Ok(())
}
