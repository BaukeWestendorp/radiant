//! # [DMX Mode Collect ](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/)
//!
//! This section is describes all DMX modes of the device.

use serde_inline_default::serde_inline_default;

use crate::{deserialize_optional_dmx_value, deserialize_optional_int_array, DmxValue, Name, Node};

/// # [DMX Mode Collect](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#dmx-mode-collect)
///
/// This section is describes all DMX modes of the device. If firmware revisions
/// change a DMX footprint, then such revisions should be specified as new DMX
/// mode. The DMX mode collect currently does not have any attributes (XML node
/// `<DMXModes>`).
///
/// As children the fixture type DMX mode collect has a list of a [DmxMode].
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct DmxModes {
    #[allow(missing_docs)]
    #[serde(rename = "$value")]
    pub modes: Vec<DmxMode>,
}

/// # [DMX Mode](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#dmx-mmode)
///
/// Each DMX mode describes logical control part of the device in a specific
/// mode (XML node `<DMXMode>`). The currently defined XML attributes of the DMX
/// mode are specified in [table 56](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-56).
///
/// DMX mode children are specified in
/// [table 57](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-57-dmx-mode-children).
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct DmxMode {
    /// The unique name of the DMX mode
    #[serde(rename = "Name")]
    pub name: Name,

    /// Description of the DMX mode
    #[serde(rename = "Description")]
    pub description: String,

    /// Name of the first geometry in the device; Only top level geometries are
    /// allowed to be linked.
    #[serde(rename = "Geometry")]
    pub geometry: Name,

    /// Description of all DMX channels used in the mode
    #[serde(rename = "DMXChannels")]
    pub dmx_channels: DmxChannels,

    /// Description of relations between channels
    #[serde(rename = "Relations")]
    pub relations: Option<Relations>,

    /// Is used to describe macros of the manufacturer.
    #[serde(rename = "FTMacros")]
    pub ft_macros: Option<FtMacros>,
}

/// # [DMX Channel Collect](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#dmx-channels)
///
/// This section defines the DMX footprint of the device. The DMX channel
/// collect currently does not have any attributes (XML node `<DMXChannels>`).
///
/// As children the DMX channel collect has a list of a [DmxChannel].
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct DmxChannels {
    #[allow(missing_docs)]
    #[serde(rename = "$value")]
    pub channels: Vec<DmxChannel>,
}

/// # [DMX Channel](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-58-dmx-channel-attributes)
///
/// This section defines the DMX channel (XML node `<DMXChannel>`). The name of
/// a DMX channel cannot be user-defined and must consist of a geometry name and
/// the attribute name of the first logical channel with separator "_". In one
/// DMX Mode, this combination needs to be unique. Currently defined XML
/// attributes of the DMX channel are specified in
/// [table 58](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-58)
///
/// As children the DMX channel has a list of a [LogicalChannel].
#[serde_inline_default]
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct DmxChannel {
    /// Number of the DMXBreak; Default value: 1; Special value: "Overwrite" –
    /// means that this number will be overwritten by Geometry Reference; Size:
    /// 4 bytes
    #[serde(rename = "DMXBreak")]
    #[serde_inline_default(DmxBreak::Value(1))]
    pub dmx_break: DmxBreak,

    /// Relative addresses of the current DMX channel from highest to least
    /// significant; Separator of values is ","; Special value: "None" – does
    /// not have any addresses; Default value: "None"; Size per int: 4 bytes
    #[serde(rename = "Offset", deserialize_with = "deserialize_optional_int_array")]
    pub offset: Option<Vec<i32>>,

    /// Link to the channel function that will be activated by default for this
    /// DMXChannel. Default value is the first channel function of the first
    /// logical function of this DMX channel.

    #[serde(rename = "InitialFunction")]
    pub initial_function: Node,

    /// Highlight value for current channel; Special value: "None". Default
    /// value: "None".
    #[serde(
        rename = "Highlight",
        deserialize_with = "deserialize_optional_dmx_value"
    )]
    pub highlight: Option<DmxValue>,

    /// Name of the geometry the current channel controls.
    ///
    /// The `Geometry` should be the place in the tree of geometries where the
    /// function of the DMX Channel (as defined by [ChannelFunction]) is located
    /// either physically or logically. If the DMX channel doesn’t have a
    /// location, put it in the top level geometry of the geometry tree.
    /// Attributes follow a trickle down principle, so they are inherited from
    /// top down.
    #[serde(rename = "Geometry")]
    pub geometry: Name,

    #[allow(missing_docs)]
    #[serde(rename = "$value")]
    pub logical_channels: Vec<LogicalChannel>,
}

/// Breaks are used if a fixture needs more than one start address. For example,
/// a scroller is added to an existing conventional fixture and the fixture is
/// connected to a dimmer. This dimmer is patched in one universe and the
/// scroller is connected to a PSU that, on the other hand, is patched in
/// another universe. Both, the conventional fixture and scroller, are treated
/// as one combined fixture.
#[derive(Debug, Clone, PartialEq)]
pub enum DmxBreak {
    /// Overwrite means that this number will be overwritten by Geometry
    /// Reference
    Overwrite,
    /// Value
    Value(i32),
}

impl<'de> serde::Deserialize<'de> for DmxBreak {
    fn deserialize<D>(deserializer: D) -> Result<DmxBreak, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Overwrite" => Ok(DmxBreak::Overwrite),
            _ => match s.parse() {
                Ok(v) => Ok(DmxBreak::Value(v)),
                Err(_) => Err(serde::de::Error::custom("invalid DmxBreak value")),
            },
        }
    }
}

/// # [Logical Channel](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-59-logical-channel-attributes)
///
/// The Fixture Type Attribute is assigned to a LogicalChannel and defines the
/// function of the LogicalChannel. All logical channels that are children of
/// the same DMX channel are mutually exclusive. In a DMX mode, only one logical
/// channel with the same attribute can reference the same geometry at a time.
/// The name of a Logical Channel cannot be user-defined and is equal to the
/// linked attribute name. The XML node of the logical channel is
/// `<LogicalChannel>`. The currently defined XML attributes of the logical
/// channel are specified in
/// [table 59](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-59).
///
/// As a child the logical channel has a list of a [ChannelFunction].
#[serde_inline_default]
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct LogicalChannel {
    /// Link to the attribute; The starting point is the Attribute Collect (see
    /// [Annex A](https://gdtf.eu/gdtf/annex/annex-a/)).
    #[serde(rename = "Attribute")]
    pub attribute: Node,

    /// If snap is enabled, the logical channel will not fade between values.
    /// Instead, it will jump directly to the new value. Value: “Yes”, “No”,
    /// “On”, “Off”. Default value: “No”
    #[serde(rename = "Snap")]
    pub snap: Snap,

    /// Defines if all the subordinate channel functions react to a Group
    /// Control defined by the control system. Values: “None”, “Grand”, “Group”;
    /// Default value: “None”.
    #[serde(rename = "Master")]
    pub master: Master,

    /// Minimum fade time for moves in black action. MibFade is defined for the
    /// complete DMX range. Default value: 0; Unit: second
    #[serde(rename = "MibFade")]
    #[serde_inline_default(0.0)]
    pub mib_fade: f32,

    /// Minimum fade time for the subordinate channel functions to change DMX
    /// values by the control system. DMXChangeTimeLimit is defined for the
    /// complete DMX range. Default value: 0; Unit: second
    #[serde(rename = "DMXChangeTimeLimit")]
    #[serde_inline_default(0.0)]
    pub dmx_change_time_limit: f32,

    #[allow(missing_docs)]
    #[serde(rename = "$value")]
    pub channel_functions: Vec<ChannelFunction>,
}

/// If snap is enabled, the logical channel will not fade between values.
/// Instead, it will jump directly to the new value.
#[derive(Debug, Clone, PartialEq, Default, serde::Deserialize)]
pub enum Snap {
    /// No
    #[default]
    No,
    /// Yes
    Yes,
    /// Off
    Off,
    /// On
    On,
}

/// Defines if all the subordinate channel functions react to a Group Control
/// defined by the control system
#[derive(Debug, Clone, PartialEq, Default, serde::Deserialize)]
pub enum Master {
    /// None
    #[default]
    None,
    /// Grand
    Grand,
    /// Group
    Group,
}

/// # [Channel Function](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-60-channel-function-attributes)
///
/// The Fixture Type Attribute is assigned to a Channel Function and defines the
/// function of its DMX Range. (XML node <ChannelFunction>). The currently
/// defined XML attributes of channel function are specified in
/// [table 60](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-60-channel-function-attributes).
///
/// Note:
/// For command based control systems, you can control the fixture by sending it
/// a string in the following style: "`/FIXTURE_ID/CUSTOM_NAME_CHANNELFUCTION`
/// ,`f FLOAT_VALUE_PHYSICAL`" or
/// "`/FIXTURE_ID/CUSTOM_NAME_CHANNELFUCTION/percent` ,`f FLOAT_VALUE_PERCENT`"
///
/// Where:
///
/// - `FIXTURE_ID` is the fixture ID is the value defined for the fixture
///   instance.
/// - `CUSTOM_NAME_CHANNELFUCTION` is the Custom Name for the ChannelFunction.
///   Note that all “.” Separators can be replaced with “/”.
/// - `FLOAT_VALUE_PHYSICAL` is the physical value that the fixture should
///   adopt. The values will be capped by the fixture by PhysicalFrom and
///   PhysicalTo.
/// - `FLOAT_VALUE_PERCENT` is the percent value that the fixture should adopt.
///   The values can be between 0 and 100.
///
/// As children the channel function has list of a [ChannelSet] and a
/// [SubChannelSet].
#[serde_inline_default]
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct ChannelFunction {
    /// Unique name; Default value: Name of attribute and number of channel
    /// function.
    #[serde(rename = "Name")]
    pub name: Name,

    /// Link to attribute; Starting point is the attributes node. Default value:
    /// “NoFeature”.
    #[serde(rename = "Attribute")]
    #[serde_inline_default(Node::from("NoFeature"))]
    pub attribute: Node,

    /// The manufacturer’s original name of the attribute; Default: empty
    #[serde(rename = "OriginalAttribute")]
    #[serde_inline_default("".to_string())]
    pub original_attribute: String,

    /// Start DMX value; The end DMX value is calculated as a DMXFrom of the
    /// next channel function – 1 or the maximum value of the DMX channel.
    /// Default value: “0/1”.
    #[serde(rename = "DMXFrom")]
    #[serde_inline_default("0/1".try_into().unwrap())]
    pub dmx_from: DmxValue,

    /// Default DMX value of channel function when activated by the control
    /// system.
    #[serde(rename = "Default")]
    pub default: DmxValue,

    /// Physical start value; Default value: 0
    #[serde(rename = "PhysicalFrom")]
    #[serde_inline_default(0.0)]
    pub physical_from: f32,

    /// Physical end value; Default value: 1
    #[serde(rename = "PhysicalTo")]
    #[serde_inline_default(1.0)]
    pub physical_to: f32,

    /// Time in seconds to move from min to max of the Channel Function; Default
    /// value: 0
    #[serde(rename = "RealFade")]
    #[serde_inline_default(0.0)]
    pub real_fade: f32,

    /// Time in seconds to accelerate from stop to maximum velocity; Default
    /// value: 0
    #[serde(rename = "RealAcceleration")]
    #[serde_inline_default(0.0)]
    pub real_acceleration: f32,

    /// Optional. Link to a wheel; Starting point: Wheel Collect
    #[serde(rename = "Wheel")]
    pub wheel: Option<Node>,

    /// Optional. Link to an emitter in the physical description; Starting
    /// point: Emitter Collect
    #[serde(rename = "Emitter")]
    pub emitter: Option<Node>,

    /// Optional. Link to a filter in the physical description; Starting point:
    /// Filter Collect
    #[serde(rename = "Filter")]
    pub filter: Option<Node>,

    /// Optional. Link to a color space in the physical description; Starting
    /// point: Physical Descriptions Collect
    #[serde(rename = "ColorSpace")]
    pub color_space: Option<Node>,

    /// Optional. Link to a gamut in the physical description; Starting point:
    /// Gamut Collect
    #[serde(rename = "Gamut")]
    pub gamut: Option<Node>,

    /// Optional. Link to DMX Channel or Channel Function; Starting point DMX
    /// mode.
    #[serde(rename = "ModeMaster")]
    pub mode_master: Option<Node>,

    /// Only used together with ModeMaster; DMX start value; Default value: 0/1
    #[serde(rename = "ModeFrom")]
    #[serde_inline_default(DmxValue::try_from("0/1").unwrap())]
    pub mode_from: DmxValue,

    /// Only used together with ModeMaster; DMX end value; Default value: 0/1
    #[serde(rename = "ModeTo")]
    #[serde_inline_default(DmxValue::try_from("0/1").unwrap())]
    pub mode_to: DmxValue,

    /// Optional link to DMX Profile; Starting point: DMX Profile Collect
    #[serde(rename = "DMXProfile")]
    pub dmx_profile: Option<Node>,

    #[serde(rename = "Min")]
    min: Option<f32>,

    #[serde(rename = "Max")]
    max: Option<f32>,

    #[serde(rename = "CustomName")]
    custom_name: Option<String>,

    #[allow(missing_docs)]
    #[serde(rename = "$value", default = "Vec::new")]
    pub channel_sets: Vec<ChannelSet>,

    #[allow(missing_docs)]
    #[serde(rename = "$value", default = "Vec::new")]
    pub subchannel_sets: Vec<SubchannelSet>,
}

impl ChannelFunction {
    /// Minimum Physical Value that will be used for the DMX Profile. Default:
    /// Value from PhysicalFrom
    pub fn min(&self) -> f32 {
        self.min.unwrap_or(self.physical_from)
    }

    /// Maximum Physical Value that will be used for the DMX Profile. Default:
    /// Value from PhysicalTo
    pub fn max(&self) -> f32 {
        self.max.unwrap_or(self.physical_to)
    }

    /// Custom Name that can he used do adress this channel function with other
    /// command based protocols like OSC. Default: Node Name of the Channel
    /// function
    ///
    /// Example: Head_Dimmer.Dimmer.Dimmer
    pub fn custom_name(&self) -> String {
        self.custom_name.clone().unwrap_or(self.name.0.clone())
    }
}

/// # [Channel Set](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-61-channel-set-attributes)
///
/// This section defines the channel sets of the channel function (XML node).
/// The currently defined XML attributes of the channel set are specified in
/// [table 61](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-61).
#[serde_inline_default]
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct ChannelSet {
    /// The name of the channel set. Default: Empty
    #[serde(rename = "Name")]
    #[serde_inline_default(Name::new("".to_string()).unwrap())]
    pub name: Name,

    /// Start DMX value; The end DMX value is calculated as a DMXFrom of the
    /// next channel set – 1 or the maximum value of the current channel
    /// function; Default value: 0/1
    #[serde(rename = "DMXFrom")]
    #[serde_inline_default(DmxValue::try_from("0/1").unwrap())]
    pub dmx_from: DmxValue,

    #[serde(rename = "PhysicalFrom")]
    physical_from: Option<f32>,

    #[serde(rename = "PhysicalTo")]
    physical_to: Option<f32>,

    /// If the channel function has a link to a wheel, a corresponding slot
    /// index shall be specified. The wheel slot index results from the order of
    /// slots of the wheel which is linked in the channel function. The wheel
    /// slot index is normalized to 1.
    #[serde(rename = "WheelSlotIndex")]
    pub wheel_slot_index: i32,
}

impl ChannelSet {
    /// Physical start value. Default value is the PhysicalFrom from the parent
    /// channel function.
    pub fn physical_from(&self, channel_function: &ChannelFunction) -> f32 {
        self.physical_from.unwrap_or(channel_function.physical_from)
    }

    /// Physical end value. Default value is the PhysicalTo from the parent
    /// channel function.
    pub fn physical_to(&self, channel_function: &ChannelFunction) -> f32 {
        self.physical_to.unwrap_or(channel_function.physical_to)
    }
}

/// # [Sub Channel Set](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-62)
///
/// This section defines the sub channel sets of the channel function (XML node
/// ). The currently defined XML attributes of the sub channel set are specified
/// in [table 62](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-62).
///
/// The sub channel set does not have any children.
#[serde_inline_default]
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct SubchannelSet {
    /// The name of the sub channel set. Default: Empty
    #[serde(rename = "Name")]
    #[serde_inline_default(Name::new("".to_string()).unwrap())]
    pub name: Name,

    /// Physical start value
    #[serde(rename = "PhysicalFrom")]
    pub physical_from: f32,

    /// Physical end value
    #[serde(rename = "PhysicalTo")]
    pub physical_to: f32,

    /// Link to the sub physical unit; Starting Point: Attribute
    #[serde(rename = "SubPhysicalUnit")]
    pub sub_physical_unit: Node,

    /// Optional link to the DMX Profile; Starting Point: DMX Profile Collect
    #[serde(rename = "DMXProfile")]
    pub dmx_profile: Option<Node>,
}

/// # [Relations Collect](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#relation-collect)
///
/// This section describes the dependencies between DMX channels and channel
/// functions, such as multiply and override. The relation collect currently
/// does not have any XML attributes (XML node` <Relations>`).
///
/// As children therelation collect has a list of a [Relation].
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Relations {
    #[allow(missing_docs)]
    #[serde(rename = "$value", default = "Vec::new")]
    pub relations: Vec<Relation>,
}

/// # [Relation](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-63-relation-attributes)
///
/// This section defines the relation between the master DMX channel and the
/// following logical channel (XML node `<Relation>`). The currently defined XML
/// attributes of the relations are specified in
/// [table 63](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-63).
///
/// The relation does not have any children.
///
/// [Listing 1](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#listing-1)
///  shows an example of a simple DMX mode described in XML.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Relation {
    /// The unique name of the relation
    #[serde(rename = "Name")]
    pub name: Name,

    /// Link to the master DMX channel; Starting point: DMX mode
    #[serde(rename = "Master")]
    pub master: Node,

    /// Link to the following channel function; Starting point: DMX mode
    #[serde(rename = "Follower")]
    pub follower: Node,

    /// Type of the relation; Values: “Multiply”, “Override”
    #[serde(rename = "Type")]
    pub r#type: RelationType,
}

/// Type of the relation.
#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize)]
pub enum RelationType {
    /// Multiply
    Multiply,
    /// Override
    Override,
}

/// # [Macro Collect](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#macro-collect)
///
/// This section describes DMX sequences to be executed by the control system.
/// The macro collect currently does not have any XML attributes (XML node
/// `<FTMacros>`). As children the macro collect has a list of a [Macro].
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct FtMacros {
    #[allow(missing_docs)]
    #[serde(rename = "$value", default = "Vec::new")]
    pub macros: Vec<FtMacro>,
}

/// # [Macro](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-64-macro-attributes)
///
/// This section defines a DMX sequence. (XML node `<FTMacro>`). The currently
/// defined XML attributes of the macro are specified in
/// [table 64](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-64).
///
/// Macro children are specified in
/// [table 65](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-65)
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct FtMacro {
    /// The unique name of the macro.
    #[serde(rename = "Name")]
    pub name: Name,

    /// Optional. Link to channel function; Starting point DMX Mode
    #[serde(rename = "ChannelFunction")]
    pub channel_function: Option<Node>,

    /// This section defines a DMX sequence.
    #[serde(rename = "MacroDMX")]
    pub macro_dmx: Option<MacroDmx>,
}

/// This section defines the sequence of DMX values which are sent by a control
/// system. (XML node `<MacroDMX>`).
///
/// As children the macro DMX has a list of [MacroDMXStep].
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct MacroDmx {
    #[allow(missing_docs)]
    #[serde(rename = "$value", default = "Vec::new")]
    pub dmx_steps: Vec<MacroDmxStep>,
}

/// # [Macro DMX Step](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-66-macro-dmx-step-attributes)
/// This section defines a DMX step (XML node `<MacroDMXStep>`). The currently
/// defined XML attributes of the macro DMX step are specified in
/// [table 66](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-66).
///
/// As children the macro DMX -Step has a list of a [MacroDmxValue].
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde_inline_default]
pub struct MacroDmxStep {
    /// Duration of a step; Default value: 1; Unit: seconds.
    #[serde(rename = "Duration")]
    #[serde_inline_default(1.0)]
    pub duration: f32,

    #[allow(missing_docs)]
    #[serde(rename = "$value", default = "Vec::new")]
    pub dmx_values: Vec<MacroDmxValue>,
}

/// # [DMX Value](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-67-dmx-value-attributes)
///
/// This section defines the value for DMX channel (XML node). The currently
/// defined XML attributes of the DMX Value are specified in
/// [table 67](https://gdtf.eu/gdtf/file-spec/dmx-mode-collect/#table-67).
///
/// The DMX value does not have any children.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct MacroDmxValue {
    /// Value of the DMX channel
    #[serde(rename = "Value")]
    pub value: DmxValue,

    /// Link to a DMX channel. Starting node DMX Channel collect.
    #[serde(rename = "DMXChannel")]
    pub dmx_channel: Node,
}
