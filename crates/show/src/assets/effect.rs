use crate::showfile;

use super::{EffectGraphId, GroupId};

super::asset_id!(pub EffectId);

#[derive(Debug, Clone, PartialEq)]
pub struct Effect {
    pub id: EffectId,
    pub label: String,
    pub group: GroupId,
    pub kind: EffectKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EffectKind {
    Graph(EffectGraphId),
}

impl super::Asset for Effect {
    type Id = EffectId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl Effect {
    pub(crate) fn from_showfile(effect: showfile::Effect) -> Self {
        Self {
            id: EffectId(effect.id),
            label: effect.label,
            group: GroupId(effect.group),
            kind: match effect.kind {
                showfile::EffectKind::Graph(graph_id) => EffectKind::Graph(EffectGraphId(graph_id)),
            },
        }
    }

    pub(crate) fn to_showfile(&self) -> showfile::Effect {
        showfile::Effect {
            id: self.id.0,
            label: self.label.clone(),
            group: self.group.0,
            kind: match self.kind {
                EffectKind::Graph(graph_id) => showfile::EffectKind::Graph(graph_id.0),
            },
        }
    }
}
