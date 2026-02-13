use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};

use crate::lua::{self, command::Command};
use crate::object::Effect;

const TARGET_FRAME_RATE: f32 = 60.0;

pub struct EffectRunner {
    effect: Effect,

    start_instant: Instant,
    last_frame_instant: Instant,

    lua: mlua::Lua,

    stop: Option<Arc<AtomicBool>>,
}

impl EffectRunner {
    pub fn new(effect: Effect) -> Result<Self> {
        let lua = mlua::Lua::new();

        Ok(Self {
            effect,
            start_instant: Instant::now(),
            last_frame_instant: Instant::now(),
            lua,
            stop: None,
        })
    }

    pub fn start(
        &mut self,
        group: lua::Group,
        command_tx: mpsc::Sender<Command>,
        stop: Arc<AtomicBool>,
    ) -> Result<()> {
        log::debug!("starting effect runner for effect '{}'", self.effect.name());

        self.stop = Some(stop);

        self.start_lifecycle(group, command_tx)?;

        Ok(())
    }

    fn start_lifecycle(
        &mut self,
        group: lua::Group,
        command_tx: mpsc::Sender<Command>,
    ) -> Result<()> {
        let mut update_cx =
            lua::effect::UpdateContext { global_time: 0.0, delta_time: 0.0, global_frame: 0 };

        let globals = self.lua.globals();
        globals.set("radiant", lua::Radiant { group, command_tx })?;

        self.lua.load(self.effect.script()).exec().context("error executing lua")?;

        if let Ok(on_start) = globals.get::<mlua::Function>("on_start") {
            on_start.call::<()>(())?;
        }

        let frame_duration = Duration::from_secs_f32(1.0 / TARGET_FRAME_RATE);
        let mut deadline = Instant::now() + frame_duration;
        while !self.should_stop() {
            self.tick(&mut update_cx);

            if let Ok(on_update) = globals.get::<mlua::Function>("on_update") {
                on_update.call::<()>(update_cx.clone())?;
            }

            spin_sleep::sleep_until(deadline);
            deadline += frame_duration;
        }

        Ok(())
    }

    fn should_stop(&self) -> bool {
        self.stop.as_ref().is_some_and(|stop| stop.load(Ordering::Relaxed))
    }

    fn tick(&mut self, context: &mut lua::effect::UpdateContext) {
        let now = Instant::now();
        context.global_time = now.duration_since(self.start_instant).as_secs_f64();
        context.delta_time = now.duration_since(self.last_frame_instant).as_secs_f64();
        context.global_frame += 1;
        self.last_frame_instant = now;
    }

    pub fn stop(&mut self) -> Result<()> {
        if let Some(stop) = &self.stop {
            stop.store(true, Ordering::SeqCst);
        }
        Ok(())
    }
}
