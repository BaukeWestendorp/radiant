mod cache;
mod compositor;
mod mapper;

use crate::{
    dmx::Multiverse,
    mvr_gdtf::gdtf::attr::AttributeName,
    object::Objects,
    patch::{FixtureId, Patch},
    programmer::Programmer,
    value::{AttributeValue, AttributeValues},
};

#[derive(Clone, Default)]
pub struct Pipeline {
    cache: cache::PipelineCache,

    attribute_values: AttributeValues,
    multiverse: Multiverse,
    highlighted_fixtures: Vec<FixtureId>,
}

impl Pipeline {
    pub fn new(patch: &Patch) -> Self {
        Self {
            cache: cache::PipelineCache::new(patch),

            attribute_values: AttributeValues::new(),
            multiverse: Multiverse::new(),
            highlighted_fixtures: Vec::new(),
        }
    }

    pub(crate) fn resolve_attributes(
        &mut self,
        objects: &Objects,
        patch: &Patch,
        programmer: &Programmer,
        highlighted_fixtures: Vec<FixtureId>,
    ) -> anyhow::Result<()> {
        self.attribute_values =
            compositor::compose(objects, patch, programmer, &highlighted_fixtures, &self.cache)?;
        self.highlighted_fixtures = highlighted_fixtures;
        Ok(())
    }

    pub(crate) fn resolve_dmx(&mut self) {
        self.multiverse = mapper::map(&self.attribute_values, &self.cache);
    }

    pub fn attribute_info(
        &self,
        fixture_id: &FixtureId,
        attribute: &AttributeName,
        programmer: &Programmer,
    ) -> Option<AttributeInfo> {
        let current_value = self.attribute_values.get(fixture_id, attribute)?;

        let source = if self.highlighted_fixtures.contains(fixture_id)
            && self.cache().get(fixture_id, attribute).is_some_and(|cf| cf.highlight.is_some())
        {
            AttributeSource::Highlight
        } else if programmer.values().contains(fixture_id, attribute) {
            AttributeSource::Programmer
        } else if self.is_default_value(fixture_id, attribute, current_value) {
            AttributeSource::Default
        } else {
            AttributeSource::Playback
        };

        Some(AttributeInfo { value: current_value, source })
    }

    fn is_default_value(
        &self,
        fixture_id: &FixtureId,
        attribute: &AttributeName,
        current_value: AttributeValue,
    ) -> bool {
        if let Some(default_val) = self.cache.initial_defaults().get(fixture_id, attribute) {
            default_val == current_value
        } else {
            false
        }
    }

    pub fn cache(&self) -> &cache::PipelineCache {
        &self.cache
    }

    pub fn attribute_values(&self) -> &AttributeValues {
        &self.attribute_values
    }

    pub fn multiverse(&self) -> &Multiverse {
        &self.multiverse
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeSource {
    Default,
    Playback,
    Programmer,
    Highlight,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttributeInfo {
    pub value: AttributeValue,
    pub source: AttributeSource,
}
