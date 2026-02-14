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
                    Some("pan") => {
                        let value: f32 = table.get("value")?;
                        Ok(Parameter::pan(value))
                    }
                    Some("tilt") => {
                        let value: f32 = table.get("value")?;
                        Ok(Parameter::tilt(value))
                    }
                    Some("rgb") => {
                        let r: f32 = table.get("r")?;
                        let g: f32 = table.get("g")?;
                        let b: f32 = table.get("b")?;
                        Ok(Parameter::rgb(r, g, b))
                    }
                    Some("rgbw") => {
                        let r: f32 = table.get("r")?;
                        let g: f32 = table.get("g")?;
                        let b: f32 = table.get("b")?;
                        let w: f32 = table.get("w")?;
                        Ok(Parameter::rgbw(r, g, b, w))
                    }
                    Some("cmy") => {
                        let c: f32 = table.get("c")?;
                        let m: f32 = table.get("m")?;
                        let y: f32 = table.get("y")?;
                        Ok(Parameter::cmy(c, m, y))
                    }
                    Some("hsb") => {
                        let hue: f32 = table.get("hue")?;
                        let saturation: f32 = table.get("saturation")?;
                        let brightness: f32 = table.get("brightness")?;
                        Ok(Parameter::hsb(hue, saturation, brightness))
                    }
                    Some("cto") => {
                        let value: f32 = table.get("value")?;
                        Ok(Parameter::cto(value))
                    }
                    Some("raw") => {
                        let attr_string = table.get::<String>("attribute")?;
                        let attr = Attribute::from_str(&attr_string).unwrap();
                        let value: f32 = table.get("value")?;
                        Ok(Parameter::raw(attr, value))
                    }
                    Some("wheel") => {
                        let wheel: u8 = table.get("wheel")?;
                        let mut builder = Parameter::wheel(wheel);
                        if let Ok(index) = table.get::<f32>("index") {
                            builder = builder.index(index);
                        }
                        if let Ok(spin) = table.get::<f32>("spin") {
                            builder = builder.spin(spin);
                        }
                        if let Ok(random) = table.get::<f32>("random") {
                            builder = builder.random(random);
                        }
                        Ok(builder.build())
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
        methods.add_function("pan", |_, val: f32| Ok(Parameter::pan(val)));
        methods.add_function("tilt", |_, val: f32| Ok(Parameter::tilt(val)));
        methods.add_function("rgb", |_, (r, g, b): (f32, f32, f32)| Ok(Parameter::rgb(r, g, b)));
        methods.add_function("rgbw", |_, (r, g, b, w): (f32, f32, f32, f32)| {
            Ok(Parameter::rgbw(r, g, b, w))
        });
        methods.add_function("cmy", |_, (c, m, y): (f32, f32, f32)| Ok(Parameter::cmy(c, m, y)));
        methods.add_function("hsb", |_, (hue, saturation, brightness): (f32, f32, f32)| {
            Ok(Parameter::hsb(hue, saturation, brightness))
        });
        methods.add_function("cto", |_, val: f32| Ok(Parameter::cto(val)));
        methods.add_function("raw", |_, (attr_string, value): (String, f32)| {
            let attr = zeevonk::attr::Attribute::from_str(&attr_string)
                .map_err(|e| mlua::Error::external(format!("Invalid attribute: {:?}", e)))?;
            Ok(Parameter::raw(attr, value))
        });
    }
}
