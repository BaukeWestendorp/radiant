mod fixture_id;
mod parameter;

pub use fixture_id::*;

use crate::lua::rd::parameter::{ParameterFactory, ParameterValueFactory};

pub fn init_globals(lua: &mlua::Lua) -> mlua::Result<()> {
    lua.globals().set("Parameter", ParameterFactory)?;
    lua.globals().set("Value", ParameterValueFactory)?;
    Ok(())
}
