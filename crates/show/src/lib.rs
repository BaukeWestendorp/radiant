use dmx::DmxUniverseId;
use effect::{Effect, EffectGraph, EffectId};

use crate::fixture::FixtureId;
use crate::patch::Patch;
use std::collections::HashMap;
use std::fs;
use std::net::Ipv4Addr;
use std::path::Path;

pub mod attr_def;
pub mod effect;
pub mod fixture;
pub mod patch;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Show {
    #[serde(default)]
    patch: Patch,
    #[serde(default)]
    assets: Assets,
    #[serde(default)]
    dmx_protocols: DmxProtocols,
}

impl Show {
    pub fn patch(&self) -> &Patch {
        &self.patch
    }

    pub fn patch_mut(&mut self) -> &mut Patch {
        &mut self.patch
    }

    pub fn assets(&self) -> &Assets {
        &self.assets
    }

    pub fn assets_mut(&mut self) -> &mut Assets {
        &mut self.assets
    }

    pub fn dmx_protocols(&self) -> &DmxProtocols {
        &self.dmx_protocols
    }

    pub fn read_from_file(path: &Path) -> anyhow::Result<Self> {
        let show_json = fs::read_to_string(path)?;
        let show: Self = serde_json::from_str(&show_json)?;
        Ok(show)
    }

    pub fn save_to_file(&self, path: &Path) -> anyhow::Result<()> {
        let show_json = serde_json::to_string_pretty(self)?;
        fs::write(path, show_json)?;

        Ok(())
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Assets {
    #[serde(default)]
    groups: HashMap<GroupId, Group>,

    #[serde(default)]
    effects: HashMap<EffectId, Effect>,
}

impl Assets {
    pub fn new_group(&mut self, id: GroupId, fixtures: Vec<FixtureId>) -> GroupId {
        let group = Group::new(id, fixtures);
        self.groups.insert(id, group);
        id
    }

    pub fn groups(&self) -> impl Iterator<Item = &Group> {
        self.groups.values()
    }

    pub fn group(&self, id: &GroupId) -> Option<&Group> {
        self.groups.get(id)
    }

    pub fn group_mut(&mut self, id: &GroupId) -> Option<&mut Group> {
        self.groups.get_mut(id)
    }

    pub fn new_effect(&mut self, id: EffectId, group: GroupId, graph: EffectGraph) -> EffectId {
        let effect = Effect::new(id, group, graph);
        self.effects.insert(id, effect);
        id
    }

    pub fn effect(&self, id: &EffectId) -> Option<&Effect> {
        self.effects.get(id)
    }

    pub fn effect_mut(&mut self, id: &EffectId) -> Option<&mut Effect> {
        self.effects.get_mut(id)
    }
}

pub type GroupId = u32;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Group {
    id: GroupId,
    fixtures: Vec<FixtureId>,
}

impl Group {
    pub(crate) fn new(id: GroupId, fixtures: Vec<FixtureId>) -> Self {
        Self { id, fixtures }
    }

    pub fn fixtures(&self) -> &[FixtureId] {
        &self.fixtures
    }

    pub fn push_fixture(&mut self, fixture: FixtureId) {
        self.fixtures.push(fixture);
    }

    pub fn len(&self) -> usize {
        self.fixtures.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fixtures.is_empty()
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct DmxProtocols {
    artnet: Vec<ArtnetNodeSettings>,
}

impl DmxProtocols {
    pub fn artnet(&self) -> &[ArtnetNodeSettings] {
        &self.artnet
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArtnetNodeSettings {
    pub destination_ip: Ipv4Addr,
    pub universe: DmxUniverseId,
}
