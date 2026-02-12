use std::collections::HashMap;

use anyhow::Result;
use gpui::{Context, Entity};

use crate::object::{Effect, EffectId};

pub struct EffectEngine {
    runners: HashMap<EffectId, EffectRunner>,
}

impl EffectEngine {
    pub fn new(effects: Entity<HashMap<EffectId, Effect>>, cx: &mut Context<Self>) -> Self {
        let mut this = Self { runners: HashMap::new() };

        for (effect_id, effect) in effects.read(cx) {
            this.register_effect(*effect_id, effect.clone()).unwrap();
        }
        cx.observe(&effects, {
            move |this, effects, cx| {
                for effect_id in this.runners.keys().cloned().collect::<Vec<_>>() {
                    this.unregister_effect(effect_id).unwrap();
                }

                for (effect_id, effect) in effects.read(cx) {
                    this.register_effect(*effect_id, effect.clone()).unwrap();
                }
            }
        })
        .detach();

        this
    }

    pub fn register_effect(&mut self, effect_id: EffectId, effect: Effect) -> Result<()> {
        let mut runner = EffectRunner::new(effect)?;
        runner.start()?;
        self.runners.insert(effect_id, runner);
        Ok(())
    }

    pub fn unregister_effect(&mut self, effect_id: EffectId) -> Result<()> {
        if let Some(mut runner) = self.runners.remove(&effect_id) {
            runner.stop()?;
        }

        Ok(())
    }

    pub fn start_effect(&mut self, effect_id: EffectId) -> Result<()> {
        if let Some(runner) = self.runners.get_mut(&effect_id) {
            runner.start()?;
        }
        Ok(())
    }

    pub fn stop_effect(&mut self, effect_id: EffectId) -> Result<()> {
        if let Some(runner) = self.runners.get_mut(&effect_id) {
            runner.stop()?;
        }
        Ok(())
    }
}

struct EffectRunner {
    effect: Effect,
}

impl EffectRunner {
    pub fn new(effect: Effect) -> Result<Self> {
        Ok(Self { effect })
    }

    pub fn start(&mut self) -> Result<()> {
        log::debug!("starting effect runner for effect '{}'", self.effect.name());
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        log::debug!("stopping effect runner for effect '{}'", self.effect.name());
        Ok(())
    }
}
