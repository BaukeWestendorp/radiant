use std::str::FromStr;

use crate::error::Error;
use crate::raw::{
    RawActivationGroup, RawAttribute, RawAttributeDefinitions, RawFeature, RawFeatureGroup,
};
use crate::{parse_color_cie, parse_name, parse_node, ColorCIE, Node};

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeDefinitions {
    pub activation_groups: Vec<ActivationGroup>,
    pub feature_groups: Vec<FeatureGroup>,
    pub attributes: Vec<Attribute>,
}

impl TryFrom<RawAttributeDefinitions> for AttributeDefinitions {
    type Error = Error;

    fn try_from(value: RawAttributeDefinitions) -> Result<Self, Self::Error> {
        Ok(Self {
            activation_groups: value
                .activation_groups
                .map(|ag| ag.groups)
                .unwrap_or_default()
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            feature_groups: value
                .feature_groups
                .groups
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            attributes: value
                .attributes
                .attributes
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActivationGroup {
    pub name: String,
}

impl TryFrom<RawActivationGroup> for ActivationGroup {
    type Error = Error;

    fn try_from(value: RawActivationGroup) -> Result<Self, Self::Error> {
        Ok(Self {
            name: parse_name(value.name)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FeatureGroup {
    pub name: String,
    pub pretty_name: String,
    pub features: Vec<Feature>,
}

impl TryFrom<RawFeatureGroup> for FeatureGroup {
    type Error = Error;

    fn try_from(value: RawFeatureGroup) -> Result<Self, Self::Error> {
        let name = parse_name(value.name)?;
        Ok(Self {
            name: name.clone(),
            pretty_name: value.pretty.unwrap_or(name),
            features: value
                .features
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Feature {
    pub name: String,
}

impl TryFrom<RawFeature> for Feature {
    type Error = Error;

    fn try_from(value: RawFeature) -> Result<Self, Self::Error> {
        Ok(Self {
            name: parse_name(value.name)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub pretty_name: String,
    activation_group: Option<Node>,
    feature: Node,
    main_attribute: Option<Node>,
    pub physical_unit: PhysicalUnit,
    pub color: Option<ColorCIE>,
}

impl Attribute {
    pub fn activation_group<'a>(
        &'a self,
        activation_groups: &'a [ActivationGroup],
    ) -> Option<&ActivationGroup> {
        self.activation_group
            .as_ref()
            .and_then(|node| activation_groups.iter().find(|ag| ag.name == node[0]))
    }

    pub fn feature<'a>(&'a self, feature_groups: &'a [FeatureGroup]) -> Option<&Feature> {
        feature_groups
            .iter()
            .find(|fg| fg.name == self.feature[0])
            .and_then(|fg| fg.features.iter().find(|f| f.name == self.feature[1]))
    }

    pub fn main_attribute<'a>(&'a self, attributes: &'a [Attribute]) -> Option<&Attribute> {
        self.main_attribute
            .as_ref()
            .and_then(|node| attributes.iter().find(|a| a.name == node[0]))
    }
}

impl TryFrom<RawAttribute> for Attribute {
    type Error = Error;

    fn try_from(value: RawAttribute) -> Result<Self, Self::Error> {
        let name = parse_name(value.name).unwrap();
        Ok(Self {
            name: name.clone(),
            pretty_name: value.pretty.unwrap_or(name),
            activation_group: value.activation_group.map(parse_node).transpose()?,
            feature: parse_node(value.feature)?,
            main_attribute: value.main_attribute.map(parse_node).transpose()?,
            physical_unit: value.physical_unit.parse()?,
            color: value.color.map(parse_color_cie).transpose()?,
        })
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum PhysicalUnit {
    /// Unitless
    #[default]
    None,
    /// Percent
    Percent,
    /// Length (m)
    Length,
    /// Mass (kg)
    Mass,
    /// Time (s)
    Time,
    /// Temperature (K)
    Temperature,
    /// LuminousIntensity (cd)
    LuminousIntensity,
    /// Angle (degree)
    Angle,
    /// Force (N)
    Force,
    /// Frequency (Hz)
    Frequency,
    /// Current (A)
    Current,
    /// Voltage (V)
    Voltage,
    /// Power (W)
    Power,
    /// Energy (J)
    Energy,
    /// Area (m2)
    Area,
    /// Volume (m3)
    Volume,
    /// Speed (m/s)
    Speed,
    /// Acceleration (m/s2)
    Acceleration,
    /// AngularSpeed (degree/s)
    AngularSpeed,
    /// AngularAccc (degree/s2)
    AngularAccc,
    /// WaveLength (nm)
    WaveLength,
    /// ColorComponent
    ColorComponent,
}

impl FromStr for PhysicalUnit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Self::None),
            "Percent" => Ok(Self::Percent),
            "Length" => Ok(Self::Length),
            "Mass" => Ok(Self::Mass),
            "Time" => Ok(Self::Time),
            "Temperature" => Ok(Self::Temperature),
            "LuminousIntensity" => Ok(Self::LuminousIntensity),
            "Angle" => Ok(Self::Angle),
            "Force" => Ok(Self::Force),
            "Frequency" => Ok(Self::Frequency),
            "Current" => Ok(Self::Current),
            "Voltage" => Ok(Self::Voltage),
            "Power" => Ok(Self::Power),
            "Energy" => Ok(Self::Energy),
            "Area" => Ok(Self::Area),
            "Volume" => Ok(Self::Volume),
            "Speed" => Ok(Self::Speed),
            "Acceleration" => Ok(Self::Acceleration),
            "AngularSpeed" => Ok(Self::AngularSpeed),
            "AngularAccc" => Ok(Self::AngularAccc),
            "WaveLength" => Ok(Self::WaveLength),
            "ColorComponent" => Ok(Self::ColorComponent),
            _ => Err(Self::Err::ParseError(format!(
                "Invalid value for PhysicalUnit: {}",
                s
            ))),
        }
    }
}
