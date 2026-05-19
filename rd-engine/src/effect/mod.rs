use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use uuid::Uuid;

mod runner;

pub use runner::*;

use crate::{Effect, FixtureCollection, ObjectReference, ObjectRegistry, RecipeId};

pub struct EffectAgent {
    objects: Arc<ObjectRegistry>,

    running_effects: RwLock<HashMap<RunningEffectId, Arc<EffectRunner>>>,

    showfile_path: Option<PathBuf>,
}

impl EffectAgent {
    pub fn new(objects: Arc<ObjectRegistry>, showfile_path: Option<PathBuf>) -> Self {
        Self { objects, running_effects: RwLock::new(HashMap::new()), showfile_path }
    }

    pub fn start_or_get_runner(
        &self,
        id: RunningEffectId,
        effect_ref: impl Into<ObjectReference>,
        fixture_collection: impl Into<FixtureCollection>,
    ) -> anyhow::Result<Arc<EffectRunner>> {
        // Try to get the runner if it already exists.
        {
            let running_effects = self.running_effects.read().unwrap();
            if let Some(runner) = running_effects.get(&id) {
                return Ok(Arc::clone(runner));
            }
        }

        // Otherwise, start a new runner.
        let effect_ref = effect_ref.into();
        let fixture_collection = fixture_collection.into();

        let Some(effect) = self.objects.get::<Effect>(effect_ref) else {
            log::warn!("effect not found in registry: {:?}", effect_ref);
            anyhow::bail!("object not found: {effect_ref:?}");
        };

        let runner = Arc::new(EffectRunner::new(
            id,
            effect,
            fixture_collection.into(),
            Arc::clone(&self.objects),
            self.showfile_path.as_ref(),
        )?);

        self.running_effects.write().unwrap().insert(id, runner.clone());

        Ok(runner)
    }

    pub fn start_runner(
        &self,
        id: RunningEffectId,
        effect_ref: impl Into<ObjectReference>,
        fixture_collection: impl Into<FixtureCollection>,
    ) -> anyhow::Result<Arc<EffectRunner>> {
        let effect_ref = effect_ref.into();
        let fixture_collection = fixture_collection.into();

        let Some(effect) = self.objects.get::<Effect>(effect_ref) else {
            log::warn!("effect not found in registry: {:?}", effect_ref);
            anyhow::bail!("object not found: {effect_ref:?}");
        };

        let runner = Arc::new(EffectRunner::new(
            id,
            effect,
            fixture_collection.into(),
            Arc::clone(&self.objects),
            self.showfile_path.as_ref(),
        )?);

        self.running_effects.write().unwrap().insert(id, runner.clone());

        Ok(runner)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RunningEffectId(Uuid);

impl RunningEffectId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<RecipeId> for RunningEffectId {
    fn from(recipe_id: RecipeId) -> Self {
        Self(recipe_id.0)
    }
}
