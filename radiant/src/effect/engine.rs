use std::collections::HashMap;
use std::str::FromStr as _;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock, mpsc};
use std::thread::{self, JoinHandle};

use anyhow::{Context as _, Result};
use gpui::{App, Context, Entity};
use zeevonk::Zeevonk;
use zeevonk::attr::Attribute;
use zeevonk::project::FixtureId;
use zeevonk::value::AttributeValues;

use crate::{
    app::state::AppState,
    effect::runner::EffectRunner,
    lua::{self, command, command::Command},
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

        this.sync_effect_registry(effects.clone(), cx);

        cx.observe(&effects, move |this, effects, cx| {
            this.sync_effect_registry(effects.clone(), cx)
        })
        .detach();

        Self::spawn_lua_command_handler(zeevonk, command_rx);

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
        self.stop_effect(effect_id)?;

        let effect = self.effect(effect_id)?;
        let fixtures = Self::tracked_fixtures(&fixture_ids, cx);

        let stop = Arc::new(AtomicBool::new(false));
        let handle = Self::spawn_effect_runner(
            effect_id,
            effect,
            fixtures,
            self.command_tx.clone(),
            Arc::clone(&stop),
        )?;

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

    fn sync_effect_registry(&mut self, effects: Entity<HashMap<EffectId, Effect>>, cx: &mut App) {
        let running_ids = self.running.keys().cloned().collect::<Vec<_>>();
        for effect_id in running_ids {
            if let Err(err) = self.stop_effect(effect_id) {
                log::error!("failed to stop effect {:?}: {}", effect_id, err);
            }
        }

        self.effects.clear();

        for (effect_id, effect) in effects.read(cx) {
            if let Err(err) = self.register_effect(*effect_id, effect.clone()) {
                log::error!("failed to register effect {:?}: {}", effect_id, err);
            }
        }
    }

    fn effect(&self, effect_id: EffectId) -> Result<Effect> {
        self.effects
            .get(&effect_id)
            .cloned()
            .with_context(|| format!("effect {:?} not registered", effect_id))
    }

    fn spawn_lua_command_handler(zeevonk: Arc<Zeevonk>, command_rx: mpsc::Receiver<Command>) {
        thread::Builder::new()
            .name("lua_command_handler".to_string())
            .spawn(move || {
                while let Ok(command) = command_rx.recv() {
                    match command {
                        Command::SetAttributeValue(command::SetAttributeValue {
                            fixture_id,
                            attribute,
                            value,
                        }) => {
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
            })
            .expect("failed to spawn lua_command_handler thread");
    }

    fn tracked_fixtures(
        fixture_ids: &Entity<Vec<FixtureId>>,
        cx: &mut App,
    ) -> Arc<RwLock<Vec<lua::Fixture>>> {
        let fixtures = Arc::new(RwLock::new(Self::fixtures_from_entity(fixture_ids, cx)));

        cx.observe(fixture_ids, {
            let fixtures = Arc::clone(&fixtures);
            move |fixture_ids, cx| {
                let new_fixtures = Self::fixtures_from_entity(&fixture_ids, cx);
                *fixtures.write().unwrap() = new_fixtures;
            }
        })
        .detach();

        fixtures
    }

    fn fixtures_from_entity(
        fixture_ids: &Entity<Vec<FixtureId>>,
        cx: &mut App,
    ) -> Vec<lua::Fixture> {
        fixture_ids
            .read(cx)
            .iter()
            .filter_map(|fixture_id| lua::Fixture::from_zeevonk(*fixture_id, cx))
            .collect()
    }

    fn spawn_effect_runner(
        effect_id: EffectId,
        effect: Effect,
        fixtures: Arc<RwLock<Vec<lua::Fixture>>>,
        command_tx: mpsc::Sender<Command>,
        stop: Arc<AtomicBool>,
    ) -> Result<JoinHandle<()>> {
        thread::Builder::new()
            .name(format!("effect_runner_{:?}", effect_id))
            .spawn(move || {
                let mut runner = match EffectRunner::new(effect) {
                    Ok(runner) => runner,
                    Err(err) => {
                        log::error!("failed to create effect runner {:?}: {}", effect_id, err);
                        stop.store(true, Ordering::SeqCst);
                        return;
                    }
                };

                if let Err(err) = runner.start(fixtures, command_tx, Arc::clone(&stop)) {
                    log::error!("effect runner {:?} exited with error: {}", effect_id, err);
                }

                stop.store(true, Ordering::SeqCst);
            })
            .context("failed to spawn effect runner thread")
    }
}

impl lua::Fixture {
    fn from_zeevonk(fixture_id: FixtureId, cx: &App) -> Option<Self> {
        let fixture = AppState::zeevonk(cx).project().stage().fixture(&fixture_id)?;
        Some(Self::new(lua::FixtureId(fixture_id), fixture.name().to_string()))
    }
}
