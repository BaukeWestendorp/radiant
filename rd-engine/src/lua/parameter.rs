use std::str::FromStr;

use zeevonk::{AttributeName, value::ClampedValue};

use crate::{Parameter, ParameterValue};

impl mlua::UserData for Parameter {}

impl mlua::FromLua for Parameter {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(ud) => {
                if let Ok(param) = ud.borrow::<Self>() {
                    Ok(param.clone())
                } else {
                    Err(mlua::Error::FromLuaConversionError {
                        from: "UserData",
                        to: "Parameter".to_string(),
                        message: Some("Failed to borrow Parameter userdata".to_string()),
                    })
                }
            }
            mlua::Value::Table(table) => {
                let param_type: Option<String> = table.get("type")?;
                match param_type.as_deref() {
                    Some("dimmer") => {
                        let value: ParameterValue = table.get("value")?;
                        Ok(Parameter::dimmer(value))
                    }
                    Some("pan") => {
                        let value: ParameterValue = table.get("value")?;
                        Ok(Parameter::pan(value))
                    }
                    Some("tilt") => {
                        let value: ParameterValue = table.get("value")?;
                        Ok(Parameter::tilt(value))
                    }
                    Some("raw") => {
                        let attr_string = table.get::<String>("attribute")?;
                        let attr = AttributeName::from_str(&attr_string).unwrap();
                        let value: ParameterValue = table.get("value")?;
                        Ok(Parameter::raw(attr, value))
                    }
                    _ => Err(mlua::Error::FromLuaConversionError {
                        from: "Table",
                        to: "Parameter".to_string(),
                        message: Some("Unknown parameter type".to_string()),
                    }),
                }
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Parameter".to_string(),
                message: Some("Unsupported Lua value for Parameter".to_string()),
            }),
        }
    }
}

pub struct ParameterFactory;

impl mlua::UserData for ParameterFactory {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("dimmer", |_, val: ParameterValue| Ok(Parameter::dimmer(val)));
        methods.add_function("pan", |_, val: ParameterValue| Ok(Parameter::pan(val)));
        methods.add_function("tilt", |_, val: ParameterValue| Ok(Parameter::tilt(val)));
        methods.add_function("raw", |_, (attr_string, value): (String, ParameterValue)| {
            let attr = zeevonk::AttributeName::from_str(&attr_string)
                .map_err(|e| mlua::Error::external(format!("Invalid attribute: {:?}", e)))?;
            Ok(Parameter::raw(attr, value))
        });
    }
}

impl mlua::UserData for ParameterValue {}

impl mlua::FromLua for ParameterValue {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(ud) => {
                if let Ok(param) = ud.borrow::<Self>() {
                    Ok(param.clone())
                } else {
                    Err(mlua::Error::FromLuaConversionError {
                        from: "UserData",
                        to: "Parameter".to_string(),
                        message: Some("Failed to borrow Parameter userdata".to_string()),
                    })
                }
            }
            mlua::Value::Table(table) => {
                let param_type: Option<String> = table.get("type")?;
                match param_type.as_deref() {
                    Some("channel_set") => {
                        todo!();
                    }
                    Some("physical") => {
                        let value: f32 = table.get("value")?;
                        Ok(ParameterValue::Physical(value))
                    }
                    Some("percent") => {
                        let value: f32 = table.get("value")?;
                        Ok(ParameterValue::Clamped(ClampedValue::new(value / 100.0)))
                    }
                    Some("clamped") => {
                        let value: f32 = table.get("value")?;
                        Ok(ParameterValue::Clamped(ClampedValue::new(value)))
                    }
                    Some("dmx") => {
                        todo!();
                    }
                    _ => Err(mlua::Error::FromLuaConversionError {
                        from: "Table",
                        to: "Parameter".to_string(),
                        message: Some("Unknown parameter value type".to_string()),
                    }),
                }
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "ParameterValue".to_string(),
                message: Some("Unsupported Lua value for ParameterValue".to_string()),
            }),
        }
    }
}

pub struct ParameterValueFactory;

impl mlua::UserData for ParameterValueFactory {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        // methods.add_function("channel_set", |_, name: String| todo!());
        methods.add_function("physical", |_, value: f32| Ok(ParameterValue::Physical(value)));
        methods.add_function("percent", |_, value: f32| {
            Ok(ParameterValue::Clamped(ClampedValue::new(value / 100.0)))
        });
        methods.add_function("clamped", |_, value: f32| {
            Ok(ParameterValue::Clamped(ClampedValue::new(value)))
        });
        // methods.add_function("dmx", |_, value: u32| todo!());
    }
}
