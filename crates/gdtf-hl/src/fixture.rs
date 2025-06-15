use crate::{Attribute, AttributeValue, Error};
use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FixtureId(pub u32);

impl Deref for FixtureId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FixtureId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<u32> for FixtureId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<FixtureId> for u32 {
    fn from(value: FixtureId) -> Self {
        *value
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DmxMode(pub String);

impl DmxMode {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for DmxMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fixture {
    id: FixtureId,
    dmx_mode: DmxMode,
    attributes: HashMap<Attribute, AttributeInfo>,
}

impl Fixture {
    pub fn new(
        id: FixtureId,
        dmx_mode: DmxMode,
        fixture_type: &gdtf::fixture_type::FixtureType,
    ) -> Result<Self, Error> {
        let mut attributes = HashMap::new();

        let gdtf_dmx_mode =
            fixture_type.dmx_mode(dmx_mode.as_str()).ok_or_else(|| Error::InvalidDmxMode {
                dmx_mode: dmx_mode.clone(),
                fixture_type_name: fixture_type.long_name.clone(),
            })?;

        for channel in &gdtf_dmx_mode.dmx_channels {
            let Some(offset) = channel.offset.clone().map(|o| {
                o.into_iter()
                    .map(|o| (o - 1).clamp(u16::MIN as i32, u16::MAX as i32) as u16)
                    .collect::<Vec<u16>>()
            }) else {
                continue;
            };

            let Some((_, channel_function)) = channel.initial_function() else {
                continue;
            };

            let default_value = channel_function.default.into();
            let highlight_value = channel.highlight.map(From::from);

            if let Some(attribute) = channel_function.attribute(&fixture_type) {
                let Some(attribute_name) = &attribute.name else { continue };

                let attribute = Attribute::from_str(attribute_name);

                let info = AttributeInfo {
                    value: default_value,
                    default_value,
                    highlight_value,
                    offset: offset.clone(),
                };

                attributes.insert(attribute, info);
            }
        }

        Ok(Self { id, dmx_mode, attributes })
    }

    pub fn id(&self) -> FixtureId {
        self.id
    }

    pub fn set_id(&mut self, id: FixtureId) {
        self.id = id
    }

    pub fn supported_attributes(&self) -> impl Iterator<Item = &Attribute> {
        self.attributes.keys()
    }

    pub fn attribute_info(&self, attribute: &Attribute) -> Option<&AttributeInfo> {
        self.attributes.get(attribute)
    }
    pub fn attribute_info_mut(&mut self, attribute: &Attribute) -> Option<&mut AttributeInfo> {
        self.attributes.get_mut(attribute)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeInfo {
    value: AttributeValue,
    default_value: AttributeValue,
    highlight_value: Option<AttributeValue>,
    offset: Vec<u16>,
}

impl AttributeInfo {
    pub fn value(&self) -> AttributeValue {
        self.value
    }

    pub fn set_value(&mut self, value: AttributeValue) {
        self.value = value;
    }

    pub fn default_value(&self) -> AttributeValue {
        self.default_value
    }

    pub fn highlight_value(&self) -> Option<AttributeValue> {
        self.highlight_value
    }
}
