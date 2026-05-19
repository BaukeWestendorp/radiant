use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::Instant,
};

pub mod builtin;

use zeevonk::project::FixtureId;

use crate::{Effect, EffectKind, Object as _, ObjectId, ObjectRegistry, Parameter, RecipeId};

pub struct EffectAgent {
    objects: Arc<ObjectRegistry>,

    running_effects: Mutex<HashMap<(RecipeId, ObjectId), EffectState>>,
}

impl EffectAgent {
    pub fn new(objects: Arc<ObjectRegistry>) -> Self {
        Self { objects, running_effects: Mutex::new(HashMap::new()) }
    }

    pub fn tick(
        &self,
        recipe_id: RecipeId,
        effect: &Effect,
        fixture_ids: &[FixtureId],
        parameters: &mut HashMap<FixtureId, Vec<Parameter>>,
    ) {
        let mut running_effects_guard = self.running_effects.lock().unwrap();
        let effect_state =
            running_effects_guard.entry((recipe_id, effect.id())).or_insert_with(|| EffectState {
                objects: Arc::clone(&self.objects),
                effect_id: effect.id(),
                start_time: Instant::now(),
                last_update_time: Mutex::new(Instant::now()),
                frame_count: AtomicU64::new(0),
            });

        effect_state.tick(fixture_ids, parameters)
    }
}

struct EffectState {
    objects: Arc<ObjectRegistry>,

    effect_id: ObjectId,
    start_time: Instant,
    last_update_time: Mutex<Instant>,
    frame_count: AtomicU64,
}

impl EffectState {
    pub fn tick(
        &self,
        fixture_ids: &[FixtureId],
        parameters: &mut HashMap<FixtureId, Vec<Parameter>>,
    ) {
        let now = Instant::now();
        let mut last_update_time = self.last_update_time.lock().unwrap();
        let delta = now.duration_since(*last_update_time);
        *last_update_time = now;
        let frame_count = self.frame_count.fetch_add(1, Ordering::SeqCst) + 1;

        let context = OnUpdateContext {
            time_seconds: now.duration_since(self.start_time).as_secs_f64(),
            frame_count,
            delta_time: delta.as_secs_f64(),

            fixture_ids,

            parameters,
        };

        if let Some(effect) = self.objects.get::<Effect>(self.effect_id) {
            match effect.kind() {
                EffectKind::Builtin(builtin) => {
                    builtin.call_on_update(context);
                }
            }
        }
    }
}

pub struct OnUpdateContext<'a> {
    time_seconds: f64,
    frame_count: u64,
    delta_time: f64,

    fixture_ids: &'a [FixtureId],

    parameters: &'a mut HashMap<FixtureId, Vec<Parameter>>,
}

impl<'a> OnUpdateContext<'a> {
    pub fn time_seconds(&self) -> f64 {
        self.time_seconds
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }

    pub fn fixture_ids(&self) -> &'a [FixtureId] {
        self.fixture_ids
    }

    pub fn set_parameter(&mut self, fixture_id: &FixtureId, parameter: impl Into<Parameter>) {
        self.parameters.entry(*fixture_id).or_insert_with(Vec::new).push(parameter.into());
    }
}
