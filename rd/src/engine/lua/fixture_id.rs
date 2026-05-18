use std::{fmt, ops, str::FromStr as _};

use zeevonk::project::IntoFixtureId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixtureId(pub(crate) zeevonk::project::FixtureId);

impl IntoFixtureId for FixtureId {
    fn into_fixture_id(self) -> Option<zeevonk::project::FixtureId> {
        Some(*self)
    }
}

impl Into<FixtureId> for zeevonk::project::FixtureId {
    fn into(self) -> FixtureId {
        FixtureId(self)
    }
}

impl fmt::Display for FixtureId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ops::Deref for FixtureId {
    type Target = zeevonk::project::FixtureId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for FixtureId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl mlua::FromLua for FixtureId {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::String(s) => {
                let inner = zeevonk::project::FixtureId::from_str(&s.to_str()?).map_err(|_| {
                    mlua::Error::FromLuaConversionError {
                        from: "String",
                        to: "FixtureId".to_string(),
                        message: Some("Failed to parse FixtureId from string".to_string()),
                    }
                })?;
                Ok(FixtureId(inner))
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "FixtureId".to_string(),
                message: Some("Expected a string".to_string()),
            }),
        }
    }
}

impl mlua::IntoLua for FixtureId {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        self.0.to_string().into_lua(lua)
    }
}
