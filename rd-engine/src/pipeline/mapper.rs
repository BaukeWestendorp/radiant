use std::collections::HashSet;

use crate::{
    dmx::Multiverse,
    mvr_gdtf::gdtf::{attr::AttributeName, dmx::RelationKind},
    patch::FixtureId,
    pipeline::cache::{ChannelFunctionKind, PipelineCache},
    value::{AttributeValue, AttributeValues, ClampedValue},
};

#[derive(Default)]
pub struct Mapper {}

impl Mapper {
    pub fn new() -> Self {
        Self {}
    }

    pub fn map(&self, attributes: &AttributeValues, cache: &PipelineCache) -> Multiverse {
        let mut multiverse = Multiverse::new();
        let mut deferred_relations = Vec::new();

        for (fixture_id, attribute, value) in attributes.values() {
            set_value(
                fixture_id,
                attribute,
                value,
                cache,
                attributes,
                &mut multiverse,
                &mut deferred_relations,
            );
        }

        let mut visited = HashSet::new();

        loop {
            let batch = std::mem::take(&mut deferred_relations);
            if batch.is_empty() {
                break;
            }
            for (fixture_id, attribute, value) in &batch {
                if !visited.insert((fixture_id.clone(), attribute.clone())) {
                    log::warn!(
                        "Cycle detected in relations for attribute '{}' of fixture '{}', skipping",
                        attribute,
                        fixture_id
                    );
                    continue;
                }
                set_value(
                    fixture_id,
                    attribute,
                    value,
                    cache,
                    attributes,
                    &mut multiverse,
                    &mut deferred_relations,
                );
            }
        }

        multiverse
    }
}

fn set_value(
    fixture_id: &FixtureId,
    attribute: &AttributeName,
    value: &AttributeValue,
    cache: &PipelineCache,
    attributes: &AttributeValues,
    multiverse: &mut Multiverse,
    deferred_relations: &mut Vec<(FixtureId, AttributeName, AttributeValue)>,
) {
    let Some(info) = cache.get(fixture_id, attribute) else {
        log::error!(
            "Could not find cache for attribute '{}' of fixture with id '{}'",
            attribute,
            fixture_id
        );
        return;
    };

    let clamped = value.to_clamped_value(info.min, info.max);

    match &info.kind {
        ChannelFunctionKind::Physical { addresses } => {
            for (address, value) in clamped.to_address_values(addresses) {
                multiverse.set_value(&address, value);
            }
        }
        ChannelFunctionKind::Virtual { relations } => {
            for relation in relations {
                match relation.kind {
                    RelationKind::Multiply => {
                        let Some(follower_info) =
                            cache.get(&relation.fixture_id, &relation.attribute)
                        else {
                            log::error!(
                                "Could not find cache for relation target '{}' of fixture '{}'",
                                relation.attribute,
                                relation.fixture_id
                            );
                            continue;
                        };

                        let master_factor = value.to_clamped_value(info.min, info.max).as_f32();
                        let follower_factor = attributes
                            .get(fixture_id, &relation.attribute)
                            .unwrap_or(follower_info.default)
                            .as_f32();

                        let new_value = AttributeValue::Clamped(ClampedValue::new(
                            master_factor * follower_factor,
                        ));

                        deferred_relations.push((
                            relation.fixture_id.clone(),
                            relation.attribute.clone(),
                            new_value,
                        ));
                    }
                    RelationKind::Override => {
                        deferred_relations.push((
                            relation.fixture_id.clone(),
                            relation.attribute.clone(),
                            *value,
                        ));
                    }
                }
            }
        }
    }
}
