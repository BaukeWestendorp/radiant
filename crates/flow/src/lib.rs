use crate::graph_def::GraphDefinition;

pub mod error;
pub mod graph;
pub mod graph_def;
pub mod node;

pub use error::*;

slotmap::new_key_type! {
    pub struct NodeId;
    pub struct InputId;
    pub struct OutputId;
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Parameter {
    Input(InputId),
    Output(OutputId),
}

#[derive(Clone)]
pub struct Input<Def: GraphDefinition> {
    id: InputId,
    node_id: NodeId,
    data_type: Def::DataType,
    pub kind: InputParameterKind<Def>,
}

impl<Def: GraphDefinition> Input<Def> {
    pub fn id(&self) -> InputId {
        self.id
    }

    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn data_type(&self) -> &Def::DataType {
        &self.data_type
    }
}

#[derive(Clone)]
pub enum InputParameterKind<Def: GraphDefinition> {
    EdgeOrConstant {
        value: Def::Value,
        control: Def::Control,
    },
}

#[derive(Clone)]
pub struct Output<Def: GraphDefinition> {
    id: OutputId,
    node_id: NodeId,
    data_type: Def::DataType,
    pub kind: OutputParameterKind<Def>,
}

impl<Def: GraphDefinition> Output<Def> {
    pub fn id(&self) -> OutputId {
        self.id
    }

    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn data_type(&self) -> &Def::DataType {
        &self.data_type
    }
}

#[derive(Clone)]
pub enum OutputParameterKind<Def: GraphDefinition> {
    Computed,
    Constant {
        value: Def::Value,
        control: Def::Control,
    },
}
