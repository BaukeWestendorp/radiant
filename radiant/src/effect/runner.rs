use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock, mpsc};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};

use crate::lua::{self, command::Command};
use crate::object::Effect;

const TARGET_FRAME_RATE: f32 = 60.0;

pub struct EffectRunner {
    effect: Effect,
    lua: mlua::Lua,

    start_instant: Instant,
    last_frame_instant: Instant,

    stop: Option<Arc<AtomicBool>>,
}

impl EffectRunner {
    pub fn new(effect: Effect) -> Result<Self> {
        Ok(Self {
            effect,
            lua: mlua::Lua::new(),
            start_instant: Instant::now(),
            last_frame_instant: Instant::now(),
            stop: None,
        })
    }

    pub fn start(
        &mut self,
        fixtures: Arc<RwLock<Vec<lua::Fixture>>>,
        command_tx: mpsc::Sender<Command>,
        stop: Arc<AtomicBool>,
    ) -> Result<()> {
        log::debug!("starting effect runner for effect '{}'", self.effect.name());

        self.stop = Some(stop);

        let globals = self.init_lua(fixtures, command_tx)?;
        self.call_on_start(&globals)?;
        self.run_update_loop(&globals);

        Ok(())
    }

    fn init_lua(
        &mut self,
        fixtures: Arc<RwLock<Vec<lua::Fixture>>>,
        command_tx: mpsc::Sender<Command>,
    ) -> Result<mlua::Table> {
        let globals = self.lua.globals();
        globals.set("radiant", lua::Radiant::new(fixtures, command_tx))?;

        self.lua.load(self.effect.script()).exec().context("error executing lua")?;

        Ok(globals)
    }

    fn call_on_start(&self, globals: &mlua::Table) -> Result<()> {
        if let Ok(on_start) = globals.get::<mlua::Function>("on_start") {
            on_start.call::<()>(())?;
        }
        Ok(())
    }

    fn run_update_loop(&mut self, globals: &mlua::Table) {
        let frame_duration = Duration::from_secs_f32(1.0 / TARGET_FRAME_RATE);
        let mut next_deadline = Instant::now() + frame_duration;

        let mut cx = lua::effect::UpdateContext::new();

        while !self.should_stop() {
            cx = self.tick(cx);

            if let Ok(on_update) = globals.get::<mlua::Function>("on_update") {
                if let Err(err) = on_update.call::<()>(cx) {
                    log::error!("error in on_update for effect '{}': {}", self.effect.name(), err);
                    break;
                }
            }

            spin_sleep::sleep_until(next_deadline);
            next_deadline += frame_duration;
        }
    }

    fn should_stop(&self) -> bool {
        self.stop.as_ref().is_some_and(|stop| stop.load(Ordering::Relaxed))
    }

    fn tick(&mut self, cx: lua::effect::UpdateContext) -> lua::effect::UpdateContext {
        let now = Instant::now();

        let global_time = now.duration_since(self.start_instant).as_secs_f64();
        let delta_time = now.duration_since(self.last_frame_instant).as_secs_f64();

        self.last_frame_instant = now;

        cx.next_frame(global_time, delta_time)
    }
}
