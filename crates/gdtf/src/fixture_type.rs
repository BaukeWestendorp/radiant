use anyhow::Result;

use crate::attribute_definitions::AttributeDefinitions;
use crate::dmx_mode::DmxMode;
use crate::raw::RawFixtureType;
use crate::{parse_guid, parse_name, parse_yes_no};

#[derive(Debug, Clone, PartialEq)]
pub struct FixtureType {
    pub name: String,
    pub short_name: String,
    pub long_name: String,
    pub manufacturer: String,
    pub description: String,
    pub fixture_type_id: String,
    pub thumbnail: Option<String>,
    pub thumbnail_offset_x: i32,
    pub thumbnail_offset_y: i32,
    pub ref_ft: Option<String>,
    pub can_have_children: bool,

    pub attribute_definitions: AttributeDefinitions,
    // FIXME: pub wheels: Vec<Wheel>,
    // FIXME: pub physical_descriptions: Vec<PhysicalDescription>,
    // FIXME: pub models: Vec<Model>,
    // FIXME: pub geometries: Vec<Geometry>,
    pub dmx_modes: Vec<DmxMode>,
    // FIXME: pub revisions: Vec<Revision>,
    // FIXME: pub ft_presets: Vec<FtPreset>,
    // FIXME: pub protocols: Vec<Protocol>,
}

impl FixtureType {
    pub(crate) fn from_raw(raw: RawFixtureType) -> Result<Self> {
        let attribute_definitions = AttributeDefinitions::from_raw(raw.attribute_definitions)?;

        let dmx_modes = raw
            .dmx_modes
            .modes
            .into_iter()
            .map(|channel| DmxMode::from_raw(channel, attribute_definitions.attributes.as_slice()))
            .collect::<Result<_>>()?;

        Ok(Self {
            name: parse_name(raw.name)?,
            short_name: raw.short_name,
            long_name: raw.long_name,
            manufacturer: raw.manufacturer,
            description: raw.description,
            fixture_type_id: parse_guid(raw.fixture_type_id)?,
            thumbnail: raw.thumbnail,
            thumbnail_offset_x: raw.thumbnail_offset_x,
            thumbnail_offset_y: raw.thumbnail_offset_y,
            ref_ft: raw.ref_ft.and_then(|ref_ft| match ref_ft.is_empty() {
                true => None,
                false => Some(ref_ft),
            }),
            can_have_children: parse_yes_no(raw.can_have_children)?,
            attribute_definitions,
            dmx_modes,
        })
    }
}
