use std::{fmt, ops, sync::mpsc};

use crate::lua::command::Command;

pub mod effect;

pub mod command;

#[derive(Debug, Clone)]
pub struct Radiant {
    pub group: Group,

    pub command_tx: mpsc::Sender<Command>,
}

impl mlua::UserData for Radiant {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("group", |_, this| Ok(this.group.clone()));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "set_attribute_value",
            |_lua, this, (fixture_id, attribute, value): (FixtureId, String, f32)| {
                let _ = this.command_tx.send(Command::SetAttributeValue {
                    fixture_id,
                    attribute,
                    value,
                });

                Ok(())
            },
        );
    }
}

#[derive(Debug, Clone)]
pub struct Group {
    pub id: u32,
    pub name: String,
    pub fixtures: Vec<Fixture>,
}

impl mlua::UserData for Group {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("id", |_, this| Ok(this.id));
        fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));
        fields.add_field_method_get("fixtures", |_, this| Ok(this.fixtures.clone()));
    }
}

#[derive(Debug, Clone)]
pub struct Fixture {
    pub id: FixtureId,
    pub name: String,
}

impl mlua::UserData for Fixture {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("id", |_, this| Ok(this.id.to_string()));
        fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixtureId(pub(crate) zeevonk::project::stage::FixtureId);

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
        match value {
            mlua::Value::String(s) => {
                let str_val = s.to_str()?;

                let inner = str_val
                    .parse::<zeevonk::project::stage::FixtureId>()
                    .map_err(|e| mlua::Error::external(e))?;

                Ok(FixtureId(inner))
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "FixtureId".to_string(),
                message: Some("expected string".into()),
            }),
        }
    }
}

impl mlua::IntoLua for FixtureId {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        Ok(mlua::Value::String(lua.create_string(self.to_string())?))
    }
}
