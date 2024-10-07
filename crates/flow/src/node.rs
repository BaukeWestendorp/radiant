use gpui::{Pixels, Point};

use crate::{FlowError, Graph, GraphProcessingCache, InputId, NodeId, NodeKind, OutputId};

#[derive(Debug, Clone)]
pub struct Node<D, V, N>
where
    N: NodeKind<DataType = D, Value = V>,
{
    pub id: NodeId,
    pub kind: N,
    pub inputs: Vec<(String, InputId)>,
    pub outputs: Vec<(String, OutputId)>,
    pub position: Point<Pixels>,
}

impl<D, V, N> Node<D, V, N>
where
    N: NodeKind<DataType = D, Value = V>,
{
    pub fn inputs<'a>(
        &'a self,
        graph: &'a Graph<D, V, N>,
    ) -> impl Iterator<Item = &Input<D, V>> + 'a {
        self.input_ids().map(|id| graph.input(id))
    }

    pub fn outputs<'a>(
        &'a self,
        graph: &'a Graph<D, V, N>,
    ) -> impl Iterator<Item = &Output<D, V>> + 'a {
        self.output_ids().map(|id| graph.output(id))
    }

    pub fn input_ids(&self) -> impl Iterator<Item = InputId> + '_ {
        self.inputs.iter().map(|(_name, id)| *id)
    }

    pub fn output_ids(&self) -> impl Iterator<Item = OutputId> + '_ {
        self.outputs.iter().map(|(_name, id)| *id)
    }

    pub fn input(&self, name: &str) -> Result<InputId, FlowError> {
        self.inputs
            .iter()
            .find(|(param_name, _id)| param_name == name)
            .map(|x| x.1)
            .ok_or_else(|| FlowError::NoSocketNamed(self.id, name.into()))
    }

    pub fn output(&self, name: &str) -> Result<OutputId, FlowError> {
        self.outputs
            .iter()
            .find(|(param_name, _id)| param_name == name)
            .map(|x| x.1)
            .ok_or_else(|| FlowError::NoSocketNamed(self.id, name.into()))
    }

    pub fn process(
        &self,
        context: &mut N::ProcessingContext,
        graph: &Graph<D, V, N>,
        cache: &mut GraphProcessingCache<V>,
    ) -> Result<(), FlowError> {
        self.kind.process(self.id, context, graph, cache)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Input<D, V> {
    pub id: InputId,
    pub data_type: D,
    pub value: V,
    pub node: NodeId,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Output<D, V> {
    pub id: OutputId,
    pub node: NodeId,
    pub data_type: D,
    pub value: OutputValue<V>,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum OutputValue<V> {
    Computed,
    Constant(V),
}
