use std::{
    collections::HashMap,
    fmt,
    str::{self, FromStr},
    time::Duration,
};

use crate::mvr_gdtf::{
    gdtf::{
        Gdtf, Name, NodePath,
        attr::{Attribute, AttributeName, SubPhysicalUnit},
        bundle,
        geo::Geometry,
        phys::{ColorSpace, DmxProfile, Emitter, Filter, Gamut},
        wheel::Wheel,
    },
    util,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DmxMode {
    name: Name,
    description: Option<String>,
    geometry: NodePath,

    dmx_channels: Vec<DmxChannel>,
    dmx_channels_by_name: HashMap<Name, usize>,
    dmx_channels_by_geometry: HashMap<Name, usize>,
    relations: Vec<Relation>,
    relations_by_name: HashMap<Name, usize>,
    ft_macros: Vec<FtMacro>,
    ft_macros_by_name: HashMap<Name, usize>,
}

impl DmxMode {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn geometry_node(&self) -> &NodePath {
        &self.geometry
    }

    pub fn geometry<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Geometry> {
        let geometry = self.geometry.parts().get(0)?;
        gdtf.geometry(geometry)
    }

    pub fn dmx_channels(&self) -> &[DmxChannel] {
        &self.dmx_channels
    }

    pub fn dmx_channel(&self, name: &Name) -> Option<&DmxChannel> {
        let ix = self.dmx_channels_by_name.get(name)?;
        self.dmx_channels.get(*ix)
    }

    pub fn dmx_channel_by_geometry(&self, geometry_name: &Name) -> Option<&DmxChannel> {
        let ix = self.dmx_channels_by_geometry.get(geometry_name)?;
        self.dmx_channels.get(*ix)
    }

    pub fn relations(&self) -> &[Relation] {
        &self.relations
    }

    pub fn relation(&self, name: &Name) -> Option<&Relation> {
        let ix = self.relations_by_name.get(name)?;
        self.relations.get(*ix)
    }

    pub fn ft_macros(&self) -> &[FtMacro] {
        &self.ft_macros
    }

    pub fn ft_macro(&self, name: &Name) -> Option<&FtMacro> {
        let ix = self.ft_macros_by_name.get(name)?;
        self.ft_macros.get(*ix)
    }
}

impl bundle::FromBundle for DmxMode {
    type Source = bundle::DmxMode;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let dmx_channels: Vec<DmxChannel> = source
            .dmx_channels
            .dmx_channels
            .iter()
            .map(|dc| DmxChannel::from_bundle(dc, bundle))
            .collect();
        let dmx_channels_by_name: HashMap<Name, usize> =
            dmx_channels.iter().enumerate().map(|(ix, dc)| (dc.name().clone(), ix)).collect();
        let dmx_channels_by_geometry: HashMap<Name, usize> = dmx_channels
            .iter()
            .enumerate()
            .map(|(ix, dc)| (dc.geometry_name().clone(), ix))
            .collect();
        let relations: Vec<Relation> = source
            .relations
            .iter()
            .flat_map(|r| r.relations.iter().map(|r| Relation::from_bundle(r, bundle)))
            .collect();
        let relations_by_name: HashMap<Name, usize> =
            relations.iter().enumerate().map(|(ix, r)| (r.name().clone(), ix)).collect();
        let ft_macros: Vec<FtMacro> = source
            .ft_macros
            .iter()
            .flat_map(|ftm| ftm.ft_macros.iter().map(|ftm| FtMacro::from_bundle(ftm, bundle)))
            .collect();
        let ft_macros_by_name: HashMap<Name, usize> =
            ft_macros.iter().enumerate().map(|(ix, ftm)| (ftm.name().clone(), ix)).collect();

        Self {
            name: Name::new(&source.name),
            description: match source.description.as_deref() {
                Some("") | None => None,
                Some(s) => Some(s.to_string()),
            },
            geometry: match &source.geometry {
                Some(name) => NodePath::from_str(name).unwrap(),
                None => {
                    let first_geometry_name = bundle
                        .description()
                        .fixture_type
                        .geometries
                        .children
                        .first()
                        .expect("FIXME: Find out what to do in this case")
                        .name();
                    NodePath::from_str(first_geometry_name).unwrap()
                }
            },
            dmx_channels,
            dmx_channels_by_name,
            dmx_channels_by_geometry,
            relations,
            relations_by_name,
            ft_macros,
            ft_macros_by_name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DmxChannel {
    dmx_break: DmxBreak,
    offset: DmxOffset,
    initial_function: Option<NodePath>,
    highlight: Option<DmxValue>,
    geometry: Name,

    logical_channels: Vec<LogicalChannel>,
    logical_channels_by_attribute: HashMap<AttributeName, usize>,

    name: Name,
}

impl DmxChannel {
    pub fn dmx_break(&self) -> DmxBreak {
        self.dmx_break
    }

    pub fn offset(&self) -> &DmxOffset {
        &self.offset
    }

    pub fn initial_function_node(&self) -> Option<&NodePath> {
        self.initial_function.as_ref()
    }

    pub fn initial_function<'a>(&'a self) -> Option<(&'a LogicalChannel, &'a ChannelFunction)> {
        match &self.initial_function {
            Some(initial_function_node) => {
                let mut dmx_channel_split =
                    initial_function_node.parts().get(0)?.as_str().split('_');
                let dmx_channel_geometry = dmx_channel_split.next()?;

                let lc_attribute = initial_function_node.parts().get(1)?;
                let logical_channel =
                    self.logical_channel(&AttributeName::from_str(lc_attribute.as_str()).unwrap())?;

                let cf_name = initial_function_node.parts().get(2)?;
                let channel_function = logical_channel.channel_function(&cf_name)?;

                if dmx_channel_geometry != self.geometry_name().as_str() {
                    return None;
                }

                Some((logical_channel, channel_function))
            }
            None => {
                let first_channel = self.logical_channels.first()?;
                let first_function = first_channel.channel_functions().first()?;
                Some((first_channel, first_function))
            }
        }
    }

    pub fn highlight(&self) -> Option<DmxValue> {
        self.highlight
    }

    pub fn geometry_name(&self) -> &Name {
        &self.geometry
    }

    pub fn geometry<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Geometry> {
        gdtf.geometry(self.geometry_name())
    }

    pub fn logical_channels(&self) -> &[LogicalChannel] {
        &self.logical_channels
    }

    pub fn logical_channel(&self, attribute_name: &AttributeName) -> Option<&LogicalChannel> {
        let ix = self.logical_channels_by_attribute.get(attribute_name)?;
        Some(&self.logical_channels[*ix])
    }

    pub fn name(&self) -> &Name {
        &self.name
    }
}

impl bundle::FromBundle for DmxChannel {
    type Source = bundle::DmxChannel;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let logical_channels = source
            .logical_channels
            .iter()
            .map(|lc| LogicalChannel::from_bundle(lc, bundle))
            .collect::<Vec<_>>();
        let logical_channels_by_attribute = logical_channels
            .iter()
            .enumerate()
            .filter_map(|(ix, lc)| {
                lc.attribute_node()
                    .parts()
                    .get(0)
                    .map(|name| (AttributeName::from_str(name.as_str()).unwrap(), ix))
            })
            .collect();

        let geometry = Name::new(&source.geometry);

        let name = {
            let attribute_name = logical_channels
                .first()
                .and_then(|lc| lc.attribute_node().parts().get(0))
                .expect("Missing attribute name")
                .clone();
            Name::new(&format!("{}_{}", geometry, attribute_name))
        };

        Self {
            dmx_break: DmxBreak::from_str(&source.dmx_break).unwrap(),
            offset: DmxOffset::from_str(&source.offset).unwrap(),
            initial_function: source
                .initial_function
                .as_ref()
                .map(|node| NodePath::from_str(node).unwrap()),
            highlight: if source.highlight.trim() == "None" {
                None
            } else {
                Some(DmxValue::from_str(&source.highlight).unwrap())
            },
            geometry,
            logical_channels,
            logical_channels_by_attribute,

            name,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DmxBreak {
    Overwrite,
    Break(u8),
}

impl str::FromStr for DmxBreak {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Overwrite" {
            Ok(DmxBreak::Overwrite)
        } else {
            match s.trim().parse::<u32>() {
                Ok(n) if n > 0 => Ok(DmxBreak::Break(n as u8)),
                Ok(_) => todo!(),
                Err(_) => todo!(),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogicalChannel {
    attribute: NodePath,
    snap: Snap,
    master: Master,
    mib_fade: Duration,
    dmx_change_time_limit: Duration,

    channel_functions: Vec<ChannelFunction>,
    channel_functions_by_name: HashMap<Name, usize>,
}

impl LogicalChannel {
    pub fn attribute_node(&self) -> &NodePath {
        &self.attribute
    }

    pub fn attribute<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Attribute> {
        let attribute_name = self.attribute_node().parts().get(0)?;
        gdtf.attribute(&AttributeName::from_str(attribute_name.as_str()).unwrap())
    }

    pub fn snap(&self) -> Snap {
        self.snap
    }

    pub fn master(&self) -> Master {
        self.master
    }

    pub fn mib_fade(&self) -> Duration {
        self.mib_fade
    }

    pub fn dmx_change_time_limit(&self) -> Duration {
        self.dmx_change_time_limit
    }

    pub fn channel_functions(&self) -> &[ChannelFunction] {
        &self.channel_functions
    }

    pub fn channel_function(&self, name: &Name) -> Option<&ChannelFunction> {
        let ix = self.channel_functions_by_name.get(name)?;
        Some(&self.channel_functions[*ix])
    }
}

impl bundle::FromBundle for LogicalChannel {
    type Source = bundle::LogicalChannel;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let channel_functions = source
            .channel_functions
            .iter()
            .enumerate()
            .map(|(ix, cf)| ChannelFunction::from_bundle(&(ix, cf.clone()), bundle))
            .collect::<Vec<_>>();
        let channel_functions_by_name =
            channel_functions.iter().enumerate().map(|(ix, cf)| (cf.name().clone(), ix)).collect();

        Self {
            attribute: NodePath::from_str(&source.attribute).unwrap(),
            snap: Snap::from_bundle(&source.snap, bundle),
            master: Master::from_bundle(&source.master, bundle),
            mib_fade: util::parse_possibly_negative_duration(source.mib_fade.unwrap_or(0.0)),
            dmx_change_time_limit: util::parse_possibly_negative_duration(
                source.dmx_change_time_limit.unwrap_or(0.0),
            ),
            channel_functions,
            channel_functions_by_name,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Snap {
    #[default]
    No,
    Yes,
    Off,
    On,
}

impl bundle::FromBundle for Snap {
    type Source = bundle::Snap;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        match source {
            bundle::Snap::Yes => Snap::Yes,
            bundle::Snap::No => Snap::No,
            bundle::Snap::On => Snap::On,
            bundle::Snap::Off => Snap::Off,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Master {
    #[default]
    None,
    Grand,
    Group,
}

impl bundle::FromBundle for Master {
    type Source = bundle::Master;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        match source {
            bundle::Master::None => Master::None,
            bundle::Master::Grand => Master::Grand,
            bundle::Master::Group => Master::Group,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelFunction {
    name: Name,
    attribute: NodePath,
    original_attribute: Option<String>,
    dmx_from: DmxValue,
    default: DmxValue,
    physical_from: f32,
    physical_to: f32,
    real_fade: Duration,
    real_acceleration: f32,
    wheel: Option<NodePath>,
    emitter: Option<NodePath>,
    filter: Option<NodePath>,
    color_space: Option<NodePath>,
    gamut: Option<NodePath>,
    mode_master: Option<ModeMaster>,
    dmx_profile: Option<NodePath>,
    min: f32,
    max: f32,
    custom_name: Option<String>,

    channel_sets: Vec<ChannelSet>,
    channel_sets_by_name: HashMap<Name, usize>,

    sub_channel_sets: Vec<SubChannelSet>,
    sub_channel_sets_by_name: HashMap<Name, usize>,
}

impl ChannelFunction {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn attribute_node(&self) -> &NodePath {
        &self.attribute
    }

    pub fn attribute<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Attribute> {
        let attribute_name = self.attribute_node().parts().get(0)?;
        gdtf.attribute(&AttributeName::from_str(attribute_name.as_str()).unwrap())
    }

    pub fn original_attribute(&self) -> Option<&String> {
        self.original_attribute.as_ref()
    }

    pub fn dmx_from(&self) -> DmxValue {
        self.dmx_from
    }

    pub fn default(&self) -> DmxValue {
        self.default
    }

    pub fn physical_from(&self) -> f32 {
        self.physical_from
    }

    pub fn physical_to(&self) -> f32 {
        self.physical_to
    }

    pub fn real_fade(&self) -> Duration {
        self.real_fade
    }

    pub fn real_acceleration(&self) -> f32 {
        self.real_acceleration
    }

    pub fn wheel_node(&self) -> Option<&NodePath> {
        self.wheel.as_ref()
    }

    pub fn wheel<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Wheel> {
        let wheel_name = self.wheel_node()?.parts().get(0)?;
        gdtf.wheel(wheel_name)
    }

    pub fn emitter_node(&self) -> Option<&NodePath> {
        self.emitter.as_ref()
    }

    pub fn emitter<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Emitter> {
        let emitter_name = self.emitter_node()?.parts().get(0)?;
        gdtf.emitter(emitter_name)
    }

    pub fn filter_node(&self) -> Option<&NodePath> {
        self.filter.as_ref()
    }

    pub fn filter<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Filter> {
        let filter_name = self.filter_node()?.parts().get(0)?;
        gdtf.filter(filter_name)
    }

    pub fn color_space_node(&self) -> Option<&NodePath> {
        self.color_space.as_ref()
    }

    pub fn color_space<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a ColorSpace> {
        let color_space_name = self.color_space_node()?.parts().get(0)?;
        if gdtf.color_space().is_some_and(|cs| cs.name() == Some(color_space_name)) {
            return gdtf.color_space();
        }
        gdtf.additional_color_space(color_space_name)
    }

    pub fn gamut_node(&self) -> Option<&NodePath> {
        self.gamut.as_ref()
    }

    pub fn gamut<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a Gamut> {
        let gamut_name = self.gamut_node()?.parts().get(0)?;
        gdtf.gamut(gamut_name)
    }

    pub fn mode_master(&self) -> Option<&ModeMaster> {
        self.mode_master.as_ref()
    }

    pub fn dmx_profile_node(&self) -> Option<&NodePath> {
        self.dmx_profile.as_ref()
    }

    pub fn dmx_profile<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a DmxProfile> {
        let dmx_profile_name = self.dmx_profile_node()?.parts().get(0)?;
        gdtf.dmx_profile(dmx_profile_name)
    }

    pub fn min(&self) -> f32 {
        self.min
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn custom_name(&self) -> Option<&String> {
        self.custom_name.as_ref()
    }

    pub fn channel_sets(&self) -> &[ChannelSet] {
        &self.channel_sets
    }

    pub fn channel_set(&self, name: &Name) -> Option<&ChannelSet> {
        let ix = self.channel_sets_by_name.get(name)?;
        Some(&self.channel_sets[*ix])
    }

    pub fn sub_channel_sets(&self) -> &[SubChannelSet] {
        &self.sub_channel_sets
    }

    pub fn sub_channel_set(&self, name: &Name) -> Option<&SubChannelSet> {
        let ix = self.sub_channel_sets_by_name.get(name)?;
        Some(&self.sub_channel_sets[*ix])
    }

    pub fn node_path(
        &self,
        dmx_channel: &DmxChannel,
        logical_channel: &LogicalChannel,
    ) -> NodePath {
        NodePath::new(dmx_channel.name().clone())
            .join_path(logical_channel.attribute_node().clone())
            .join(self.name().clone())
    }
}

impl bundle::FromBundle for ChannelFunction {
    type Source = (usize, bundle::ChannelFunction);

    fn from_bundle((cf_ix, source): &Self::Source, bundle: &bundle::Bundle) -> Self {
        let channel_sets = source
            .channel_sets
            .iter()
            .map(|cs| ChannelSet::from_bundle(cs, bundle))
            .collect::<Vec<_>>();
        let channel_sets_by_name = channel_sets
            .iter()
            .enumerate()
            .filter_map(|(ix, cs)| cs.name().map(|name| (name.clone(), ix)))
            .collect();
        let sub_channel_sets = source
            .sub_channel_sets
            .iter()
            .map(|scs| SubChannelSet::from_bundle(scs, bundle))
            .collect::<Vec<_>>();
        let sub_channel_sets_by_name = sub_channel_sets
            .iter()
            .enumerate()
            .filter_map(|(ix, scs)| scs.name().map(|name| (name.clone(), ix)))
            .collect();

        Self {
            name: source
                .name
                .as_ref()
                .map(Name::new)
                .unwrap_or_else(|| Name::new(format!("{} {}", source.attribute, cf_ix + 1))),
            attribute: NodePath::from_str(&source.attribute).unwrap(),
            original_attribute: match source.original_attribute.as_str() {
                "" => None,
                other => Some(other.to_string()),
            },
            dmx_from: DmxValue::from_str(&source.dmx_from).unwrap(),
            default: DmxValue::from_str(&source.default).unwrap(),
            physical_from: source.physical_from.unwrap_or(0.0),
            physical_to: source.physical_to.unwrap_or(1.0),
            real_fade: util::parse_possibly_negative_duration(source.real_fade.unwrap_or(0.0)),
            real_acceleration: source.real_acceleration.unwrap_or(0.0),
            wheel: source.wheel.as_ref().map(|s| NodePath::from_str(&s).unwrap()),
            emitter: source.emitter.as_ref().map(|s| NodePath::from_str(&s).unwrap()),
            filter: source.filter.as_ref().map(|s| NodePath::from_str(&s).unwrap()),
            color_space: source.color_space.as_ref().map(|s| NodePath::from_str(&s).unwrap()),
            gamut: source.gamut.as_ref().map(|s| NodePath::from_str(&s).unwrap()),
            mode_master: source.mode_master.as_ref().map(|mm| ModeMaster {
                node: NodePath::from_str(mm).unwrap(),
                from: DmxValue::from_str(&source.mode_from).unwrap(),
                to: DmxValue::from_str(&source.mode_to).unwrap(),
            }),
            dmx_profile: source.dmx_profile.as_ref().map(|s| NodePath::from_str(s).unwrap()),
            min: source.min.unwrap_or(source.physical_from.unwrap_or(0.0)),
            max: source.max.unwrap_or(source.physical_to.unwrap_or(1.0)),
            custom_name: source.custom_name.clone(),
            channel_sets,
            channel_sets_by_name,
            sub_channel_sets,
            sub_channel_sets_by_name,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ModeMaster {
    node: NodePath,
    from: DmxValue,
    to: DmxValue,
}

impl ModeMaster {
    pub fn node_path(&self) -> &NodePath {
        &self.node
    }

    pub fn node<'a>(&self, dmx_mode: &'a DmxMode) -> Option<ModeMasterNode<'a>> {
        let parts = self.node.parts();
        let channel_name = parts.get(0)?;
        let channel = dmx_mode.dmx_channels().iter().find(|c| c.geometry_name() == channel_name)?;

        match (parts.get(1), parts.get(2)) {
            (Some(lc_attribute), Some(cf_attribute)) => {
                let logical_channel = channel
                    .logical_channel(&AttributeName::from_str(lc_attribute.as_str()).unwrap())?;
                let function = logical_channel.channel_function(&cf_attribute)?;
                Some(ModeMasterNode::ChannelFunction(channel, logical_channel, function))
            }
            _ => Some(ModeMasterNode::DmxChannel(channel)),
        }
    }

    pub fn from(&self) -> DmxValue {
        self.from
    }

    pub fn to(&self) -> DmxValue {
        self.to
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModeMasterNode<'a> {
    DmxChannel(&'a DmxChannel),
    ChannelFunction(&'a DmxChannel, &'a LogicalChannel, &'a ChannelFunction),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelSet {
    name: Option<Name>,
    dmx_from: DmxValue,
    physical_from: f32,
    physical_to: f32,
    wheel_slot_index: Option<u32>,
}

impl ChannelSet {
    pub fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }

    pub fn dmx_from(&self) -> DmxValue {
        self.dmx_from
    }

    pub fn physical_from(&self) -> f32 {
        self.physical_from
    }

    pub fn physical_to(&self) -> f32 {
        self.physical_to
    }

    pub fn wheel_slot_index(&self) -> Option<u32> {
        self.wheel_slot_index
    }
}

impl bundle::FromBundle for ChannelSet {
    type Source = bundle::ChannelSet;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self {
            name: source.name.as_ref().map(Name::new),
            dmx_from: DmxValue::from_str(&source.dmx_from).unwrap(),
            physical_from: source.physical_from.unwrap_or(0.0),
            physical_to: source.physical_to.unwrap_or(1.0),
            wheel_slot_index: source.wheel_slot_index.map(|i| i as u32),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubChannelSet {
    name: Option<Name>,
    physical_from: f32,
    physical_to: f32,
    sub_physical_unit: NodePath,
    dmx_profile: Option<NodePath>,
}

impl SubChannelSet {
    pub fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }

    pub fn physical_from(&self) -> f32 {
        self.physical_from
    }

    pub fn physical_to(&self) -> f32 {
        self.physical_to
    }

    pub fn sub_physical_unit_node(&self) -> &NodePath {
        &self.sub_physical_unit
    }

    pub fn sub_physical_unit<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a SubPhysicalUnit> {
        let attribute_name = self.sub_physical_unit.parts().get(0)?;
        let attribute = gdtf.attribute(&AttributeName::from_str(attribute_name.as_str()).ok()?)?;
        let sub_name = self.sub_physical_unit.parts().get(1)?;
        attribute
            .sub_physical_units()
            .iter()
            .find(|unit| unit.r#type().to_string().as_str() == sub_name.as_str())
    }

    pub fn dmx_profile_node(&self) -> Option<&NodePath> {
        self.dmx_profile.as_ref()
    }

    pub fn dmx_profile<'a>(&self, gdtf: &'a Gdtf) -> Option<&'a DmxProfile> {
        let profile_node = self.dmx_profile_node()?;
        let profile_name = profile_node.parts().get(0)?;
        gdtf.dmx_profile(profile_name)
    }
}

impl bundle::FromBundle for SubChannelSet {
    type Source = bundle::SubChannelSet;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self {
            name: source.name.as_ref().map(Name::new),
            physical_from: source.physical_from,
            physical_to: source.physical_to,
            sub_physical_unit: NodePath::from_str(&source.sub_physical_unit).unwrap(),
            dmx_profile: source.dmx_profile.as_ref().map(|s| NodePath::from_str(s).unwrap()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Relation {
    name: Name,
    master: NodePath,
    follower: NodePath,
    kind: RelationKind,
}

impl Relation {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn master_node(&self) -> &NodePath {
        &self.master
    }

    pub fn follower_node(&self) -> &NodePath {
        &self.follower
    }

    pub fn master<'a>(&self, dmx_mode: &'a DmxMode) -> Option<&'a DmxChannel> {
        let name = self.master.parts().get(0)?;
        dmx_mode.dmx_channels().iter().find(|c| c.name() == name)
    }

    pub fn follower<'a>(
        &self,
        dmx_mode: &'a DmxMode,
    ) -> Option<(&'a DmxChannel, &'a LogicalChannel, &'a ChannelFunction)> {
        let parts = self.follower.parts();
        let channel_name = parts.get(0)?;
        let dc = dmx_mode.dmx_channels().iter().find(|c| c.name() == channel_name)?;

        let lc_attribute = parts.get(1)?;
        let lc = dc.logical_channel(&AttributeName::from_str(lc_attribute.as_str()).unwrap())?;

        let cf_name = parts.get(2)?;
        let cf = lc.channel_function(&cf_name)?;

        Some((dc, lc, cf))
    }

    pub fn kind(&self) -> RelationKind {
        self.kind
    }
}

impl bundle::FromBundle for Relation {
    type Source = bundle::Relation;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            name: Name::new(&source.name),
            master: NodePath::from_str(&source.master).unwrap(),
            follower: NodePath::from_str(&source.follower).unwrap(),
            kind: RelationKind::from_bundle(&source.r#type, bundle),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationKind {
    Multiply,
    Override,
}

impl bundle::FromBundle for RelationKind {
    type Source = bundle::RelationType;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        match source {
            bundle::RelationType::Multiply => RelationKind::Multiply,
            bundle::RelationType::Override => RelationKind::Override,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FtMacro {
    name: Name,
    channel_function: Option<NodePath>,
    dmx: Vec<MacroDmx>,
}

impl FtMacro {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn channel_function_node(&self) -> Option<&NodePath> {
        self.channel_function.as_ref()
    }

    pub fn channel_function<'a>(
        &self,
        dmx_mode: &'a DmxMode,
    ) -> Option<(&'a DmxChannel, &'a LogicalChannel, &'a ChannelFunction)> {
        let node = self.channel_function.as_ref()?;
        let parts = node.parts();
        let channel_name = parts.get(0)?;
        let channel = dmx_mode.dmx_channels().iter().find(|c| c.geometry_name() == channel_name)?;
        let lc_attribute = parts.get(1)?;
        let logical_channel =
            channel.logical_channel(&AttributeName::from_str(lc_attribute.as_str()).unwrap())?;
        let cf_attribute = parts.get(2)?;
        let channel_function = logical_channel.channel_function(&cf_attribute)?;
        Some((channel, logical_channel, channel_function))
    }

    pub fn dmx(&self) -> &[MacroDmx] {
        &self.dmx
    }
}

impl bundle::FromBundle for FtMacro {
    type Source = bundle::FtMacro;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            name: Name::new(&source.name),
            channel_function: source
                .channel_function
                .as_ref()
                .map(|s| NodePath::from_str(s).unwrap()),
            dmx: source.macro_dmx.iter().map(|v| MacroDmx::from_bundle(v, bundle)).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroDmx {
    steps: Vec<MacroDmxStep>,
}

impl MacroDmx {
    pub fn steps(&self) -> &[MacroDmxStep] {
        &self.steps
    }
}

impl bundle::FromBundle for MacroDmx {
    type Source = bundle::MacroDmx;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            steps: source
                .macro_dmx_steps
                .iter()
                .map(|s| MacroDmxStep::from_bundle(s, bundle))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroDmxStep {
    duration: Duration,
    values: Vec<MacroDmxValue>,
}

impl MacroDmxStep {
    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn values(&self) -> &[MacroDmxValue] {
        &self.values
    }
}

impl bundle::FromBundle for MacroDmxStep {
    type Source = bundle::MacroDmxStep;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        Self {
            duration: util::parse_possibly_negative_duration(source.duration.unwrap_or_default()),
            values: source
                .macro_dmx_values
                .iter()
                .map(|v| MacroDmxValue::from_bundle(v, bundle))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroDmxValue {
    value: DmxValue,
    dmx_channel: NodePath,
}

impl MacroDmxValue {
    pub fn value(&self) -> DmxValue {
        self.value
    }

    pub fn dmx_channel_node(&self) -> &NodePath {
        &self.dmx_channel
    }

    pub fn dmx_channel<'a>(&self, dmx_mode: &'a DmxMode) -> Option<&'a DmxChannel> {
        let name = self.dmx_channel.parts().get(0)?;
        dmx_mode.dmx_channel(name)
    }
}

impl bundle::FromBundle for MacroDmxValue {
    type Source = bundle::MacroDmxValue;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Self {
            value: DmxValue::from_str(&source.value).unwrap(),
            dmx_channel: NodePath::from_str(&source.dmx_channel).unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DmxValue {
    value: u32,
    bytes: u8,
    shifting: bool,
}

impl DmxValue {
    pub fn from_u8(value: u8, shifting: bool) -> Self {
        DmxValue { value: value as u32, bytes: 1, shifting }
    }

    pub fn from_u16(value: u16, shifting: bool) -> Self {
        DmxValue { value: value as u32, bytes: 2, shifting }
    }

    pub fn from_u24(value: u32, shifting: bool) -> Self {
        if value > 0xFFFFFF {
            todo!();
        }
        DmxValue { value, bytes: 3, shifting }
    }

    pub fn from_u32(value: u32, shifting: bool) -> Self {
        DmxValue { value, bytes: 4, shifting }
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn bytes(&self) -> u8 {
        self.bytes
    }

    pub fn shifting(&self) -> bool {
        self.shifting
    }

    pub fn to_normalized(&self) -> f64 {
        let bytes = self.bytes as u32;
        let max_value = (1u64 << (bytes * 8)) - 1;
        self.value as f64 / max_value as f64
    }
}

impl Default for DmxValue {
    fn default() -> Self {
        Self { value: 0, bytes: 1, shifting: false }
    }
}

impl str::FromStr for DmxValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value_str, bytes_str) = match s.split_once('/') {
            Some(v) => v,
            None => todo!("handle error"),
        };

        let (bytes_str, shifting) = match bytes_str.strip_suffix('s') {
            Some(stripped) => (stripped, true),
            None => (bytes_str, false),
        };

        let value = match value_str.parse::<u32>() {
            Ok(v) => v,
            Err(_) => todo!("handle error"),
        };
        let bytes = match bytes_str.parse::<u8>() {
            Ok(b) => b.max(1),
            Err(_) => todo!("handle error"),
        };

        match bytes {
            1 => Ok(DmxValue::from_u8(value as u8, shifting)),
            2 => Ok(DmxValue::from_u16(value as u16, shifting)),
            3 => Ok(DmxValue::from_u24(value as u32, shifting)),
            4 => Ok(DmxValue::from_u32(value as u32, shifting)),
            _ => todo!(),
        }
    }
}

impl fmt::Display for DmxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.shifting {
            write!(f, "{}/{}s", self.value, self.bytes)
        } else {
            write!(f, "{}/{}", self.value, self.bytes)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DmxOffset {
    Physical(Vec<u32>),
    Virtual,
}

impl str::FromStr for DmxOffset {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.is_empty() || s == "None" {
            return Ok(DmxOffset::Virtual);
        }

        let mut values = Vec::new();
        for part in s.split(',') {
            if part.is_empty() {
                todo!();
            }
            let v = part.trim().parse().unwrap();
            values.push(v);
        }

        Ok(DmxOffset::Physical(values))
    }
}
