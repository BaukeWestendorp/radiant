use crate::{
    graph::{DataType, Value},
    node::NodeId,
};

pub type SocketId = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Edge {
    pub from: (NodeId, SocketId),
    pub to: (NodeId, SocketId),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Input {
    pub label: String,
    pub data_type: DataType,
    pub default: Value,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Output {
    pub label: String,
    pub data_type: DataType,
    pub default: Value,
}
