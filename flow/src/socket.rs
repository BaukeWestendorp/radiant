use crate::{GraphDef, NodeId, Value};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnySocket {
    Input(InputSocket),
    Output(OutputSocket),
}

#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputSocket {
    pub node_id: NodeId,
    pub name: String,
}

impl InputSocket {
    pub fn new(node_id: NodeId, id: String) -> Self {
        Self { node_id, name: id }
    }
}

#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OutputSocket {
    pub node_id: NodeId,
    pub name: String,
}

impl OutputSocket {
    pub fn new(node_id: NodeId, id: String) -> Self {
        Self { node_id, name: id }
    }
}

#[derive(Debug, Clone)]
pub struct Input<D: GraphDef> {
    id: String,
    label: String,
    default: D::Value,
    control: D::Control,
}

impl<D: GraphDef> Input<D> {
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        default: D::Value,
        control: D::Control,
    ) -> Self {
        Self { id: id.into(), label: label.into(), default, control }
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

    pub fn control(&self) -> &D::Control {
        &self.control
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
