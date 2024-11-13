use crate::{Graph, GraphError, NodeId, OutputId};

use std::collections::HashMap;

pub trait GraphDefinition: Sized + Clone
where
    Self: 'static,
{
    #[cfg(not(feature = "serde"))]
    type NodeKind: NodeKind<Self> + Clone;
    #[cfg(feature = "serde")]
    type NodeKind: NodeKind<Self> + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>;

    #[cfg(not(feature = "serde"))]
    type NodeData: NodeData + Clone;
    #[cfg(feature = "serde")]
    type NodeData: Clone + serde::Serialize + for<'de> serde::Deserialize<'de>;

    #[cfg(not(feature = "serde"))]
    type Value: Value<Self> + Clone;
    #[cfg(feature = "serde")]
    type Value: Value<Self> + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>;

    #[cfg(not(feature = "serde"))]
    type DataType: DataType<Self> + Clone;
    #[cfg(feature = "serde")]
    type DataType: DataType<Self> + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>;

    type ProcessingContext;

    #[cfg(all(feature = "gpui"))]
    type NodeCategory: crate::gpui::NodeCategory;

    #[cfg(all(feature = "gpui", feature = "serde"))]
    type Control: crate::gpui::Control<Self>
        + Clone
        + serde::Serialize
        + for<'de> serde::Deserialize<'de>;
    #[cfg(all(feature = "gpui", not(feature = "serde")))]
    type Control: crate::gpui::Control<Self> + Clone;
}

pub trait NodeKind<Def: GraphDefinition> {
    fn build(&self, graph: &mut Graph<Def>, node_id: NodeId);

    fn process(
        &self,
        node_id: NodeId,
        context: &mut Def::ProcessingContext,
        graph: &Graph<Def>,
    ) -> Result<ProcessingResult<Def>, GraphError>;

    #[cfg(feature = "gpui")]
    fn name(&self) -> &str;

    #[cfg(feature = "gpui")]
    fn category(&self) -> Def::NodeCategory;

    #[cfg(feature = "gpui")]
    #[allow(opaque_hidden_inferred_bound)]
    fn all() -> impl Iterator<Item = Self>;
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

    pub fn get_output_value(&self, id: &OutputId) -> &Def::Value {
        self.values
            .get(id)
            .expect("output value should always be set after processing a node")
    }

    pub fn set_output_value(&mut self, id: OutputId, value: Def::Value) {
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

    #[cfg(feature = "gpui")]
    fn color(&self) -> gpui::Hsla;
}

pub trait NodeData: Default {
    #[cfg(feature = "gpui")]
    fn position(&self) -> &geo::Point;

    #[cfg(feature = "gpui")]
    fn set_position(&mut self, position: geo::Point);
}
