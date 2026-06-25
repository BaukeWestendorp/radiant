use std::{collections::HashMap, sync::Arc};

use anyhow::Context;

use crate::{
    dmx::Address,
    gdtf::dmx::ChannelFunctionPath,
    mvr_gdtf::gdtf::{self, Gdtf, Name, attr::AttributeName, resource::ResourceKey},
    patch::{Fixture, FixtureDefinition, FixtureId, FixtureIdPart},
};

pub struct FixtureBuilder<'a> {
    root_id: FixtureId,
    name: String,

    gdtf: &'a Arc<Gdtf>,
    dmx_mode: &'a gdtf::dmx::DmxMode,
    dmx_address: Address,

    sibling_count_stack: Vec<u32>,
}

impl<'a> FixtureBuilder<'a> {
    pub fn new(
        definition: FixtureDefinition,
        gdtfs: &'a HashMap<ResourceKey, Arc<Gdtf>>,
    ) -> anyhow::Result<Self> {
        let root_id = FixtureId::new(definition.id);
        let name = definition.name;
        let dmx_address = definition.dmx_address;

        let gdtf_dmx_mode = definition.gdtf_dmx_mode;
        let fixture_name_for_errors = name.clone();

        let gdtf = gdtfs
            .get(&ResourceKey::new(definition.gdtf_file_name))
            .context("Could not find GDTF by resource key")?;

        let dmx_mode = gdtf
            .dmx_mode(&gdtf::Name::new(&gdtf_dmx_mode))
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Could not create fixture builder: No DMX mode '{}' found in GDTF for fixture '{}'. Skipping.",
                    gdtf_dmx_mode,
                    fixture_name_for_errors
                )
            })?;

        Ok(Self { root_id, name, gdtf, dmx_mode, dmx_address, sibling_count_stack: Vec::new() })
    }

    pub fn build(mut self) -> anyhow::Result<Vec<Fixture>> {
        let root_geometry = self.dmx_mode.geometry(&self.gdtf).ok_or_else(|| {
            anyhow::anyhow!("Root geometry not found for fixture '{}'. Skipping.", self.name)
        })?;
        let fixtures = self.fixtures_from_geometry(self.root_id, &root_geometry);
        let fixtures = normalize::normalize(fixtures);
        Ok(fixtures)
    }

    fn fixtures_from_geometry(
        &mut self,
        child_id: FixtureId,
        geometry: &gdtf::geo::Geometry,
    ) -> Vec<Fixture> {
        self.sibling_count_stack.push(0);

        let fixtures = match geometry {
            gdtf::geo::Geometry::GeometryReference(reference) => {
                self.fixture_from_reference_geometry(child_id, reference)
            }
            geo => self.fixture_from_geometry(child_id, geo),
        };

        self.sibling_count_stack.pop();

        fixtures
    }

    fn fixture_from_geometry(
        &mut self,
        child_id: FixtureId,
        geometry: &gdtf::geo::Geometry,
    ) -> Vec<Fixture> {
        let name =
            if child_id.len() == 1 { self.name.clone() } else { geometry.name().to_string() };

        self.create_child_fixture(child_id, name, geometry.name(), 0)
    }

    fn fixture_from_reference_geometry(
        &mut self,
        child_id: FixtureId,
        reference_geometry: &gdtf::geo::ReferenceGeometry,
    ) -> Vec<Fixture> {
        if reference_geometry.breaks().len() > 1 {
            log::warn!("Multiple breaks not yet supported");
        }

        let geometry_dmx_offset = match reference_geometry.breaks().first() {
            Some(offset) => match offset.absolute().checked_sub_signed(1) {
                Some(offset) => offset,
                None => {
                    log::warn!("Found a DMX break offset of 0, while the minimum is 1");
                    0
                }
            },
            None => 0,
        };

        let referenced_geometry_name = match reference_geometry.geometry(&self.gdtf).as_ref() {
            Some(n) => n.name(),
            None => return vec![],
        };

        self.create_child_fixture(
            child_id,
            reference_geometry.name().to_string(),
            referenced_geometry_name,
            geometry_dmx_offset,
        )
    }

    fn create_child_fixture(
        &mut self,
        id: FixtureId,
        name: String,
        referenced_geometry: &gdtf::Name,
        geometry_dmx_offset: u32,
    ) -> Vec<Fixture> {
        let referenced_geometry_name = referenced_geometry;

        let Some(referenced_geometry) = self.gdtf.geometry(referenced_geometry_name) else {
            log::error!(
                "Referenced geometry '{}' not found in fixture type '{}'",
                referenced_geometry_name,
                self.gdtf.name()
            );
            return vec![];
        };

        let child_fixtures = self.collect_child_fixtures(&id, referenced_geometry);
        let child_ids = self.collect_direct_child_ids(&id, &child_fixtures);
        let channel_functions = self.find_channel_functions(referenced_geometry.name());

        let mut fixtures = vec![Fixture {
            id,
            name,
            dmx_address: self.dmx_address,
            gdtf: Arc::clone(&self.gdtf),
            dmx_mode: self.dmx_mode.name().clone(),
            channel_functions,
            geometry_dmx_offset,
            child_ids,
        }];

        fixtures.extend(child_fixtures);
        fixtures
    }

    fn collect_child_fixtures(
        &mut self,
        id: &FixtureId,
        geometry: &gdtf::geo::Geometry,
    ) -> Vec<Fixture> {
        let mut child_fixtures = Vec::new();

        for child_geometry in geometry.children() {
            let sibling_count = *self.sibling_count_stack.last().unwrap();

            let part = match FixtureIdPart::new(sibling_count + 1) {
                Ok(part) => id.extended_with(part),
                Err(err) => {
                    log::error!("invalid FixtureIdPart: {}", err);
                    continue;
                }
            };

            let fixtures_for_child = self.fixtures_from_geometry(part, child_geometry);
            if fixtures_for_child.is_empty() {
                continue;
            }

            *self.sibling_count_stack.last_mut().unwrap() += 1;
            child_fixtures.extend(fixtures_for_child);
        }

        child_fixtures
    }

    fn collect_direct_child_ids(
        &self,
        id: &FixtureId,
        child_fixtures: &[Fixture],
    ) -> Vec<FixtureId> {
        child_fixtures
            .iter()
            .map(|f| f.id())
            .filter(|child_id| child_id.len() == id.len() + 1)
            .collect()
    }

    fn find_channel_functions(&self, referenced_geometry: &Name) -> Vec<ChannelFunctionPath> {
        let dmx_channels_with_geometry = self
            .dmx_mode
            .dmx_channels()
            .iter()
            .filter(|dmx_channel| dmx_channel.geometry_name() == referenced_geometry);

        let mut channel_functions = Vec::new();

        for dmx_channel in dmx_channels_with_geometry {
            for logical_channel in dmx_channel.logical_channels().iter() {
                for channel_function in logical_channel.channel_functions() {
                    if channel_function
                        .attribute(self.gdtf)
                        .is_some_and(|a| a.name() != &AttributeName::NoFeature)
                    {
                        let path = channel_function.path(dmx_channel, logical_channel);
                        channel_functions.push(path.clone());
                    }
                }
            }
        }

        channel_functions
    }
}

mod normalize {
    use std::collections::{BTreeMap, BTreeSet};

    use crate::patch::{Fixture, FixtureId, FixtureIdPart};

    pub fn normalize(fixtures: Vec<Fixture>) -> Vec<Fixture> {
        let mut map: BTreeMap<FixtureId, Fixture> =
            fixtures.into_iter().map(|f| (f.id(), f)).collect();

        prune_empty(&mut map);
        collapse(&mut map);
        renumber(&mut map);

        map.into_values().collect()
    }

    fn prune_empty(map: &mut BTreeMap<FixtureId, Fixture>) {
        let parent_map = build_parent_map(map);

        let mut keep: BTreeSet<FixtureId> = BTreeSet::new();

        for (id, fixture) in map.iter() {
            if id.is_root() {
                continue;
            }
            if fixture.channel_functions.is_empty() {
                continue;
            }
            let mut current = Some(id.clone());
            while let Some(cur) = current {
                if !keep.insert(cur.clone()) {
                    break;
                }
                current = parent_map.get(&cur).cloned();
            }
        }

        map.retain(|id, _| keep.contains(id));

        let surviving: BTreeSet<FixtureId> = map.keys().cloned().collect();
        for fixture in map.values_mut() {
            fixture.child_ids.retain(|cid| surviving.contains(cid));
        }
    }

    fn collapse(map: &mut BTreeMap<FixtureId, Fixture>) {
        loop {
            let parent_ids: Vec<FixtureId> = map.keys().cloned().collect();
            let mut changed = false;

            for parent_id in parent_ids {
                let children_with_cfs: Vec<FixtureId> = {
                    let Some(parent) = map.get(&parent_id) else { continue };
                    if parent.child_ids.is_empty() {
                        continue;
                    }

                    parent
                        .child_ids
                        .iter()
                        .filter(|cid| {
                            map.get(*cid).is_some_and(|c| !c.channel_functions.is_empty())
                        })
                        .cloned()
                        .collect()
                };

                if children_with_cfs.len() != 1 {
                    continue;
                }
                let child_id = children_with_cfs[0].clone();

                let (child_cfs, child_direct_children) = {
                    let Some(child) = map.get_mut(&child_id) else { continue };
                    let cfs = std::mem::take(&mut child.channel_functions);
                    let grandchildren = child.child_ids.clone();
                    (cfs, grandchildren)
                };

                if child_cfs.is_empty() {
                    continue;
                }

                if let Some(parent) = map.get_mut(&parent_id) {
                    parent.channel_functions.extend(child_cfs);

                    parent.child_ids.retain(|cid| cid != &child_id);
                    for grandchild_id in child_direct_children {
                        if !parent.child_ids.contains(&grandchild_id) {
                            parent.child_ids.push(grandchild_id);
                        }
                    }
                }

                map.remove(&child_id);

                changed = true;
            }

            if !changed {
                break;
            }
        }
    }

    fn renumber(map: &mut BTreeMap<FixtureId, Fixture>) {
        loop {
            let root_ids: Vec<FixtureId> = map.keys().filter(|id| id.is_root()).cloned().collect();

            let mut id_map: BTreeMap<FixtureId, FixtureId> = BTreeMap::new();
            let mut stack: Vec<FixtureId> = Vec::new();

            for root_id in root_ids {
                id_map.insert(root_id.clone(), root_id.clone());
                stack.push(root_id);
            }

            while let Some(old_parent_id) = stack.pop() {
                let Some(parent) = map.get(&old_parent_id) else { continue };

                let mut children: Vec<FixtureId> =
                    parent.child_ids.iter().filter(|cid| map.contains_key(*cid)).cloned().collect();
                children.sort();

                for (ix, old_child_id) in children.into_iter().enumerate() {
                    let Ok(new_part) = FixtureIdPart::new((ix as u32) + 1) else {
                        continue;
                    };
                    let mapped_parent = id_map
                        .get(&old_parent_id)
                        .cloned()
                        .unwrap_or_else(|| old_parent_id.clone());
                    let new_child_id = mapped_parent.extended_with(new_part);

                    id_map.insert(old_child_id.clone(), new_child_id);
                    stack.push(old_child_id);
                }
            }

            if id_map.iter().all(|(old, new)| old == new) {
                break;
            }

            let old_fixtures = std::mem::take(map);
            for (old_id, mut fixture) in old_fixtures {
                let new_id = id_map.get(&old_id).cloned().unwrap_or(old_id);
                fixture.id = new_id;

                for cid in fixture.child_ids.iter_mut() {
                    if let Some(mapped) = id_map.get(cid) {
                        *cid = mapped.clone();
                    }
                }

                map.insert(fixture.id(), fixture);
            }
        }
    }

    fn build_parent_map(map: &BTreeMap<FixtureId, Fixture>) -> BTreeMap<FixtureId, FixtureId> {
        let mut parent_map = BTreeMap::new();
        for (parent_id, fixture) in map.iter() {
            for child_id in &fixture.child_ids {
                parent_map.insert(child_id.clone(), parent_id.clone());
            }
        }
        parent_map
    }
}
