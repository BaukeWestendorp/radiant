use crate::error::GraphError;
use crate::graph::Graph;
use crate::NodeId;

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
    ) -> Result<ProcessingResult, GraphError>;

    fn label(&self) -> &'static str;

    // FIXME: I don't know what this warning is about...
    #[allow(opaque_hidden_inferred_bound)]
    fn all() -> impl Iterator<Item = Self>;
}

#[derive(Debug, Clone)]
pub struct ProcessingResult {}

pub trait Value<Def: GraphDefinition> {
    fn try_cast_to(&self, target: &Def::DataType) -> Result<Self, GraphError>
    where
        Self: Sized;
}

pub trait DataType<Def: GraphDefinition> {
    fn default_value(&self) -> Def::Value;
}

pub trait Control<Def: GraphDefinition> {}
