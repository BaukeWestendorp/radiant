use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use crate::{
    effect::runner::EffectRunner,
    object::{Effect, FixtureCollection, Object, ObjectId, ObjectReference, ObjectRegistry},
};

pub mod runner;

pub struct EffectAgent {
    objects: Arc<ObjectRegistry>,

    running_effects: RwLock<HashMap<ObjectId, Arc<EffectRunner>>>,

    showfile_path: Option<PathBuf>,
}

impl EffectAgent {
    pub fn new(objects: Arc<ObjectRegistry>, showfile_path: Option<PathBuf>) -> Self {
        Self { objects, running_effects: RwLock::new(HashMap::new()), showfile_path }
    }

    pub fn runner(&self, effect_ref: impl Into<ObjectReference>) -> Option<Arc<EffectRunner>> {
        let object_id = self.objects.get_id::<Effect>(effect_ref)?;
        self.running_effects.write().unwrap().get(&object_id).cloned()
    }

    pub fn get_or_start_runner(
        &self,
        effect_ref: impl Into<ObjectReference>,
        fixture_collection: impl Into<FixtureCollection>,
    ) -> Result<Arc<EffectRunner>, crate::Error> {
        let effect_ref = effect_ref.into();
        let fixture_collection = fixture_collection.into();

        if let Some(object_id) = self.objects.get_id::<Effect>(effect_ref.clone()) {
            if let Some(runner) = self.running_effects.write().unwrap().get(&object_id).cloned() {
                return Ok(runner);
            }
        }

        self.start_runner(effect_ref.clone(), fixture_collection.clone())?;

        if let Some(object_id) = self.objects.get_id::<Effect>(effect_ref) {
            if let Some(runner) = self.running_effects.write().unwrap().get(&object_id).cloned() {
                return Ok(runner);
            }
        }

        log::warn!("effect runner could not be started or found: {:?}", effect_ref);
        Err(crate::object::Error::ObjectNotFound(effect_ref).into())
    }

    fn start_runner(
        &self,
        effect_ref: impl Into<ObjectReference>,
        fixture_collection: impl Into<FixtureCollection>,
    ) -> Result<(), crate::Error> {
        let effect_ref = effect_ref.into();
        let fixture_collection = fixture_collection.into();

        let Some(effect) = self.objects.get::<Effect>(effect_ref) else {
            log::warn!("effect not found in registry: {:?}", effect_ref);
            return Err(crate::object::Error::ObjectNotFound(effect_ref).into());
        };

        let runner = Arc::new(EffectRunner::new(
            effect,
            fixture_collection.into(),
            Arc::clone(&self.objects),
            self.showfile_path.as_ref(),
        )?);

        self.running_effects.write().unwrap().insert(effect.id(), runner);

        Ok(())
    }
}
