use zeevonk::{attr::Attribute, value::ClampedValue};

#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Parameter {
    Dimmer(DimmerParameter),
    Raw((Attribute, ClampedValue)),
}

impl Parameter {
    pub fn dimmer(value: impl Into<ClampedValue>) -> Self {
        Self::Dimmer(DimmerParameter::Dimmer(value.into()))
    }

    pub fn raw(attr: Attribute, value: impl Into<ClampedValue>) -> Self {
        Self::Raw((attr, value.into()))
    }

    pub fn to_attribute_values(&self) -> Vec<(Attribute, ClampedValue)> {
        match self {
            Parameter::Dimmer(p) => p.to_attributes(),
            Parameter::Raw(p) => vec![*p],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum DimmerParameter {
    Dimmer(ClampedValue),
}

impl DimmerParameter {
    fn to_attributes(&self) -> Vec<(Attribute, ClampedValue)> {
        match self {
            DimmerParameter::Dimmer(v) => vec![(Attribute::Dimmer, *v)],
        }
    }
}
