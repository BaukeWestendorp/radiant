use flow::ProcessingContext;
use gpui::{App, AsyncApp, Entity, ReadGlobal, Timer};
use show::{
    Show,
    asset::{AssetId, Cue, EffectGraphDef, EffectGraphState, Executor},
};
use std::time::Duration;

const INTERVAL: Duration = Duration::from_millis(16);

pub fn start(multiverse: Entity<dmx::Multiverse>, cx: &mut App) {
    cx.spawn(async move |cx: &mut AsyncApp| {
        loop {
            cx.update(|cx| {
                if let Err(err) = generate_multiverse(&multiverse, cx) {
                    log::error!("Failed to generate DMX output: {err}")
                }
            })
            .expect("should update context");

            Timer::after(INTERVAL).await;
        }
    })
    .detach();
}

fn generate_multiverse(multiverse: &Entity<dmx::Multiverse>, cx: &mut App) -> anyhow::Result<()> {
    set_default_values(multiverse, cx)?;

    let executor_ids = Show::global(cx).assets.executors.keys().cloned().collect::<Vec<_>>();
    for id in executor_ids {
        process_executor(&id, multiverse, cx);
    }

    Ok(())
}

fn set_default_values(multiverse: &Entity<dmx::Multiverse>, cx: &mut App) -> anyhow::Result<()> {
    let patch = Show::global(cx).patch.read(cx);

    let mut new_multiverse = dmx::Multiverse::new();

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

fn process_cue(id: AssetId<Cue>, multiverse: &Entity<dmx::Multiverse>, cx: &mut App) {
    let show = Show::global(cx);
    let cue = show.assets.cues.get(&id).unwrap().read(cx);

    let Some(effect_graph_id) = cue.data.effect_graph else { return };
    let Some(effect_graph) = show.assets.effect_graphs.get(&effect_graph_id).cloned() else {
        log::warn!("No effect graph to process!");
        return;
    };

    let Some(fixture_group_id) = cue.data.fixture_group else { return };
    let Some(fixture_group) = show.assets.fixture_groups.get(&fixture_group_id).cloned() else {
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
}

fn process_executor(id: &AssetId<Executor>, multiverse: &Entity<dmx::Multiverse>, cx: &mut App) {
    let show = Show::global(cx);

    let Some(executor) = show.assets.executors.get(id) else { return };
    let executor = &executor.read(cx).data;

    let Some(cue_ix) = executor.current_index else { return };

    let Some(sequence_id) = executor.sequence else { return };
    let Some(sequence) = show.assets.sequences.get(&sequence_id) else {
        return;
    };

    let Some(cue_id) = sequence.read(cx).data.cues.get(cue_ix) else { return };

    process_cue(*cue_id, multiverse, cx);
}
