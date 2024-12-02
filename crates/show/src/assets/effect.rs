use super::{EffectGraphId, GroupId};

super::asset_id!(pub EffectId);

pub struct Effect {
    pub id: EffectId,
    pub label: String,
    pub group: GroupId,
    pub kind: EffectKind,
}

pub enum EffectKind {
    Graph(EffectGraphId),
}

impl super::Asset for Effect {
    type Id = EffectId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl From<showfile::Effect> for Effect {
    fn from(effect: showfile::Effect) -> Self {
        Self {
            id: EffectId(effect.id),
            label: effect.label,
            group: GroupId(effect.group),
            kind: match effect.kind {
                showfile::EffectKind::Graph(graph_id) => EffectKind::Graph(EffectGraphId(graph_id)),
            },
        }
    }
}
