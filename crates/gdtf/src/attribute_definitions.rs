use std::rc::Rc;

use anyhow::{anyhow, Result};

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
    // FIXME: pub main_attribute: Option<Rc<Attribute>>,
    // FIXME: pub physical_unit: PhysicalUnit,
    // FIXME: pub color: ColorCie,
}

impl Attribute {
    pub(crate) fn from_raw(raw: RawAttribute, feature_groups: &[Rc<FeatureGroup>]) -> Result<Self> {
        let split = raw.feature.split('.').collect::<Vec<_>>();
        if split.len() != 2 {
            return Err(anyhow!("Invalid node length for feature: {}", raw.feature));
        }
        let feature_group_name = split[0];
        let feature_name = split[1];

        let feature = feature_groups
            .iter()
            .find(|fg| fg.name == feature_group_name)
            .and_then(|fg| fg.features.iter().find(|f| f.name == feature_name))
            .unwrap()
            .clone();

        Ok(Self {
            name: parse_name(raw.name)?,
            pretty_name: raw.pretty,
            feature,
        })
    }
}
