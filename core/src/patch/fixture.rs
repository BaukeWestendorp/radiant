use std::collections::HashMap;
use std::str::FromStr;

use eyre::ContextCompat;

use crate::{Attribute, AttributeValue, Result};

/// A unique id for a [Fixture].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[derive(
    derive_more::FromStr,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::Display,
    derive_more::From,
    derive_more::Into
)]
pub struct FixtureId(pub u32);

/// A specific mode for a [Fixture]. Often defined in the GDTF description.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(derive_more::Display)]
pub struct DmxMode(String);

impl DmxMode {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A single patched fixture and has information about its attributes.
#[derive(Debug, Clone, PartialEq)]
pub struct Fixture {
    id: FixtureId,
    pub(crate) address: dmx::Address,
    pub(crate) dmx_mode: DmxMode,
    pub(crate) gdtf_file_name: String,
    attributes: HashMap<Attribute, AttributeInfo>,

    dmx_modes: Vec<DmxMode>,
}

impl Fixture {
    pub fn new(
        id: FixtureId,
        address: dmx::Address,
        dmx_mode: DmxMode,
        gdtf_file_name: String,
        fixture_type: &gdtf::fixture_type::FixtureType,
    ) -> Result<Self> {
        let mut attributes = HashMap::new();

        let gdtf_dmx_mode = fixture_type.dmx_mode(dmx_mode.as_str()).with_context(|| {
            format!(
                "fried to get dmx mode '{}' for fixture type '{}'",
                dmx_mode, fixture_type.long_name
            )
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

            if let Some(attribute) = channel_function.attribute(fixture_type) {
                let Some(attribute_name) = &attribute.name else { continue };

                let attribute = Attribute::from_str(attribute_name).unwrap();

                let info = AttributeInfo { default_value, highlight_value, offset: offset.clone() };

                attributes.insert(attribute, info);
            }
        }

        let dmx_modes = fixture_type
            .dmx_modes
            .iter()
            .flat_map(|dmx_mode| dmx_mode.name.as_ref().map(|name| DmxMode::new(name.as_ref())))
            .collect();

        Ok(Self { id, dmx_mode, gdtf_file_name, address, attributes, dmx_modes })
    }

    pub fn id(&self) -> FixtureId {
        self.id
    }

    pub fn address(&self) -> &dmx::Address {
        &self.address
    }

    pub fn dmx_mode(&self) -> &DmxMode {
        &self.dmx_mode
    }

    pub fn gdtf_file_name(&self) -> &str {
        &self.gdtf_file_name
    }

    /// Gives an iterator over all attributes this
    /// fixture has defined in its GDTF definition.
    pub fn supported_attributes(&self) -> impl Iterator<Item = &Attribute> {
        self.attributes.keys()
    }

    pub fn supported_dmx_modes(&self) -> &[DmxMode] {
        &self.dmx_modes
    }

    /// Gets information about a specific [Attribute],
    /// if this fixture supports that [Attribute].
    pub fn attribute_info(&self, attribute: &Attribute) -> Option<&AttributeInfo> {
        self.attributes.get(attribute)
    }

    /// Gets the [dmx::Value]s of the used [dmx::Channel]s
    /// for a given [Attribute] and [AttributeValue].
    pub fn get_channel_values(
        &self,
        attribute: &Attribute,
        value: &AttributeValue,
    ) -> Result<Vec<(dmx::Channel, dmx::Value)>> {
        let mut values = Vec::new();

        let info = self.attribute_info(attribute).with_context(|| {
            format!(
                "attribute info for attribute '{}' not found for fixture '{}'",
                attribute, self.id
            )
        })?;

        let int_value = (value.as_f32() * u32::MAX as f32) as u32;
        let bytes: [u8; 4] = int_value.to_be_bytes();

        for (i, offset) in info.offset.iter().enumerate() {
            let value = dmx::Value(bytes[i]);
            let channel = dmx::Channel::new(u16::from(self.address.channel) + *offset - 1)
                .expect("channel should always be in range of universe");
            values.push((channel, value));
        }

        Ok(values)
    }

    /// Gets a [Vec] of the [dmx::Value]s for each [dmx::Channel]
    /// that contains the default [dmx::Value]s for this fixture.
    ///
    /// For example, the Pan and Tilt attributes often default to the middle,
    /// having a value of 0.5 instead of 0.
    pub fn get_default_channel_values(&self) -> Vec<(dmx::Channel, dmx::Value)> {
        let mut values = Vec::new();
        for info in self.attributes.values() {
            let int_value = (info.default_value().as_f32() * u32::MAX as f32) as u32;
            let bytes: [u8; 4] = int_value.to_be_bytes();

            for (i, offset) in info.offset.iter().enumerate() {
                let value = dmx::Value(bytes[i]);
                let channel = dmx::Channel::new(u16::from(self.address.channel) + *offset)
                    .expect("channel should always be in range of universe");
                values.push((channel, value));
            }
        }
        values
    }

    /// Gets a [Vec] of the [dmx::Value]s for each [dmx::Channel]
    /// that contains the highlight [dmx::Value]s for this fixture.
    ///
    /// For example, the Dimmer and Shutter attributes often should
    /// change to give some basic visible output when checking its position.
    pub fn get_highlight_channel_values(&self) -> Vec<(dmx::Channel, dmx::Value)> {
        let mut values = Vec::new();
        for info in self.attributes.values() {
            let Some(highlight_value) = info.highlight_value else { continue };

            let int_value = (highlight_value.as_f32() * u32::MAX as f32) as u32;
            let bytes: [u8; 4] = int_value.to_be_bytes();

            for (i, offset) in info.offset.iter().enumerate() {
                let value = dmx::Value(bytes[i]);
                let channel = dmx::Channel::new(u16::from(self.address.channel) + *offset)
                    .expect("channel should always be in range of universe");
                values.push((channel, value));
            }
        }
        values
    }
}

/// Some baked information about a specific attribute.
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeInfo {
    default_value: AttributeValue,
    highlight_value: Option<AttributeValue>,
    offset: Vec<u16>,
}

impl AttributeInfo {
    /// The default value for an attribute.
    ///
    /// For example, the Pan and Tilt attributes often default to the middle,
    /// having a value of 0.5 instead of 0.
    pub fn default_value(&self) -> AttributeValue {
        self.default_value
    }

    /// The highlight value for an attribute.
    ///
    /// For example, the Dimmer and Shutter attributes often should
    /// change to give some basic visible output when checking its position.
    pub fn highlight_value(&self) -> Option<AttributeValue> {
        self.highlight_value
    }
}
