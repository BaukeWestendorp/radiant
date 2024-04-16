use serde_inline_default::serde_inline_default;

use super::{RawAttributeDefinitions, RawDmxModes, RawEnum, RawGuid, RawName, RawResource};

/// # [Fixture Type Node](https://gdtf.eu/gdtf/file-spec/fixture-type-node/#fixture-type-node)
///
/// The FixtureType node is the starting point of the description of the fixture
/// type within the XML file. The defined Fixture Type Node attributes of the
/// fixture type are specified in
/// [table 3](https://gdtf.eu/gdtf/file-spec/fixture-type-node/#table-3-fixture-type-node-attributes).
#[serde_inline_default]
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct RawFixtureType {
    /// Name of the fixture type. As it is based on Name, it is safe for
    /// parsing.
    #[serde(rename = "Name")]
    pub name: RawName,

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
    pub description: Option<String>,

    /// Unique number of the fixture type.
    #[serde(rename = "FixtureTypeID")]
    pub fixture_type_id: RawGuid,

    /// Optional. File name without extension containing description of the
    /// thumbnail. Use the following as a resource file:
    /// - png file to provide the rasterized picture. Maximum resolution of
    ///   picture: 1024x1024
    /// - svg file to provide the vector graphic.
    /// - These resource files are located in the root directory of the zip
    ///   file.
    #[serde(rename = "Thumbnail")]
    pub thumbnail: Option<RawResource>,

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
    pub ref_ft: Option<RawGuid>,

    /// Describes if it is possible to mount other devices to this device.
    /// Value: “Yes”, “No”. Default value: “Yes”
    #[serde_inline_default("Yes".to_string())]
    #[serde(rename = "CanHaveChildren")]
    pub can_have_children: RawEnum,

    /// Defines all Fixture Type Attributes that are used in the fixture type.
    #[serde(rename = "AttributeDefinitions")]
    pub attribute_definitions: RawAttributeDefinitions,

    /// Contains descriptions of the DMX modes.
    #[serde(rename = "DMXModes")]
    pub dmx_modes: RawDmxModes,
}
