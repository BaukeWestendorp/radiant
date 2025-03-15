use crate::{GraphDef, NodeId, Value};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnySocket {
    Input(Socket),
    Output(Socket),
}

impl AnySocket {
    pub fn socket(&self) -> &Socket {
        match self {
            Self::Input(socket) | Self::Output(socket) => socket,
        }
    }
}

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

    pub fn data_type(&self) -> D::DataType {
        self.default().data_type()
    }
}

#[derive(Debug, Clone)]
pub struct Output<D: GraphDef> {
    id: String,
    label: String,
    data_type: D::DataType,
}

impl<D: GraphDef> Output<D> {
    pub fn new(id: impl Into<String>, label: impl Into<String>, data_type: D::DataType) -> Self {
        Self { id: id.into(), label: label.into(), data_type }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn data_type(&self) -> &D::DataType {
        &self.data_type
    }
}

#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Edge {
    pub source: Socket,
    pub target: Socket,
}
