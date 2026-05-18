use mlua::ObjectLike;
use zeevonk::project::FixtureId;

use crate::engine::{
    Effect, EffectOptionValue, FixtureCollection, ObjectRegistry, Parameter, RunningEffectId, lua,
};

use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::Instant,
};

pub struct EffectRunner {
    id: RunningEffectId,
    fixture_collection: FixtureCollection,

    lua: mlua::Lua,

    start_time: Instant,
    last_update_time: Mutex<Instant>,
    frame_count: AtomicU64,

    objects: Arc<ObjectRegistry>,
}

impl EffectRunner {
    pub fn new(
        id: RunningEffectId,
        effect: &Effect,
        fixture_collection: FixtureCollection,
        objects: Arc<ObjectRegistry>,
        showfile_path: Option<&PathBuf>,
    ) -> anyhow::Result<Self> {
        let lua_source = effect.load_lua_source(showfile_path)?;
        let lua = mlua::Lua::new();
        lua::init_globals(&lua)?;
        if let Err(err) = lua.load(lua_source).exec() {
            log::error!("failed to execute Lua script: {}", err);
        }

        let now = Instant::now();
        Ok(EffectRunner {
            id,

            fixture_collection,

            lua,

            start_time: now,
            last_update_time: Mutex::new(now),
            frame_count: AtomicU64::new(0),

            objects,
        })
    }

    pub fn id(&self) -> RunningEffectId {
        self.id
    }

    pub fn call_on_update(
        self: &Arc<Self>,
        options: HashMap<String, EffectOptionValue>,
        parameters: Arc<Mutex<HashMap<FixtureId, Vec<Parameter>>>>,
    ) {
        let now = Instant::now();
        let mut last_update_time = self.last_update_time.lock().unwrap();
        let delta = now.duration_since(*last_update_time);
        *last_update_time = now;
        let frame_count = self.frame_count.fetch_add(1, Ordering::SeqCst) + 1;

        let fixture_ids = self
            .fixture_collection
            .to_fixture_ids(&self.objects)
            .iter()
            .copied()
            .map(Into::into)
            .collect::<Vec<_>>();

        let context = lua::OnUpdateContext {
            time_seconds: now.duration_since(self.start_time).as_secs_f64(),
            frame_count,
            delta_time: delta.as_secs_f64(),

            fixture_ids,
            options,

            parameters,
        };

        if let Err(err) = self.lua.globals().call_function::<()>("on_update", context) {
            log::error!("Error calling Lua 'on_update': {}", err);
        }
    }
}
