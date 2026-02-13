use std::collections::HashMap;
use std::str::FromStr as _;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock, mpsc};
use std::thread::{self, JoinHandle};

use anyhow::{Context as _, Result};
use gpui::{App, Context, Entity};
use zeevonk::project::stage::FixtureId;
use zeevonk::{Zeevonk, attr::Attribute, value::AttributeValues};

use crate::{
    app::state::AppState,
    effect::runner::EffectRunner,
    lua::{self, command::Command},
    object::{Effect, EffectId},
};

struct RunnerThread {
    stop: Arc<AtomicBool>,
    handle: JoinHandle<()>,
}

pub struct EffectEngine {
    effects: HashMap<EffectId, Effect>,
    running: HashMap<EffectId, RunnerThread>,

    command_tx: mpsc::Sender<Command>,
}

impl EffectEngine {
    pub fn new(
        effects: Entity<HashMap<EffectId, Effect>>,
        zeevonk: Arc<Zeevonk>,
        cx: &mut Context<Self>,
    ) -> Self {
        let (command_tx, command_rx) = mpsc::channel();

        let mut this = Self { effects: HashMap::new(), running: HashMap::new(), command_tx };

        // Initialize effect registry from current show state.
        for (effect_id, effect) in effects.read(cx) {
            if let Err(err) = this.register_effect(*effect_id, effect.clone()) {
                log::error!("failed to register effect {:?}: {}", effect_id, err);
            }
        }

        cx.observe(&effects, {
            move |this, effects, _cx| {
                for effect_id in this.running.keys().cloned().collect::<Vec<_>>() {
                    if let Err(err) = this.stop_effect(effect_id) {
                        log::error!("failed to stop effect {:?}: {}", effect_id, err);
                    }
                }

                this.effects.clear();

                for (effect_id, effect) in effects.read(_cx) {
                    if let Err(err) = this.register_effect(*effect_id, effect.clone()) {
                        log::error!("failed to register effect {:?}: {}", effect_id, err);
                    }
                }
            }
        })
        .detach();

        thread::Builder::new()
            .name("lua_command_handler".to_string())
            .spawn({
                move || {
                    while let Ok(command) = command_rx.recv() {
                        match command {
                            Command::SetAttributeValue { fixture_id, attribute, value } => {
                                let mut values = AttributeValues::new();
                                values.set(
                                    *fixture_id,
                                    Attribute::from_str(&attribute).unwrap(),
                                    value,
                                );
                                zeevonk.set_attribute_values(values);
                            }
                        }
                    }
                }
            })
            .expect("failed to spawn lua_command_handler thread");

        this
    }

    pub fn register_effect(&mut self, effect_id: EffectId, effect: Effect) -> Result<()> {
        self.effects.insert(effect_id, effect);
        Ok(())
    }

    pub fn unregister_effect(&mut self, effect_id: EffectId) -> Result<()> {
        let _ = self.stop_effect(effect_id);
        self.effects.remove(&effect_id);
        Ok(())
    }

    pub fn start_effect(
        &mut self,
        effect_id: EffectId,
        fixture_ids: Entity<Vec<FixtureId>>,
        cx: &mut App,
    ) -> Result<()> {
        if self.running.contains_key(&effect_id) {
            self.stop_effect(effect_id)?;
        }

        let effect = self
            .effects
            .get(&effect_id)
            .cloned()
            .with_context(|| format!("effect {:?} not registered", effect_id))?;

        let command_tx = self.command_tx.clone();
        let stop = Arc::new(AtomicBool::new(false));
        let stop_in_thread = Arc::clone(&stop);

        let fixtures = Arc::new(RwLock::new(
            fixture_ids
                .read(cx)
                .iter()
                .filter_map(|fixture_id| {
                    let fixture = AppState::zeevonk(cx).project().stage().fixture(fixture_id)?;
                    Some(lua::Fixture {
                        id: lua::FixtureId(*fixture_id),
                        name: fixture.name().to_string(),
                    })
                })
                .collect(),
        ));

        cx.observe(&fixture_ids, {
            let fixtures = Arc::clone(&fixtures);
            move |fixture_ids, cx| {
                let new_fixtures = fixture_ids
                    .read(cx)
                    .iter()
                    .filter_map(|fixture_id| {
                        let fixture =
                            AppState::zeevonk(cx).project().stage().fixture(fixture_id)?;
                        Some(lua::Fixture {
                            id: lua::FixtureId(*fixture_id),
                            name: fixture.name().to_string(),
                        })
                    })
                    .collect();

                *fixtures.write().unwrap() = new_fixtures;
            }
        })
        .detach();

        let handle = thread::Builder::new()
            .name(format!("effect_runner_{:?}", effect_id))
            .spawn(move || {
                let mut runner = match EffectRunner::new(effect) {
                    Ok(runner) => runner,
                    Err(err) => {
                        log::error!("failed to create effect runner {:?}: {}", effect_id, err);
                        return;
                    }
                };

                if let Err(err) = runner.start(fixtures, command_tx, Arc::clone(&stop_in_thread)) {
                    log::error!("effect runner {:?} exited with error: {}", effect_id, err);
                }

                stop_in_thread.store(true, Ordering::SeqCst);
            })
            .context("failed to spawn effect runner thread")?;

        self.running.insert(effect_id, RunnerThread { stop, handle });

        Ok(())
    }

    pub fn stop_effect(&mut self, effect_id: EffectId) -> Result<()> {
        let Some(running) = self.running.remove(&effect_id) else {
            return Ok(());
        };

        running.stop.store(true, Ordering::SeqCst);

        let _ = running.handle.join();
        Ok(())
    }

    pub fn effect_running(&self, effect_id: EffectId) -> bool {
        self.running.contains_key(&effect_id)
    }
}
