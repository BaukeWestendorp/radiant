use gpui::Hsla;

use crate::{FlowError, Graph, GraphProcessingCache, NodeId};

pub trait NodeKind: Clone {
    type DataType;
    type Value;
    type ProcessingContext;

    fn label(&self) -> &str;

    fn build(&self, graph: &mut Graph<Self::DataType, Self::Value, Self>, node_id: NodeId)
    where
        Self: Sized;

    fn process(
        &self,
        node_id: NodeId,
        context: &mut Self::ProcessingContext,
        graph: &Graph<Self::DataType, Self::Value, Self>,
        cache: &mut GraphProcessingCache<Self::Value>,
    ) -> Result<(), FlowError>
    where
        Self: Sized;
}

pub trait Value {
    type DataType;

    fn try_cast_to(self, target_type: &Self::DataType) -> Result<Self, FlowError>
    where
        Self: Sized;
}

pub trait DataType: From<Self::Value> {
    type Value: Value;

    fn color(&self) -> Hsla;
}
