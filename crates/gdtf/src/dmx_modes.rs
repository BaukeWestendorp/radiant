use std::{collections::HashMap, str::FromStr};

use crate::{
    error::Error,
    parse_int_array, parse_name, parse_node,
    raw::{
        RawChannelFunction, RawChannelSet, RawDmxChannel, RawDmxMode, RawLogicalChannel,
        RawSubchannelSet,
    },
    Attribute, DmxValue, Node,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DmxMode {
    pub name: String,
    pub description: String,
    pub geometry: String,

    pub dmx_channels: Vec<DmxChannel>,
    // FIXME: Implement
    // pub relations: Vec<Relation>,
    // FIXME: Implement
    // pub ft_macros: Vec<FtMacro>,
}

impl DmxMode {
    pub fn all_channel_functions(&self) -> Vec<&ChannelFunction> {
        self.all_logical_channels()
            .iter()
            .flat_map(|c| &c.channel_functions)
            .collect()
    }

    pub fn all_logical_channels(&self) -> Vec<&LogicalChannel> {
        self.dmx_channels
            .iter()
            .flat_map(|c| c.logical_channels.iter())
            .collect()
    }

    pub fn default_channel_values(&self) -> HashMap<String, Vec<u8>> {
        let mut values = HashMap::new();
        self.dmx_channels.iter().for_each(|c| {
            c.logical_channels.iter().for_each(|lc| {
                lc.channel_functions.iter().for_each(|cf| {
                    let offset_len = c.offset.as_ref().map(|o| o.len()).unwrap_or(0);
                    if offset_len == 0 {
                        return;
                    }

                    let attribute_name = lc.attribute[0].clone();
                    let value = cf
                        .default
                        .bytes(crate::ChannelBitResolution::from(offset_len as u8))
                        .unwrap();
                    values.insert(attribute_name, value);
                })
            })
        });
        values
    }
}

impl TryFrom<RawDmxMode> for DmxMode {
    type Error = Error;

    fn try_from(value: RawDmxMode) -> Result<Self, Self::Error> {
        Ok(Self {
            name: parse_name(value.name)?,
            description: value.description,
            geometry: parse_name(value.geometry)?,
            dmx_channels: value
                .dmx_channels
                .channels
                .into_iter()
                .map(DmxChannel::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DmxChannel {
    pub dmx_break: u32,
    pub offset: Option<Vec<i32>>,
    initial_function: Option<Node>,
    pub highlight: Option<DmxValue>,
    pub geometry: String,

    pub logical_channels: Vec<LogicalChannel>,
}

impl DmxChannel {
    pub fn initial_function<'a>(&'a self) -> Option<&ChannelFunction> {
        match &self.initial_function {
            Some(node) => {
                let first = node[0].split("_").collect::<Vec<_>>();
                let [_geometry, logical_channel] = [first[0], first[1]];
                let channel_function = &node[1];
                self.logical_channels
                    .iter()
                    .find(|lc| lc.attribute[0] == logical_channel)
                    .and_then(|lc| {
                        lc.channel_functions
                            .iter()
                            .find(|cf| cf.attribute[0] == *channel_function)
                    })
            }

            None => self
                .logical_channels
                .first()
                .and_then(|lc| lc.channel_functions.first()),
        }
    }

    pub fn all_channel_functions(&self) -> Vec<&ChannelFunction> {
        self.logical_channels
            .iter()
            .flat_map(|c| &c.channel_functions)
            .collect()
    }
}

impl TryFrom<RawDmxChannel> for DmxChannel {
    type Error = Error;

    fn try_from(value: RawDmxChannel) -> Result<Self, Self::Error> {
        Ok(Self {
            dmx_break: value.dmx_break.parse().map_err(|_| {
                Error::ParseError(format!("Invalid DMX break: '{}'", value.dmx_break))
            })?,
            offset: {
                match value.offset.as_str() {
                    "" => None,
                    offset => Some(parse_int_array(offset)?),
                }
            },
            initial_function: value.initial_function.map(parse_node).transpose()?,
            highlight: value.highlight.map(|v| v.parse()).transpose()?,
            geometry: parse_name(value.geometry)?,

            logical_channels: value
                .logical_channels
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogicalChannel {
    attribute: Node,
    pub snap: Snap,
    pub master: Master,
    pub mib_fade: f32,
    pub dmx_change_time_limit: f32,

    pub channel_functions: Vec<ChannelFunction>,
}

impl LogicalChannel {
    pub fn attribute<'a>(&'a self, attributes: &'a Vec<Attribute>) -> &Attribute {
        attributes
            .iter()
            .find(|a| a.name == self.attribute[0])
            .expect("Invalid attribute")
    }
}

impl TryFrom<RawLogicalChannel> for LogicalChannel {
    type Error = Error;

    fn try_from(value: RawLogicalChannel) -> Result<Self, Self::Error> {
        Ok(Self {
            attribute: parse_node(value.attribute)?,
            snap: value.snap.parse()?,
            master: value.master.parse()?,
            mib_fade: value.mib_fade,
            dmx_change_time_limit: value.dmx_change_time_limit,

            channel_functions: value
                .channel_functions
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Snap {
    Yes,
    #[default]
    No,
    On,
    Off,
}

impl FromStr for Snap {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Yes" => Ok(Self::Yes),
            "No" => Ok(Self::No),
            "On" => Ok(Self::On),
            "Off" => Ok(Self::Off),
            _ => Err(Error::ParseError(format!("Invalid snap: '{}'", s))),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Master {
    #[default]
    None,
    Grand,
    Group,
}

impl FromStr for Master {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Self::None),
            "Grand" => Ok(Self::Grand),
            "Group" => Ok(Self::Group),
            _ => Err(Error::ParseError(format!("Invalid master: '{}'", s))),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelFunction {
    pub name: String,
    attribute: Node,
    pub original_attribute: String,
    pub dmx_from: DmxValue,
    pub default: DmxValue,
    pub physical_from: f32,
    pub physical_to: f32,
    pub real_fade: f32,
    pub real_acceleration: f32,
    // FIXME: Implement getter.
    wheel: Option<Node>,
    // FIXME: Implement getter.
    emitter: Option<Node>,
    // FIXME: Implement getter.
    filter: Option<Node>,
    // FIXME: Implement getter.
    color_space: Option<Node>,
    // FIXME: Implement getter.
    gamut: Option<Node>,
    // FIXME: Implement getter.
    mode_master: Option<Node>,
    pub mode_from: DmxValue,
    pub mode_to: DmxValue,
    // FIXME: Implement getter.
    dmx_profile: Option<Node>,
    pub min: f32,
    pub max: f32,

    pub channel_sets: Vec<ChannelSet>,
    pub subchannel_sets: Vec<SubChannelSet>,
}

impl ChannelFunction {
    pub fn attribute<'a>(&'a self, attributes: &'a Vec<Attribute>) -> &Attribute {
        attributes
            .iter()
            .find(|a| a.name == self.attribute[0])
            .expect("Invalid attribute")
    }
}

impl TryFrom<RawChannelFunction> for ChannelFunction {
    type Error = Error;

    fn try_from(value: RawChannelFunction) -> Result<Self, Self::Error> {
        Ok(Self {
            name: parse_name(value.name)?,
            attribute: parse_node(value.attribute)?,
            original_attribute: value.original_attribute,
            dmx_from: value.dmx_from.parse()?,
            default: value.default.parse()?,
            physical_from: value.physical_from,
            physical_to: value.physical_to,
            real_fade: value.real_fade,
            real_acceleration: value.real_acceleration,
            wheel: value.wheel.map(parse_node).transpose()?,
            emitter: value.emitter.map(parse_node).transpose()?,
            filter: value.filter.map(parse_node).transpose()?,
            color_space: value.color_space.map(parse_node).transpose()?,
            gamut: value.gamut.map(parse_node).transpose()?,
            mode_master: value.mode_master.map(parse_node).transpose()?,
            mode_from: value.mode_from.parse()?,
            mode_to: value.mode_to.parse()?,
            dmx_profile: value.dmx_profile.map(parse_node).transpose()?,
            min: value.min.unwrap_or(value.physical_from),
            max: value.max.unwrap_or(value.physical_to),

            channel_sets: value
                .channel_sets
                .into_iter()
                .map(|cs| ChannelSet::try_from_raw(value.physical_from, value.physical_to, cs))
                .collect::<Result<Vec<_>, _>>()?,
            subchannel_sets: value
                .subchannel_sets
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelSet {
    pub name: String,
    pub dmx_from: DmxValue,
    pub physical_from: f32,
    pub physical_to: f32,
    pub wheel_slot_index: i32,
}

impl ChannelSet {
    pub(crate) fn try_from_raw(
        channel_function_physical_from: f32,
        channel_function_physical_to: f32,
        value: RawChannelSet,
    ) -> Result<Self, Error> {
        Ok(Self {
            name: parse_name(value.name)?,
            dmx_from: value.dmx_from.parse()?,
            physical_from: value
                .physical_from
                .unwrap_or(channel_function_physical_from),
            physical_to: value.physical_to.unwrap_or(channel_function_physical_to),
            wheel_slot_index: value.wheel_slot_index,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubChannelSet {
    pub name: String,
    pub physical_from: f32,
    pub physical_to: f32,
    // FIXME: Implement getter.
    sub_physical_unit: Node,
    // FIXME: Implement getter.
    dmx_profile: Option<Node>,
}

impl TryFrom<RawSubchannelSet> for SubChannelSet {
    type Error = Error;

    fn try_from(value: RawSubchannelSet) -> Result<Self, Self::Error> {
        Ok(Self {
            name: parse_name(value.name)?,
            physical_from: value.physical_from,
            physical_to: value.physical_to,
            sub_physical_unit: parse_node(value.sub_physical_unit)?,
            dmx_profile: value.dmx_profile.map(parse_node).transpose()?,
        })
    }
}
