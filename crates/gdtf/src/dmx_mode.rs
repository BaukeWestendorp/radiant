use std::rc::Rc;
use std::str::FromStr;

use anyhow::{anyhow, Result};

use crate::attribute_definitions::Attribute;
use crate::raw::{RawChannelFunction, RawDmxChannel, RawDmxMode, RawLogicalChannel};
use crate::{parse_i32_array, parse_name, DmxValue};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DmxMode {
    pub name: String,
    pub description: Option<String>,
    // FIXME: pub geometry: Geometry,
    pub dmx_channels: Vec<DmxChannel>,
    // FIXME: pub relations: Vec<Relation>,
    // FIXME: pub ft_macros: Vec<FtMacro>,
}

impl DmxMode {
    pub(crate) fn from_raw(raw: RawDmxMode, attributes: &[Rc<Attribute>]) -> Result<Self> {
        Ok(Self {
            name: parse_name(raw.name)?,
            description: raw.description,
            dmx_channels: raw
                .dmx_channels
                .channels
                .into_iter()
                .map(|channel| DmxChannel::from_raw(channel, attributes))
                .collect::<Result<_>>()?,
        })
    }

    pub fn channel_with_attribute(&self, attribute_name: &str) -> Option<&DmxChannel> {
        // FIXME: We currently just get the first logical channel with this attribute, but is that the right way?
        self.dmx_channels.iter().find(|channel| {
            channel
                .logical_channels
                .iter()
                .any(|lc| lc.attribute.name == attribute_name)
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DmxChannel {
    pub dmx_break: DmxBreak,
    pub offset: Option<Vec<i32>>,
    pub initial_function: ChannelFunction,
    pub highlight: Option<DmxValue>,
    // FIXME: pub geometry: Geometry,
    pub logical_channels: Vec<LogicalChannel>,
}

impl DmxChannel {
    pub(crate) fn from_raw(raw: RawDmxChannel, attributes: &[Rc<Attribute>]) -> Result<Self> {
        let logical_channels = raw
            .logical_channels
            .into_iter()
            .map(|logical_channel| LogicalChannel::from_raw(logical_channel, attributes))
            .collect::<Result<Vec<_>>>()?;

        let initial_function = match raw.initial_function {
            None => logical_channels
                .first()
                .and_then(|lc| lc.channel_functions.first())
                .unwrap()
                .clone(),
            Some(_initial_function_node) => {
                // FIXME: We should parse the node to get the actual initial function.
                logical_channels
                    .first()
                    .and_then(|lc| lc.channel_functions.first())
                    .unwrap()
                    .clone()
            }
        };

        Ok(Self {
            dmx_break: DmxBreak::from_str(&raw.dmx_break)?,
            offset: match raw.offset.as_str() {
                "" | "None" => None,
                offset => Some(parse_i32_array(offset.to_string())?),
            },
            initial_function,
            highlight: match raw.highlight.as_str() {
                "" | "None" => None,
                highlight => Some(DmxValue::from_str(highlight)?),
            },
            logical_channels,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DmxBreak {
    Overwrite,
    Value(i32),
}

impl FromStr for DmxBreak {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Overwrite" => Ok(Self::Overwrite),
            other => Ok(Self::Value(other.parse()?)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LogicalChannel {
    pub attribute: Rc<Attribute>,
    // FIXME: pub snap: Snap,
    // FIXME: pub master: Master,
    // FIXME: pub mib_fade: f32,
    // FIXME: pub dmx_change_time_limit: f32,
    pub channel_functions: Vec<ChannelFunction>,
}

impl LogicalChannel {
    pub(crate) fn from_raw(raw: RawLogicalChannel, attributes: &[Rc<Attribute>]) -> Result<Self> {
        Ok(Self {
            attribute: attributes
                .iter()
                .find(|attribute| attribute.name == raw.attribute)
                .ok_or_else(|| anyhow!("Unknown attribute: '{}'", raw.attribute))?
                .clone(),
            channel_functions: raw
                .channel_functions
                .into_iter()
                .map(|channel_function| ChannelFunction::from_raw(channel_function, attributes))
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChannelFunction {
    pub name: String,
    pub attribute: Option<Rc<Attribute>>,
    // FIXME: pub original_attribute: Option<String>,
    pub dmx_from: DmxValue,
    pub default: DmxValue,
    // FIXME: physical_from: f32,
    // FIXME: physical_to: f32,
    // FIXME: real_fade: f32,
    // FIXME: real_acceleration: f32,
    // FIXME: wheel: Option<RcWheel>>,
    // FIXME: emitter: Option<Rc<Emitter>>,
    // FIXME: filter: Option<Rc<Filter>>,
    // FIXME: color_space: Option<Rc<ColorSpace>>,
    // FIXME: gamut: Option<Rc<Gamut>>,
    // FIXME: mode_master: Option<Rc<ModeMaster>>,
    // FIXME: mode_from: DmxValue,
    // FIXME: mode_to: DmxValue,
    // FIXME: dmx_profile: Option<Rc<DmxProfile>>,
    // FIXME: min: f32,
    // FIXME: max: f32,
    // FIXME: custom_name: String,
}

impl ChannelFunction {
    pub(crate) fn from_raw(raw: RawChannelFunction, attributes: &[Rc<Attribute>]) -> Result<Self> {
        Ok(Self {
            name: parse_name(raw.name)?,
            attribute: match raw.attribute.as_str() {
                "NoFeature" => None,
                attribute_name => Some(
                    attributes
                        .iter()
                        .find(|attribute| attribute.name == attribute_name)
                        .ok_or_else(|| anyhow!("Unknown attribute: '{}'", attribute_name))?
                        .clone(),
                ),
            },
            dmx_from: DmxValue::from_str(&raw.dmx_from)?,
            default: DmxValue::from_str(&raw.default)?,
        })
    }
}
