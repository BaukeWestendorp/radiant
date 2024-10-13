use gpui::*;

use super::{
    super::NodeKind, error::GraphError, node_kind::ProcessingResult, view::control::Control,
    DataType, Graph, InputId, NodeId, OutputId, ProcessingContext, Value,
};

#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
    pub inputs: Vec<(String, InputId)>,
    pub outputs: Vec<(String, OutputId)>,
    pub position: Point<Pixels>,
}

impl Node {
    pub fn input_ids(&self) -> impl Iterator<Item = InputId> + '_ {
        self.inputs.iter().map(|(_name, id)| *id)
    }

    pub fn output_ids(&self) -> impl Iterator<Item = OutputId> + '_ {
        self.outputs.iter().map(|(_name, id)| *id)
    }

    pub fn input(&self, name: &str) -> Result<InputId, GraphError> {
        self.inputs
            .iter()
            .find(|(param_name, _id)| param_name == name)
            .map(|x| x.1)
            .ok_or_else(|| GraphError::NoSocketNamed(self.id, name.into()))
    }

    pub fn output(&self, name: &str) -> Result<OutputId, GraphError> {
        self.outputs
            .iter()
            .find(|(param_name, _id)| param_name == name)
            .map(|x| x.1)
            .ok_or_else(|| GraphError::NoSocketNamed(self.id, name.into()))
    }

    pub fn process(
        &self,
        context: &mut ProcessingContext,
        graph: &Graph,
    ) -> Result<ProcessingResult, GraphError> {
        self.kind.process(self.id, context, graph)
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pub id: InputId,
    pub data_type: DataType,
    pub node: NodeId,
}

#[derive(Debug, Clone)]
pub struct Output {
    pub id: OutputId,
    pub node: NodeId,
    pub data_type: DataType,
    pub value: OutputValue,
}

#[derive(Debug, Clone)]
pub enum OutputValue {
    Computed,
    Constant { value: Value, control: Control },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Socket {
    Input(InputId),
    Output(OutputId),
}
