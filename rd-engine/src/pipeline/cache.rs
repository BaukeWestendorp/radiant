use std::{collections::HashMap, str::FromStr as _};

use anyhow::Context;

use crate::{
    dmx,
    mvr_gdtf::gdtf::{
        NodePath,
        attr::{AttributeName, PhysicalUnit},
        dmx::{ChannelFunction, DmxChannel, DmxOffset, LogicalChannel, RelationKind},
    },
    patch::{Fixture, FixtureId, Patch},
    value::{AttributeValue, AttributeValues, ClampedValue},
};

#[derive(Debug, Default)]
pub struct PipelineCache {
    channel_functions: HashMap<FixtureId, HashMap<AttributeName, ChannelFunctionInfo>>,
    initial_defaults: AttributeValues,
    highlights: HashMap<FixtureId, HashMap<dmx::Address, dmx::Value>>,
}

impl PipelineCache {
    pub fn new(patch: &Patch) -> Self {
        let mut this = Self {
            channel_functions: HashMap::new(),
            initial_defaults: AttributeValues::new(),
            highlights: HashMap::new(),
        };
        this.regenerate(patch);
        this
    }

    pub fn initial_defaults(&self) -> &AttributeValues {
        &self.initial_defaults
    }

    pub fn highlights(&self) -> &HashMap<FixtureId, HashMap<dmx::Address, dmx::Value>> {
        &self.highlights
    }

    pub fn get(
        &self,
        fixture_id: &FixtureId,
        attribute: &AttributeName,
    ) -> Option<&ChannelFunctionInfo> {
        self.channel_functions.get(fixture_id)?.get(attribute)
    }

    fn get_mut(
        &mut self,
        fixture_id: &FixtureId,
        attribute: &AttributeName,
    ) -> Option<&mut ChannelFunctionInfo> {
        self.channel_functions.get_mut(fixture_id)?.get_mut(attribute)
    }

    fn regenerate(&mut self, patch: &Patch) {
        let mut deferred = Vec::new();

        for fixture in patch.fixtures() {
            for cf_path in fixture.channel_functions() {
                if let Err(err) = self.generate_channel_function(cf_path, fixture, &mut deferred) {
                    log::error!("Failed to cache channel function info: {err}");
                };
            }
        }

        if let Err(err) = self.resolve_deferred(deferred) {
            log::error!("Failed to cache deferred (virtual) channels: {err}");
        };
    }

    fn generate_channel_function<'a>(
        &mut self,
        cf_path: &NodePath,
        fixture: &'a Fixture,
        deferred: &mut Vec<(&'a Fixture, NodePath)>,
    ) -> anyhow::Result<()> {
        let (dc, lc, cf) = resolve_channel_function_path(fixture, cf_path)?;
        let attr = cf.attribute(fixture.gdtf()).context("Could not find attribute")?;

        let is_unitless = matches!(attr.physical_unit(), PhysicalUnit::None);
        let default_clamped = ClampedValue::from(cf.default());
        let (min, max, default) = if is_unitless {
            (
                AttributeValue::Clamped(ClampedValue::new(0.0)),
                AttributeValue::Clamped(ClampedValue::new(1.0)),
                AttributeValue::Clamped(default_clamped),
            )
        } else {
            let from = cf.physical_from();
            let to = cf.physical_to();
            let default = from + (to - from) * default_clamped.as_f32();

            (
                AttributeValue::Physical(from),
                AttributeValue::Physical(to),
                AttributeValue::Physical(default),
            )
        };

        let kind = match dc.offset() {
            DmxOffset::Physical(offsets) => {
                let mut addresses = Vec::with_capacity(4);
                for offset in offsets {
                    let channel_offset = *offset as i32 - 1;
                    let geometry_offset = fixture.geometry_dmx_offset() as i32;
                    let total_offset = channel_offset + geometry_offset;
                    let address = fixture
                        .dmx_address()
                        .with_channel_offset(total_offset)
                        .expect("Offset should be >= 0");
                    addresses.push(address);
                }
                ChannelFunctionKind::Physical { addresses }
            }
            DmxOffset::Virtual => {
                deferred.push((fixture, cf_path.clone()));
                ChannelFunctionKind::Virtual { relations: Vec::new() }
            }
        };

        if let Some((initial_lc, initial_cf)) = dc.initial_function() {
            if initial_cf.node_path(dc, initial_lc) == cf.node_path(dc, lc) {
                let initial_default_clamped = ClampedValue::from(initial_cf.default());
                if is_unitless {
                    self.initial_defaults.set(
                        fixture.id(),
                        attr.name().clone(),
                        AttributeValue::Clamped(initial_default_clamped),
                    );
                } else {
                    let from = cf.physical_from();
                    let to = cf.physical_to();
                    let initial_default = from + (to - from) * default_clamped.as_f32();
                    self.initial_defaults.set(
                        fixture.id(),
                        attr.name().clone(),
                        AttributeValue::Physical(initial_default),
                    );
                }

                if let ChannelFunctionKind::Physical { addresses } = &kind {
                    if let Some(highlight) = dc.highlight() {
                        let values = ClampedValue::from(highlight).to_address_values(addresses);
                        for (address, value) in values {
                            self.highlights.entry(fixture.id()).or_default().insert(address, value);
                        }
                    }
                }
            }
        }

        self.channel_functions
            .entry(fixture.id())
            .or_default()
            .insert(attr.name().clone(), ChannelFunctionInfo { default, min, max, kind });

        Ok(())
    }

    fn resolve_deferred(&mut self, deferred: Vec<(&Fixture, NodePath)>) -> anyhow::Result<()> {
        for (fixture, cf_path) in deferred {
            let (dc, _lc, cf) = resolve_channel_function_path(fixture, &cf_path)?;
            let attr = cf.attribute(fixture.gdtf()).context("Could not find attribute")?;

            let relations = relations_for_dmx_channel(fixture, dc);

            let Some(info) = self.get_mut(&fixture.id(), attr.name()) else {
                continue;
            };

            info.kind = ChannelFunctionKind::Virtual { relations };
        }

        Ok(())
    }
}

fn resolve_channel_function_path<'a>(
    fixture: &'a Fixture,
    path: &NodePath,
) -> anyhow::Result<(&'a DmxChannel, &'a LogicalChannel, &'a ChannelFunction)> {
    let mut parts = path.parts().iter();
    let dc_name = parts.next().context("NodePath did not contain DMX channel name")?;
    let lc_name = parts.next().context("NodePath did not contain logical channel name")?;
    let lc_attr = AttributeName::from_str(lc_name.as_str()).unwrap();
    let cf_name = parts.next().context("NodePath did not contain channel function name")?;
    let dc = fixture.dmx_mode().dmx_channel(dc_name).context("Could not find DMX channel")?;
    let lc = dc.logical_channel(&lc_attr).context("Could not find logical channel")?;
    let cf = lc.channel_function(cf_name).context("Could not find channel function")?;
    Ok((dc, lc, cf))
}

fn relations_for_dmx_channel(
    fixture: &Fixture,
    dmx_channel: &DmxChannel,
) -> Vec<ChannelFunctionRelation> {
    let mut channel_relations = Vec::new();

    let relations = fixture.dmx_mode().relations().iter().filter(|relation| {
        relation
            .master(fixture.dmx_mode())
            .is_some_and(|master| master.name() == dmx_channel.name())
    });

    for relation in relations {
        let Some((_, _, follower_channel_function)) = relation.follower(fixture.dmx_mode()) else {
            log::warn!("Could not find follower for relation with master '{}'", dmx_channel.name());
            continue;
        };

        let Some(attribute) = follower_channel_function.attribute(fixture.gdtf()) else {
            continue;
        };

        channel_relations.push(ChannelFunctionRelation {
            fixture_id: fixture.id(),
            attribute: attribute.name().clone(),
            kind: relation.kind(),
        });
    }

    channel_relations
}

#[derive(Debug)]
pub struct ChannelFunctionInfo {
    pub default: AttributeValue,
    pub min: AttributeValue,
    pub max: AttributeValue,
    pub kind: ChannelFunctionKind,
}

#[derive(Debug)]
pub enum ChannelFunctionKind {
    Physical { addresses: Vec<dmx::Address> },
    Virtual { relations: Vec<ChannelFunctionRelation> },
}

#[derive(Debug)]
pub struct ChannelFunctionRelation {
    pub fixture_id: FixtureId,
    pub attribute: AttributeName,
    pub kind: RelationKind,
}
