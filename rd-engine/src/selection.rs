use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::{
    gdtf::{Gdtf, Name, attr::AttributeName},
    patch::{FixtureId, Patch},
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Selection {
    pub(crate) fixture_ids: Vec<FixtureId>,
    pub(crate) attribute_tree: AttributeTree,
}

impl Selection {
    pub(crate) fn new() -> Self {
        Self { fixture_ids: Vec::new(), attribute_tree: AttributeTree::default() }
    }

    pub fn fixture_ids(&self) -> &[FixtureId] {
        &self.fixture_ids
    }

    pub fn is_empty(&self) -> bool {
        self.fixture_ids.is_empty()
    }

    pub fn len(&self) -> usize {
        self.fixture_ids.len()
    }

    pub fn contains(&self, fixture: &FixtureId) -> bool {
        self.fixture_ids.contains(fixture)
    }

    pub fn attribute_tree(&self) -> &AttributeTree {
        &self.attribute_tree
    }

    pub fn unique_gdtds<'a>(&self, patch: &'a Patch) -> Vec<&'a Gdtf> {
        let mut unique_gdtfs = Vec::<&Gdtf>::new();
        for fixture_id in &self.fixture_ids {
            let Some(fixture) = patch.fixture(fixture_id) else { continue };

            let ftid = fixture.gdtf().fixture_type_id();
            if !unique_gdtfs.iter().any(|gdtf| gdtf.fixture_type_id() == ftid) {
                unique_gdtfs.push(fixture.gdtf());
            }
        }
        unique_gdtfs
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AttributeTree {
    attributes: BTreeMap<Name, BTreeSet<AttributeName>>,
}

impl AttributeTree {
    pub fn new<'a, I: IntoIterator<Item = &'a Gdtf>>(gdtfs: I) -> Self {
        let mut gdtfs = gdtfs.into_iter();

        let mut attributes = BTreeMap::<Name, BTreeSet<AttributeName>>::new();

        let Some((reference, others)) = gdtfs.next().map(|first| (first, gdtfs.skip(1))) else {
            return Self::default();
        };

        for ref_attr in reference.attributes() {
            let Some(feature) = ref_attr.feature(reference) else { continue };
            attributes.entry(feature.name().clone()).or_default().insert(ref_attr.name().clone());
        }

        for other in others {
            let mut other_attributes = HashMap::<Name, HashSet<AttributeName>>::new();
            for other_attr in other.attributes() {
                let Some(feature) = other_attr.feature(other) else { continue };
                other_attributes
                    .entry(feature.name().clone())
                    .or_default()
                    .insert(other_attr.name().clone());
            }

            attributes.retain(|feature_name, attribute_names| {
                if let Some(other_set) = other_attributes.get(feature_name) {
                    attribute_names.retain(|attr_name| other_set.contains(attr_name));
                    !attribute_names.is_empty()
                } else {
                    false
                }
            });
        }

        Self { attributes }
    }

    pub fn feature_names(&self) -> impl Iterator<Item = &Name> {
        self.attributes.keys()
    }

    pub fn attributes(&self, feature_name: &Name) -> impl Iterator<Item = &AttributeName> {
        self.attributes.get(feature_name).into_iter().flatten()
    }
}
