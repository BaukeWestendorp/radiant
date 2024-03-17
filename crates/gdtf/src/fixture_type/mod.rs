//! # [Fixture Type Node](https://gdtf.eu/gdtf/file-spec/fixture-type-node/)
//!
//! Describes the starting point of the description of the fixture type.

use crate::{deserialize_yes_no, Guid, Resource};

use attribute_definitions::AttributeDefinitions;
use serde_inline_default::serde_inline_default;
use wheels::Wheels;

use self::attribute_definitions::Attribute;
use self::dmx_modes::{DmxChannel, DmxModes};

pub mod attribute_definitions;
pub mod dmx_modes;
pub mod wheels;

/// # [Fixture Type Node](https://gdtf.eu/gdtf/file-spec/fixture-type-node/#fixture-type-node)
///
/// The FixtureType node is the starting point of the description of the fixture
/// type within the XML file. The defined Fixture Type Node attributes of the
/// fixture type are specified in
/// [table 3](https://gdtf.eu/gdtf/file-spec/fixture-type-node/#table-3-fixture-type-node-attributes).
#[serde_inline_default]
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct FixtureType {
    /// Name of the fixture type. As it is based on Name, it is safe for
    /// parsing.
    #[serde(rename = "Name")]
    pub name: String,

    /// Shortened name of the fixture type. Non detailed version or an
    /// abbreviation. Can use any characters or symbols.
    #[serde(rename = "ShortName")]
    pub short_name: String,

    /// Detailed, complete name of the fixture type, can include any characters
    /// or extra symbols.
    #[serde(rename = "LongName")]
    pub long_name: String,

    /// Manufacturer of the fixture type.
    #[serde(rename = "Manufacturer")]
    pub manufacturer: String,

    /// Description of the fixture type.
    #[serde(rename = "Description")]
    pub description: String,

    /// Unique number of the fixture type.
    #[serde(rename = "FixtureTypeID")]
    pub fixture_type_id: Guid,

    /// Optional. File name without extension containing description of the
    /// thumbnail. Use the following as a resource file:
    /// - png file to provide the rasterized picture. Maximum resolution of
    ///   picture: 1024x1024
    /// - svg file to provide the vector graphic.
    /// - These resource files are located in the root directory of the zip
    ///   file.
    #[serde(rename = "Thumbnail")]
    pub thumbnail: Resource,

    /// Horizontal offset in pixels from the top left of the viewbox to the
    /// insertion point on a label. Default value: 0
    #[serde_inline_default(0)]
    #[serde(rename = "ThumbnailOffsetX")]
    pub thumbnail_offset_x: i32,

    /// Vertical offset in pixels from the top left of the viewbox to the
    /// insertion point on a label. Default value: 0
    #[serde_inline_default(0)]
    #[serde(rename = "ThumbnailOffsetY")]
    pub thumbnail_offset_y: i32,

    /// Optional. GUID of the referenced fixture type.
    #[serde(rename = "RefFT")]
    pub ref_ft: Option<Guid>,

    /// Describes if it is possible to mount other devices to this device.
    /// Value: “Yes”, “No”. Default value: “Yes”
    #[serde_inline_default(true)]
    #[serde(rename = "CanHaveChildren", deserialize_with = "deserialize_yes_no")]
    pub can_have_children: bool,

    /// Defines all Fixture Type Attributes that are used in the fixture type.
    #[serde(rename = "AttributeDefinitions")]
    pub attribute_definitions: AttributeDefinitions,

    /// Defines the physical or virtual color wheels, gobo wheels, media server
    /// content and others.
    #[serde(rename = "Wheels")]
    pub wheels: Option<Wheels>,

    // FIXME: Implement the following fields.
    // /// Contains additional physical descriptions.
    // #[serde(rename = "PhysicalDescriptions")]
    // pub physical_descriptions: Option<PhysicalDescriptions>,

    // /// Contains models of physically separated parts of the device.
    // #[serde(rename = "Models")]
    // pub models: Option<Models>,

    // /// Describes physically separated parts of the device.
    // #[serde(rename = "Geometries")]
    // pub geometries: Geometries,
    /// Contains descriptions of the DMX modes.
    #[serde(rename = "DMXModes")]
    pub dmx_modes: DmxModes,
    // /// Describes the history of the fixture type.
    // #[serde(rename = "Revisions")]
    // pub revisions: Option<Revisions>,

    // /// Is used to transfer user-defined and fixture type specific presets to
    // /// other show files.
    // #[serde(rename = "FTPresets")]
    // pub ft_presets: Option<FTPresets>,

    // /// Is used to specify supported protocols.
    // #[serde(rename = "Protocols")]
    // pub protocols: Option<Protocols>,
}

impl FixtureType {
    /// Returns the DMX channels that are actually used for the given mode.
    pub fn used_dmx_channels_for_mode(&self, mode_index: usize) -> Option<Vec<&DmxChannel>> {
        let mode = &self.dmx_modes.modes.get(mode_index)?;

        Some(
            mode.dmx_channels
                .channels
                .iter()
                // FIXME: Is this the right way of checking if a channel is used?
                .filter(|c| c.offset.as_ref().is_some_and(|c| !c.is_empty()))
                .collect(),
        )
    }

    /// Returns the attributes that are actually used for the given mode.
    pub fn used_attributes_for_mode(&self, mode_index: usize) -> Option<Vec<&Attribute>> {
        Some(
            self.used_dmx_channels_for_mode(mode_index)?
                .iter()
                .map(|c| c.logical_channels.get(0).unwrap().attribute.clone())
                .filter_map(|node| {
                    let attribute_name = node.references().get(0).unwrap();
                    self.attribute_from_name(attribute_name)
                })
                .collect(),
        )
    }

    /// Returns the attribute with the given name.
    pub fn attribute_from_name(&self, name: &str) -> Option<&Attribute> {
        self.attribute_definitions
            .attributes
            .attributes
            .iter()
            .find(|a| a.name.to_string() == name)
    }
}
