pub mod effect;

pub use effect::*;

use super::patch::FixtureId;

use graph::{EffectGraph, EffectGraphId};

use std::collections::HashMap;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Assets {
    #[serde(default)]
    groups: HashMap<GroupId, Group>,

    #[serde(default)]
    effects: HashMap<EffectId, Effect>,

    #[serde(default)]
    effect_graphs: HashMap<EffectGraphId, EffectGraph>,
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

    pub fn new_effect(&mut self, id: EffectId, group: GroupId, kind: EffectKind) -> EffectId {
        let effect = Effect::new(id, group, kind);
        self.effects.insert(id, effect);
        id
    }

    pub fn effects(&self) -> impl Iterator<Item = &Effect> {
        self.effects.values()
    }

    pub fn effect(&self, id: &EffectId) -> Option<&Effect> {
        self.effects.get(id)
    }

    pub fn effect_mut(&mut self, id: &EffectId) -> Option<&mut Effect> {
        self.effects.get_mut(id)
    }

    pub fn effect_graph(&self, id: &EffectGraphId) -> Option<&EffectGraph> {
        self.effect_graphs.get(id)
    }

    pub fn effect_graph_mut(&mut self, id: &EffectGraphId) -> Option<&mut EffectGraph> {
        self.effect_graphs.get_mut(id)
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
