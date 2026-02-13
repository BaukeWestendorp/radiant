use anyhow::Result;

use crate::object::Effect;

pub struct EffectRunner {
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
