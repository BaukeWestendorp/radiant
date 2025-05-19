use crate::pipeline::Pipeline;
use crate::show::Show;
use crate::show::asset::{AssetId, Cue, Executor, effect_graph};
use dmx::Multiverse;
use flow::ProcessingContext;
use gpui::{App, AppContext, AsyncApp, Entity, ReadGlobal};
use std::time::Duration;

const INTERVAL: Duration = Duration::from_millis(16);

pub fn start(multiverse: Entity<Multiverse>, cx: &App) {
    cx.spawn(async move |cx: &mut AsyncApp| {
        let pipeline = cx.new(|_| Pipeline::new(multiverse.clone())).unwrap();

        loop {
            cx.update(|cx| {
                multiverse.update(cx, |multiverse, cx| {
                    multiverse.clear();
                    cx.notify();
                });

                if let Err(err) = process(&pipeline, cx) {
                    log::error!("Failed to process output: {err}")
                }

                pipeline.update(cx, |pipeline, cx| {
                    if let Err(err) = pipeline.flush(cx) {
                        log::error!("Failed to flush pipeline: {err}")
                    }
                });
            })
            .unwrap();

            gpui::Timer::after(INTERVAL).await;
        }
    })
    .detach();
}

pub fn process(pipeline: &Entity<Pipeline>, cx: &mut App) -> anyhow::Result<()> {
    let executor_ids = Show::global(cx).assets.executors.keys().cloned().collect::<Vec<_>>();
    for id in executor_ids {
        process_executor(&id, pipeline, cx)?;
    }

    Ok(())
}

fn process_cue(id: AssetId<Cue>, pipeline: &Entity<Pipeline>, cx: &mut App) -> anyhow::Result<()> {
    let show = Show::global(cx);
    let cue = show.assets.cues.get(&id).unwrap().read(cx);

    let Some(effect_graph_id) = cue.data.effect_graph else {
        log::warn!("No effect graph to process!");
        return Ok(());
    };
    let Some(effect_graph) = show.assets.effect_graphs.get(&effect_graph_id).cloned() else {
        log::warn!("No effect graph to process!");
        return Ok(());
    };

    let Some(fixture_group_id) = cue.data.fixture_group else {
        log::warn!("No fixture group to process!");
        return Ok(());
    };
    let Some(fixture_group) = show.assets.fixture_groups.get(&fixture_group_id).cloned() else {
        log::warn!("No fixture group to process!");
        return Ok(());
    };

    let total_fixtures = fixture_group.read(cx).data.fixtures.len();
    for ix in 0..total_fixtures {
        effect_graph.update(cx, |effect_graph, cx| {
            let fixtures = fixture_group.read(cx).data.fixtures.clone();
            let mut pcx = ProcessingContext::<effect_graph::Def>::new(
                effect_graph::State::new(fixtures, pipeline.clone(), cx).unwrap(),
            );
            pcx.group_index = ix;
            effect_graph.data.process(&mut pcx, cx);
        });
    }

    Ok(())
}

fn process_executor(
    id: &AssetId<Executor>,
    pipeline: &Entity<Pipeline>,
    cx: &mut App,
) -> anyhow::Result<()> {
    let show = Show::global(cx);

    let Some(executor) = show.assets.executors.get(id) else {
        log::warn!("Executor not found");
        return Ok(());
    };
    let executor = &executor.read(cx).data;

    let Some(cue_ix) = executor.current_index else { return Ok(()) };

    let Some(sequence_id) = executor.sequence else {
        return Ok(());
    };
    let Some(sequence) = show.assets.sequences.get(&sequence_id) else {
        log::warn!("Sequence not found");
        return Ok(());
    };

    let Some(cue_id) = sequence.read(cx).data.cues.get(cue_ix) else {
        log::warn!("Cue not found");
        return Ok(());
    };

    process_cue(*cue_id, pipeline, cx)?;

    Ok(())
}
