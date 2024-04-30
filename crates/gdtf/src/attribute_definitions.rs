use std::rc::Rc;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

use crate::parse_name;
use crate::raw::{RawAttribute, RawAttributeDefinitions, RawFeature, RawFeatureGroup};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributeDefinitions {
    // FIXME: pub activation_groups: Vec<ActivationGroup>,
    pub feature_groups: Vec<Rc<FeatureGroup>>,
    pub attributes: Vec<Rc<Attribute>>,
}

impl AttributeDefinitions {
    pub(crate) fn from_raw(raw: RawAttributeDefinitions) -> Result<Self> {
        let feature_groups = raw
            .feature_groups
            .groups
            .into_iter()
            .map(|feature_group| FeatureGroup::from_raw(feature_group).map(Rc::new))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            attributes: raw
                .attributes
                .attributes
                .into_iter()
                .map(|attribute| Attribute::from_raw(attribute, &feature_groups).map(Rc::new))
                .collect::<Result<_>>()?,
            feature_groups,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FeatureGroup {
    pub name: String,
    pub pretty_name: Option<String>,
    pub features: Vec<Rc<Feature>>,
}

impl FeatureGroup {
    pub(crate) fn from_raw(raw: RawFeatureGroup) -> Result<Self> {
        Ok(Self {
            name: parse_name(raw.name)?,
            pretty_name: raw.pretty,
            features: raw
                .features
                .into_iter()
                .map(|feature| Feature::from_raw(feature).map(Rc::new))
                .collect::<Result<_>>()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Feature {
    pub name: String,
}

impl Feature {
    pub(crate) fn from_raw(raw: RawFeature) -> Result<Self> {
        Ok(Self {
            name: parse_name(raw.name)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attribute {
    pub name: String,
    pub pretty_name: Option<String>,
    // FIXME: pub activation_group: Option<Rc<ActivationGroup>>,
    pub feature: Rc<Feature>,
    // FIXME: We should make this an Rc<Attribute>, but we haven't parsed all attributes yet.
    pub main_attribute: Option<String>,
    pub physical_unit: PhysicalUnit,
    // FIXME: pub color: ColorCie,

    // HELPERS:
    pub feature_group: Rc<FeatureGroup>,
}

impl Attribute {
    pub(crate) fn from_raw(raw: RawAttribute, feature_groups: &[Rc<FeatureGroup>]) -> Result<Self> {
        let split = raw.feature.split('.').collect::<Vec<_>>();
        if split.len() != 2 {
            return Err(anyhow!("Invalid node length for feature: {}", raw.feature));
        }
        let feature_group_name = split[0];
        let feature_name = split[1];

        let feature_group = feature_groups
            .iter()
            .find(|fg| fg.name == feature_group_name)
            .ok_or_else(|| anyhow!("Unknown feature group: {}", feature_group_name))?
            .clone();

        let feature = feature_group
            .features
            .iter()
            .find(|f| f.name == feature_name)
            .ok_or_else(|| anyhow!("Unknown feature: {}", feature_name))?
            .clone();

        Ok(Self {
            name: parse_name(raw.name)?,
            pretty_name: raw.pretty,
            feature,
            main_attribute: raw.main_attribute,
            physical_unit: raw.physical_unit.parse()?,

            feature_group,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum PhysicalUnit {
    #[default]
    None,
    Percent,
    Length,
    Mass,
    Time,
    Temperature,
    LuminousIntensity,
    Angle,
    Force,
    Frequency,
    Current,
    Voltage,
    Power,
    Energy,
    Area,
    Volume,
    Speed,
    Acceleration,
    AngularSpeed,
    AngularAcceleration,
    WaveLength,
    ColorComponent,
}

impl FromStr for PhysicalUnit {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "none" => Ok(Self::None),
            "percent" => Ok(Self::Percent),
            "length" => Ok(Self::Length),
            "mass" => Ok(Self::Mass),
            "time" => Ok(Self::Time),
            "temperature" => Ok(Self::Temperature),
            "luminousintensity" => Ok(Self::LuminousIntensity),
            "angle" => Ok(Self::Angle),
            "force" => Ok(Self::Force),
            "frequency" => Ok(Self::Frequency),
            "current" => Ok(Self::Current),
            "voltage" => Ok(Self::Voltage),
            "power" => Ok(Self::Power),
            "energy" => Ok(Self::Energy),
            "area" => Ok(Self::Area),
            "volume" => Ok(Self::Volume),
            "speed" => Ok(Self::Speed),
            "acceleration" => Ok(Self::Acceleration),
            "angularspeed" => Ok(Self::AngularSpeed),
            "angularacceleration" => Ok(Self::AngularAcceleration),
            "wavelength" => Ok(Self::WaveLength),
            "colorcomponent" => Ok(Self::ColorComponent),
            _ => Err(anyhow!("Unknown physical unit: {}", s)),
        }
    }
}
