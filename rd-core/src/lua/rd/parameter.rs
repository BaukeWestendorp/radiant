use std::str::FromStr;

use zeevonk::attr::Attribute;

use crate::parameter::Parameter;

impl mlua::UserData for Parameter {}

impl mlua::FromLua for Parameter {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(ud) => {
                if let Ok(param) = ud.borrow::<Parameter>() {
                    Ok(*param)
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
                        let value: f32 = table.get("value")?;
                        Ok(Parameter::dimmer(value))
                    }
                    Some("raw") => {
                        let attr_string = table.get::<String>("attribute")?;
                        let attr = Attribute::from_str(&attr_string).unwrap();
                        let value: f32 = table.get("value")?;
                        Ok(Parameter::raw(attr, value))
                    }
                    _ => Err(mlua::Error::FromLuaConversionError {
                        from: "Table",
                        to: "Parameter".to_string(),
                        message: Some("Unknown parameter type".to_string()),
                    }),
                }
            }
            mlua::Value::Number(n) => Ok(Parameter::dimmer(n as f32)),
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
        methods.add_function("dimmer", |_, val: f32| Ok(Parameter::dimmer(val)));
        methods.add_function("raw", |_, (attr_string, value): (String, f32)| {
            let attr = zeevonk::attr::Attribute::from_str(&attr_string)
                .map_err(|e| mlua::Error::external(format!("Invalid attribute: {:?}", e)))?;
            Ok(Parameter::raw(attr, value))
        });
    }
}
