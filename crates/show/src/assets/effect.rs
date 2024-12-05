use crate::showfile;

use super::EffectGraphId;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Effect {
    Graph(EffectGraphId),
}

impl Effect {
    pub(crate) fn from_showfile(effect: showfile::Effect) -> Self {
        match effect {
            showfile::Effect::Graph(graph_id) => Effect::Graph(EffectGraphId(graph_id)),
        }
    }

    pub(crate) fn to_showfile(&self) -> showfile::Effect {
        match self {
            Effect::Graph(graph_id) => showfile::Effect::Graph(graph_id.0),
        }
    }
}
