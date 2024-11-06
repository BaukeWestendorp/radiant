use crate::error::GraphError;
use crate::graph::Graph;
use crate::{NodeId, OutputId};
use std::collections::HashMap;

pub trait GraphDefinition: Sized + Clone {
    type NodeKind: NodeKind<Self> + Clone;
    type NodeData: Clone;
    type Value: Value<Self> + Clone;
    type DataType: DataType<Self> + Clone;
    type Control: Control<Self> + Clone;
}

pub trait NodeKind<Def: GraphDefinition> {
    type ProcessingContext;

    fn build(&self, graph: &mut Graph<Def>, node_id: NodeId);

    fn process(
        &self,
        node_id: NodeId,
        context: &mut Self::ProcessingContext,
        graph: &Graph<Def>,
    ) -> Result<ProcessingResult<Def>, GraphError>;
}

#[derive(Debug, Clone, Default)]
pub struct ProcessingResult<Def: GraphDefinition> {
    values: HashMap<OutputId, Def::Value>,
}

impl<Def: GraphDefinition> ProcessingResult<Def> {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get_value(&self, id: &OutputId) -> &Def::Value {
        self.values
            .get(id)
            .expect("output value should always be set after processing a node")
    }

    pub fn set_value(&mut self, id: OutputId, value: Def::Value) {
        self.values.insert(id, value);
    }
}

pub trait Value<Def: GraphDefinition> {
    fn try_cast_to(&self, target: &Def::DataType) -> Result<Self, GraphError>
    where
        Self: Sized;
}

pub trait DataType<Def: GraphDefinition> {
    fn default_value(&self) -> Def::Value;
}

pub trait Control<Def: GraphDefinition> {}
