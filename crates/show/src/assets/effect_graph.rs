super::asset_id!(pub EffectGraphId);

#[derive(Debug, Clone, PartialEq)]
pub struct EffectGraph {
    pub id: EffectGraphId,
    pub label: String,
    pub graph: (),
}

impl super::Asset for EffectGraph {
    type Id = EffectGraphId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl From<showfile::EffectGraph> for EffectGraph {
    fn from(graph: showfile::EffectGraph) -> Self {
        Self {
            id: EffectGraphId(graph.id),
            label: graph.label,
            graph: (),
        }
    }
}
