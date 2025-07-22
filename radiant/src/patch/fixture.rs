use std::str::FromStr;

use eyre::ContextCompat;
use gdtf::attribute::FeatureGroup;
use gdtf::dmx_mode::DmxMode;
use gdtf::fixture_type::FixtureType;
use uuid::Uuid;

use crate::error::Result;
use crate::patch::{Attribute, AttributeValue, Patch};

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
    pub(crate) fixture_type_id: Uuid,
    pub(crate) dmx_mode: String,
}

impl Fixture {
    /// Creates a new fixture with the corresponding patch information and
    /// fixture definition.
    pub fn new(
        id: FixtureId,
        address: dmx::Address,
        fixture_type_id: Uuid,
        dmx_mode: impl Into<String>,
    ) -> Self {
        Self { id, address, fixture_type_id, dmx_mode: dmx_mode.into() }
    }

    /// Returns this fixture's unique id.
    pub fn id(&self) -> FixtureId {
        self.id
    }

    /// Returns the address of this fixture.
    pub fn address(&self) -> &dmx::Address {
        &self.address
    }
    /// Returns a reference to the [FixtureType] associated with this fixture.
    ///
    /// # Panics
    ///
    /// Panics if the fixture type id is not valid in the provided [Patch].
    pub fn fixture_type<'a>(&self, patch: &'a Patch) -> &'a FixtureType {
        patch
            .fixture_type(self.fixture_type_id)
            .expect("fixture should always have a valid fixture type id")
    }

    /// Returns a slice of [FeatureGroup]s supported by this fixture.
    ///
    /// This is derived from the fixture's GDTF definition.
    pub fn feature_groups<'a>(&self, patch: &'a Patch) -> &'a [FeatureGroup] {
        &self.fixture_type(patch).attribute_definitions.feature_groups
    }

    /// Returns a reference to the [DmxMode] for this fixture.
    ///
    /// # Panics
    ///
    /// Panics if the DMX mode is not valid for the fixture type in the provided
    /// [Patch].
    pub fn dmx_mode<'a>(&self, patch: &'a Patch) -> &'a DmxMode {
        self.fixture_type(patch)
            .dmx_mode(&self.dmx_mode)
            .expect("fixture should always have a valid dmx mode index")
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
        patch: &Patch,
    ) -> Result<Vec<(dmx::Channel, dmx::Value)>> {
        let channels = self.channels_for_attribute(attribute, patch)?;
        let mut values = Vec::new();
        for (ix, channel) in channels.into_iter().enumerate() {
            let int_value = (value.as_f32() * u32::MAX as f32) as u32;
            let bytes: [u8; 4] = int_value.to_be_bytes();
            let value = dmx::Value(bytes[ix]);
            values.push((channel, value));
        }
        Ok(values)
    }

    /// Returns attributes supported by this fixture along with
    /// their corresponding default [AttributeValue]s as
    /// defined in the fixture's GDTF definition.
    pub fn get_default_attribute_values(&self, patch: &Patch) -> Vec<(Attribute, AttributeValue)> {
        let fixture_type = self.fixture_type(patch);

        let mut values = Vec::new();
        for dmx_channel in &self.dmx_mode(patch).dmx_channels {
            let Some((_, channel_function)) = dmx_channel.initial_function() else {
                continue;
            };
            let Some(mut attribute) = channel_function.attribute(fixture_type) else {
                continue;
            };
            if let Some(main_attribute) =
                attribute.main_attribute(&fixture_type.attribute_definitions)
            {
                attribute = main_attribute;
            };

            let Some(attribute_name) = attribute.name.as_ref() else { continue };
            let attribute = Attribute::from_str(&attribute_name).unwrap();

            values.push((attribute, channel_function.default.into()));
        }
        values
    }

    pub fn channels_for_attribute(
        &self,
        attribute: &Attribute,
        patch: &Patch,
    ) -> Result<Vec<dmx::Channel>> {
        let dmx_channel = &self
            .dmx_mode(patch)
            .dmx_channels
            .iter()
            .find(|dmx_channel| {
                dmx_channel.logical_channels.iter().any(|logical_channel| {
                    let fixture_type = self.fixture_type(patch);
                    if logical_channel.attribute(fixture_type).is_some_and(|attr| {
                        attr.name.as_ref().is_some_and(|name| **name == attribute.to_string())
                    }) {
                        return true;
                    } else if logical_channel.channel_functions.iter().any(|channel_function| {
                        channel_function.attribute(fixture_type).is_some_and(|attr| {
                            attr.name.as_ref().is_some_and(|name| **name == attribute.to_string())
                        })
                    }) {
                        return true;
                    } else {
                        return false;
                    }
                })
            })
            .wrap_err_with(|| format!("channel not found for attribute {attribute}"))?;

        let offsets = dmx_channel
            .offset
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|offset| (offset - 1).clamp(u16::MIN as i32, u16::MAX as i32) as u16);

        let channels = offsets
            .map(|offset| {
                dmx::Channel::new(u16::from(self.address.channel) + offset)
                    .expect("channel should always be in range of universe")
            })
            .collect();

        Ok(channels)
    }
}
