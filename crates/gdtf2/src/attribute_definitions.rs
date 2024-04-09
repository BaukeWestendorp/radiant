use std::rc::Rc;

use anyhow::Result;

use crate::parse_name;
use crate::raw::{RawAttribute, RawAttributeDefinitions};

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeDefinitions {
    // FIXME: pub activation_groups: Vec<ActivationGroup>,
    // FIXME: pub feature_groups: Vec<FeatureGroup>,
    pub attributes: Vec<Rc<Attribute>>,
}

impl AttributeDefinitions {
    pub(crate) fn from_raw(raw: RawAttributeDefinitions) -> Result<Self> {
        Ok(Self {
            attributes: raw
                .attributes
                .attributes
                .into_iter()
                .map(|attribute| Attribute::from_raw(attribute).map(Rc::new))
                .collect::<Result<_>>()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub pretty_name: Option<String>,
    // FIXME: pub activation_group: Option<Rc<ActivationGroup>>,
    // FIXME: pub feature_group: Rc<Feature>,
    // FIXME: pub main_attribute: Option<Rc<Attribute>>,
    // FIXME: pub physical_unit: PhysicalUnit,
    // FIXME: pub color: ColorCie,
}

impl Attribute {
    pub(crate) fn from_raw(raw: RawAttribute) -> Result<Self> {
        Ok(Self {
            name: parse_name(raw.name)?,
            pretty_name: raw.pretty,
        })
    }
}
