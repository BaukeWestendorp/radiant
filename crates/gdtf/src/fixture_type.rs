use crate::error::Error;
use crate::raw::RawFixtureType;
use crate::{
    parse_guid, parse_name, parse_optional_guid, parse_yes_no, AttributeDefinitions, DmxChannel,
    DmxMode, Guid, Resource,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FixtureType {
    pub name: String,
    pub short_name: String,
    pub long_name: String,
    pub manufacturer: String,
    pub description: String,
    pub id: Guid,
    pub thumbnail: Option<Resource>,
    pub thumbnail_offset_x: i32,
    pub thumbnail_offset_y: i32,
    pub ref_ft: Option<Guid>,
    pub can_have_children: bool,
    pub attribute_definitions: AttributeDefinitions,
    pub dmx_modes: Vec<DmxMode>,
}

impl FixtureType {
    pub fn used_channels_for_mode(&self, mode_index: usize) -> Vec<&DmxChannel> {
        self.dmx_modes[mode_index]
            .dmx_channels
            .iter()
            .filter(|c| c.offset.is_some())
            .collect()
    }
}

impl TryFrom<RawFixtureType> for FixtureType {
    type Error = Error;

    fn try_from(value: RawFixtureType) -> Result<Self, Self::Error> {
        Ok(Self {
            name: parse_name(value.name)?,
            short_name: value.short_name,
            long_name: value.long_name,
            manufacturer: value.manufacturer,
            description: value.description,
            id: parse_guid(value.fixture_type_id)?,
            thumbnail: value.thumbnail,
            thumbnail_offset_x: value.thumbnail_offset_x,
            thumbnail_offset_y: value.thumbnail_offset_y,
            ref_ft: parse_optional_guid(value.ref_ft)?,
            can_have_children: parse_yes_no(&value.can_have_children)?,
            attribute_definitions: value.attribute_definitions.try_into()?,
            dmx_modes: value
                .dmx_modes
                .modes
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}
