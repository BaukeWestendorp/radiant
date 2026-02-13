use std::{
    fmt, ops,
    sync::{Arc, RwLock, mpsc},
};

pub mod command;
pub mod effect;

#[derive(Debug, Clone)]
pub struct Radiant {
    fixtures: Arc<RwLock<Vec<Fixture>>>,
    command_tx: mpsc::Sender<command::Command>,
}

impl Radiant {
    pub fn new(
        fixtures: Arc<RwLock<Vec<Fixture>>>,
        command_tx: mpsc::Sender<command::Command>,
    ) -> Self {
        Self { fixtures, command_tx }
    }

    fn fixtures(&self) -> Vec<Fixture> {
        self.fixtures.read().unwrap().clone()
    }

    fn set_attribute_value(&self, fixture_id: FixtureId, attribute: String, value: f32) {
        let _ =
            self.command_tx.send(command::Command::SetAttributeValue(command::SetAttributeValue {
                fixture_id,
                attribute,
                value,
            }));
    }
}

impl mlua::UserData for Radiant {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("fixtures", |_, this| Ok(this.fixtures()));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "set_attribute_value",
            |_lua, this, (fixture_id, attribute, value): (FixtureId, String, f32)| {
                this.set_attribute_value(fixture_id, attribute, value);
                Ok(())
            },
        );
    }
}

#[derive(Debug, Clone)]
pub struct Fixture {
    pub id: FixtureId,
    pub name: String,
}

impl Fixture {
    pub fn new(id: FixtureId, name: impl Into<String>) -> Self {
        Self { id, name: name.into() }
    }
}

impl mlua::UserData for Fixture {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("id", |_, this| Ok(this.id.to_string()));
        fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixtureId(pub(crate) zeevonk::project::stage::FixtureId);

impl FixtureId {
    fn parse(s: impl AsRef<str>) -> mlua::Result<Self> {
        let inner = s
            .as_ref()
            .parse::<zeevonk::project::stage::FixtureId>()
            .map_err(mlua::Error::external)?;
        Ok(Self(inner))
    }
}

impl ops::Deref for FixtureId {
    type Target = zeevonk::project::stage::FixtureId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for FixtureId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl mlua::FromLua for FixtureId {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::String(s) = value else {
            return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "FixtureId".to_string(),
                message: Some("expected string".into()),
            });
        };

        Self::parse(s.to_str()?)
    }
}

impl mlua::IntoLua for FixtureId {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        Ok(mlua::Value::String(lua.create_string(self.to_string())?))
    }
}
