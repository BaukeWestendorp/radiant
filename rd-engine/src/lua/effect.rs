use zeevonk::project::FixtureId;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{EffectOptionValue, Parameter, lua};

pub struct OnUpdateContext {
    pub time_seconds: f64,
    pub frame_count: u64,
    pub delta_time: f64,

    pub fixture_ids: Vec<lua::fixture_id::FixtureId>,
    pub options: HashMap<String, EffectOptionValue>,

    pub parameters: Arc<Mutex<HashMap<FixtureId, Vec<Parameter>>>>,
}

impl mlua::UserData for OnUpdateContext {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("time_seconds", |_, this| Ok(this.time_seconds));
        fields.add_field_method_get("frame_count", |_, this| Ok(this.frame_count));
        fields.add_field_method_get("delta_time", |_, this| Ok(this.delta_time));

        fields.add_field_method_get("fixture_ids", |_, this| Ok(this.fixture_ids.clone()));
        fields.add_field_method_get("options", |_, this| Ok(this.options.clone()));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut(
            "set_parameter",
            |_, this, (fixture_id, parameter): (lua::fixture_id::FixtureId, Parameter)| {
                this.parameters
                    .lock()
                    .unwrap()
                    .entry(*fixture_id)
                    .or_insert_with(Vec::new)
                    .push(parameter);
                Ok(())
            },
        );
    }
}

impl mlua::IntoLua for EffectOptionValue {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        match self {
            EffectOptionValue::Nil => Ok(mlua::Value::Nil),
            EffectOptionValue::Boolean(v) => v.into_lua(lua),
            EffectOptionValue::Integer(v) => v.into_lua(lua),
            EffectOptionValue::Number(v) => v.into_lua(lua),
            EffectOptionValue::String(v) => v.into_lua(lua),
        }
    }
}
