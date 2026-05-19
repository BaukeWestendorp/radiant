use zeevonk::{
    AttributeName,
    value::{AttributeValue, ClampedValue},
};

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Parameter {
    Dimmer(ParameterValue),
    Pan(ParameterValue),
    Tilt(ParameterValue),
    Raw((AttributeName, ParameterValue)),
}

impl Parameter {
    pub fn dimmer(value: impl Into<ParameterValue>) -> Self {
        Self::Dimmer(value.into())
    }

    pub fn pan(value: impl Into<ParameterValue>) -> Self {
        Self::Pan(value.into())
    }

    pub fn tilt(value: impl Into<ParameterValue>) -> Self {
        Self::Tilt(value.into())
    }

    pub fn raw(attr: AttributeName, value: impl Into<ParameterValue>) -> Self {
        Self::Raw((attr, value.into()))
    }

    pub fn to_attribute_values(&self) -> Vec<(AttributeName, AttributeValue)> {
        match self {
            Parameter::Dimmer(p) => vec![(AttributeName::Dimmer, (*p).into())],
            Parameter::Pan(p) => vec![(AttributeName::Pan, (*p).into())],
            Parameter::Tilt(p) => vec![(AttributeName::Tilt, (*p).into())],
            Parameter::Raw((name, value)) => vec![(name.clone(), (*value).into())],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ParameterValue {
    Clamped(ClampedValue),
    Physical(f32),
}

impl From<ParameterValue> for AttributeValue {
    fn from(value: ParameterValue) -> Self {
        match value {
            ParameterValue::Clamped(v) => AttributeValue::Clamped(v),
            ParameterValue::Physical(v) => AttributeValue::Physical(v),
        }
    }
}
