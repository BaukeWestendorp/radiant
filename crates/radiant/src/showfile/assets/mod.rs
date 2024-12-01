pub mod effect;
pub mod group;

pub use effect::*;
pub use group::*;

use flow_graph::FlowEffectGraph;
use gpui::SharedString;

use super::patch::FixtureId;

use std::collections::HashMap;

#[derive(
    Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct AnyAssetId(pub u32);

macro_rules! asset_id {
    ($vis:vis $name:ident) => {
        #[derive(
            Clone,
            Copy,
            Default,
            PartialEq,
            Eq,
            Ord,
            PartialOrd,
            Hash,
            serde::Serialize,
            serde::Deserialize,
        )]
        $vis struct $name(pub u32);

        impl From<$name> for crate::showfile::AnyAssetId {
            fn from(id: $name) -> Self {
                crate::showfile::AnyAssetId(id.0)
            }
        }

        impl From<crate::showfile::AnyAssetId> for $name {
            fn from(id: crate::showfile::AnyAssetId) -> Self {
                Self(id.0)
            }
        }
    };
}

pub(crate) use asset_id;

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
    pub fn new_group(
        &mut self,
        id: GroupId,
        label: SharedString,
        fixtures: Vec<FixtureId>,
    ) -> GroupId {
        let group = Group::new(id, label, fixtures);
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

    pub fn new_effect(
        &mut self,
        id: EffectId,
        label: SharedString,
        group: GroupId,
        kind: EffectKind,
    ) -> EffectId {
        let effect = Effect::new(id, label, group, kind);
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

    pub fn new_effect_graph(
        &mut self,
        id: EffectGraphId,
        label: SharedString,
        graph: FlowEffectGraph,
    ) -> EffectGraphId {
        let effect_graph = EffectGraph::new(label, graph);
        self.effect_graphs.insert(id, effect_graph);
        id
    }

    pub fn effect_graphs(&self) -> impl Iterator<Item = &EffectGraph> {
        self.effect_graphs.values()
    }

    pub fn effect_graph(&self, id: &EffectGraphId) -> Option<&EffectGraph> {
        self.effect_graphs.get(id)
    }

    pub fn effect_graph_mut(&mut self, id: &EffectGraphId) -> Option<&mut EffectGraph> {
        self.effect_graphs.get_mut(id)
    }
}
