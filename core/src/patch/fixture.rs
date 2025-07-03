use std::collections::HashMap;
use std::str::FromStr;

use eyre::ContextCompat;

use crate::error::Result;
use crate::patch::{Attribute, AttributeValue};

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
#[derive(serde::Deserialize)]
#[serde(transparent)]
pub struct FixtureId(pub u32);

/// A specific mode for a [Fixture]. Defined in the GDTF description.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(derive_more::Display)]
pub struct DmxMode(String);

impl DmxMode {
    /// Creates a new [DmxMode] with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Gets a string slice of the mode's name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A single patched fixture and has information about its attributes.
///
/// A fixture represents a lighting device that has been patched into the
/// system, containing information about its DMX address, supported attributes,
/// and GDTF definition. It provides methods to convert between high-level
/// attribute values and low-level DMX channel data.
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
    /// Creates a new fixture from GDTF fixture type data.
    ///
    /// Parses the GDTF fixture type definition to extract attribute
    /// information, DMX channel mappings, and default values. Returns an
    /// error if the specified DMX mode is not found in the fixture type
    /// definition.
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

    /// Returns this fixture's unique id.
    pub fn id(&self) -> FixtureId {
        self.id
    }

    /// Returns the DMX address of this fixture.
    pub fn address(&self) -> &dmx::Address {
        &self.address
    }

    /// Returns the currently active DMX mode of this fixture.
    pub fn dmx_mode(&self) -> &DmxMode {
        &self.dmx_mode
    }

    /// Returns the name of the GDTF file this fixture is based on.
    pub fn gdtf_file_name(&self) -> &str {
        &self.gdtf_file_name
    }

    /// Returns an iterator over all attributes this fixture supports.
    ///
    /// The attributes are those defined in the fixture's GDTF definition
    /// for the current DMX mode.
    pub fn supported_attributes(&self) -> impl Iterator<Item = &Attribute> {
        self.attributes.keys()
    }

    /// Returns a slice of all DMX modes supported by this fixture.
    pub fn supported_dmx_modes(&self) -> &[DmxMode] {
        &self.dmx_modes
    }

    /// Gets information about a specific [Attribute].
    ///
    /// Returns `None` if this fixture does not support the specified attribute.
    pub fn attribute_info(&self, attribute: &Attribute) -> Option<&AttributeInfo> {
        self.attributes.get(attribute)
    }

    /// Converts an attribute value to DMX channel values.
    ///
    /// Takes a high-level [AttributeValue] and converts it to the corresponding
    /// DMX channel and value pairs that should be sent to control this
    /// attribute. Returns an error if the attribute is not supported by
    /// this fixture.
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
            let channel = dmx::Channel::new(u16::from(self.address.channel) + *offset)
                .expect("channel should always be in range of universe");
            values.push((channel, value));
        }

        Ok(values)
    }

    /// Gets the default DMX channel values for this fixture.
    ///
    /// Returns DMX channel and value pairs that represent the fixture's
    /// default state. For example, 'Pan' and 'Tilt' attributes often default
    /// to the middle position (0.5) instead of zero.
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

    /// Gets the highlight DMX channel values for this fixture.
    ///
    /// Returns DMX channel and value pairs that make the fixture visible
    /// for identification purposes. For example, Dimmer and Shutter attributes
    /// are often set to provide basic visible output when checking the
    /// fixture's position or functionality.
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
    /// For example, the 'Pan' and 'Tilt' attributes often default to the
    /// middle, having a value of 0.5 instead of 0.
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
