use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use anyhow::Context;
use zeevonk::{
    project::Stage,
    value::{AttributeValues, ClampedValue},
};

use crate::{Cue, Executor, ExecutorContent, ExecutorId, MergeMode, Objects, RecipeContent};

pub struct Compositor {
    cue_list_executor_meta: HashMap<ExecutorId, CueListExecutorMeta>,
}

impl Compositor {
    pub fn new() -> Self {
        Self { cue_list_executor_meta: HashMap::new() }
    }

    pub fn compose(&mut self, objects: &Objects, stage: &Stage) -> anyhow::Result<AttributeValues> {
        let mut output = AttributeValues::new();
        self.compose_cue_list_executors(objects, stage, &mut output)?;
        Ok(output)
    }

    fn compose_cue_list_executors(
        &mut self,
        objects: &Objects,
        stage: &Stage,
        output: &mut AttributeValues,
    ) -> anyhow::Result<()> {
        let mut cue_list_executors = objects
            .executors()
            .filter(|(_, exec)| {
                exec.enabled() && matches!(exec.content(), Some(ExecutorContent::CueList { .. }))
            })
            .collect::<Vec<_>>();

        // Sort by Priority first, then by Activation Time.
        // Oldest/lowest priority executors are evaluated first.
        // Newer/higher priority executors are evaluated later, naturally overwriting LTP channels.
        cue_list_executors.sort_by_key(|(id, exec)| {
            let meta = self.cue_list_executor_meta.entry(*id).or_insert(Default::default());
            match exec.content() {
                Some(ExecutorContent::CueList { priority, .. }) => {
                    (*priority, meta.last_activation.time)
                }
                None => (u32::MIN, Instant::now()),
            }
        });

        for (id, cue_list_executor) in cue_list_executors {
            if let Err(err) =
                self.compose_cue_list_executor(id, cue_list_executor, objects, stage, output)
            {
                log::error!("failed to compose cue list executor {}: {}", id, err);
            };
        }

        Ok(())
    }

    fn compose_cue_list_executor(
        &mut self,
        id: ExecutorId,
        executor: &Executor,
        objects: &Objects,
        stage: &Stage,
        output: &mut AttributeValues,
    ) -> anyhow::Result<()> {
        let meta = self.cue_list_executor_meta.entry(id).or_insert(Default::default());
        let delta_time = Instant::now() - meta.last_composition;

        match executor.content() {
            Some(ExecutorContent::CueList {
                cue_list,
                cue_index,
                merge_mode,
                start_from_previous_cue,
                ..
            }) => {
                let cue_list = objects.cue_lists.get_by_object_id(cue_list)?;
                let current_cue = cue_list.cue(*cue_index)?;

                if meta.last_activation.cue_index != *cue_index {
                    if *start_from_previous_cue && !meta.ever_composed && *cue_index > 0 {
                        meta.last_activation.values_snapshot = match cue_list.cue(cue_index - 1) {
                            Ok(prev_cue) => values_from_cue(prev_cue, objects, stage)?.clone(),
                            Err(_) => AttributeValues::new(),
                        };
                    } else {
                        meta.last_activation.values_snapshot = meta.values.clone();
                    }

                    meta.last_activation.cue_index = *cue_index;
                    meta.last_activation.time = Instant::now();
                    meta.fade_progress = 0.0;
                }

                if current_cue.fade_time() > Duration::ZERO {
                    meta.fade_progress +=
                        delta_time.as_secs_f32() / current_cue.fade_time().as_secs_f32();
                    meta.fade_progress = meta.fade_progress.min(1.0);
                } else {
                    meta.fade_progress = 1.0;
                }

                let mut executor_values = AttributeValues::new();
                for (fixture_id, attribute, target_value) in
                    values_from_cue(current_cue, objects, stage)?.values()
                {
                    let Some(channel_function) = stage
                        .fixture(fixture_id)
                        .with_context(|| format!("fixture {} not found on stage", fixture_id))?
                        .channel_function(attribute)
                    else {
                        continue;
                    };

                    let start_value = meta
                        .last_activation
                        .values_snapshot
                        .get(fixture_id, attribute)
                        .unwrap_or_else(|| channel_function.default());

                    let start_value = start_value.to_clamped_value(channel_function);
                    let target_value = target_value.to_clamped_value(channel_function);
                    let current_value =
                        ClampedValue::lerp(&start_value, &target_value, meta.fade_progress);
                    let current_value =
                        ClampedValue::new(current_value.as_f32() * executor.master());

                    executor_values.set(*fixture_id, attribute.clone(), current_value);

                    match merge_mode {
                        MergeMode::Ltp => {
                            output.set(*fixture_id, attribute.clone(), current_value);
                        }
                        MergeMode::Htp => {
                            let existing_value = output
                                .get(fixture_id, attribute)
                                .map(|v| v.to_clamped_value(channel_function))
                                .unwrap_or_else(|| ClampedValue::new(0.0));

                            let merged = if current_value > existing_value {
                                current_value
                            } else {
                                existing_value
                            };

                            output.set(*fixture_id, attribute.clone(), merged);
                        }
                    }
                }

                meta.values = executor_values;
                meta.ever_composed = true;
            }
            None => {}
        }

        meta.last_composition = Instant::now();

        Ok(())
    }
}

fn values_from_cue(cue: &Cue, objects: &Objects, stage: &Stage) -> anyhow::Result<AttributeValues> {
    let mut values = AttributeValues::new();
    for recipe in cue.recipes() {
        for fixture_id in recipe.fixtures().fixture_ids(objects, stage)? {
            match recipe.content() {
                RecipeContent::Static(recipe_values) => {
                    for (attribute, value) in recipe_values {
                        values.set(*fixture_id, attribute.clone(), *value);
                    }
                }
                RecipeContent::Preset(preset_id) => {
                    let preset = match objects.preset_by_object_id(preset_id) {
                        Ok(preset) => preset,
                        Err(err) => {
                            log::error!("{err}");
                            continue;
                        }
                    };

                    // Universal values.
                    for (attribute, value) in preset.universal() {
                        values.set(*fixture_id, attribute.clone(), *value);
                    }

                    // Global values.
                    if let Some(fixture_type_id) =
                        stage.fixture(fixture_id).map(|f| f.gdtf_fixture_type_id())
                        && !preset.global().is_empty()
                    {
                        if let Some(global_values) = preset.global().get(&fixture_type_id) {
                            for (attribute, value) in global_values {
                                values.set(*fixture_id, attribute.clone(), *value);
                            }
                        }
                    }

                    // Selective values.
                    if let Some(selective_values) = preset.selective().get(fixture_id) {
                        for (attribute, value) in selective_values {
                            values.set(*fixture_id, attribute.clone(), *value);
                        }
                    }
                }
            }
        }
    }
    Ok(values)
}

struct CueListExecutorMeta {
    pub last_activation: CueListExecutorActivation,
    pub fade_progress: f32,
    pub last_composition: Instant,
    pub values: AttributeValues,
    pub ever_composed: bool,
}

impl Default for CueListExecutorMeta {
    fn default() -> Self {
        Self {
            values: AttributeValues::new(),
            last_activation: CueListExecutorActivation {
                time: Instant::now(),
                cue_index: 0,
                values_snapshot: AttributeValues::new(),
            },
            fade_progress: 0.0,
            last_composition: Instant::now(),
            ever_composed: false,
        }
    }
}

struct CueListExecutorActivation {
    pub time: Instant,
    pub cue_index: usize,
    pub values_snapshot: AttributeValues,
}
