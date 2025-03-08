use crate::{GraphDef, NodeId};

#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Socket {
    pub node_id: NodeId,
    pub id: String,
}

impl Socket {
    pub fn new(node_id: NodeId, id: String) -> Self {
        Self { node_id, id }
    }
}

#[derive(Debug, Clone)]
pub struct Input<D: GraphDef> {
    id: String,
    label: String,
    default: D::Value,
}

impl<D: GraphDef> Input<D> {
    pub fn new(id: impl Into<String>, label: impl Into<String>, default: D::Value) -> Self {
        Self { id: id.into(), label: label.into(), default }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn default(&self) -> &D::Value {
        &self.default
    }
}

#[derive(Debug, Clone)]
pub struct Output {
    id: String,
    label: String,
}

impl Output {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self { id: id.into(), label: label.into() }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Edge {
    pub source: Socket,
    pub target: Socket,
}
