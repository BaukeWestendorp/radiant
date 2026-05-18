mod effect;
mod fixture_id;
mod parameter;

pub use effect::*;
pub use fixture_id::*;
pub use parameter::*;

pub fn init_globals(lua: &mlua::Lua) -> mlua::Result<()> {
    lua.globals().set("Parameter", ParameterFactory)?;
    lua.globals().set("Value", ParameterValueFactory)?;
    Ok(())
}
