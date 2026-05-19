mod effect;
mod fixture_id;
mod parameter;

pub use effect::OnUpdateContext;

pub fn init_globals(lua: &mlua::Lua) -> mlua::Result<()> {
    lua.globals().set("Parameter", parameter::ParameterFactory)?;
    lua.globals().set("Value", parameter::ParameterValueFactory)?;
    Ok(())
}
