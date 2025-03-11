use std::collections::HashMap;

use flow::{GraphDef, NodeId};
use gpui::{Pixels, Point};

#[derive(serde::Deserialize)]
struct GraphIntermediate<D: GraphDef + 'static> {
    graph: flow::Graph<D>,
    node_positions: HashMap<NodeId, Point<Pixels>>,
    offset: Point<Pixels>,
}

impl<'de, D: GraphDef + serde::Deserialize<'de> + 'static> serde::Deserialize<'de>
    for crate::Graph<D>
{
    fn deserialize<De>(deserializer: De) -> Result<Self, De::Error>
    where
        De: serde::Deserializer<'de>,
    {
        let GraphIntermediate { graph: flow_graph, node_positions, offset } =
            GraphIntermediate::<D>::deserialize(deserializer)?;

        let graph =
            crate::Graph { flow_graph, node_positions, dragged_node_position: None, offset };

        Ok(graph)
    }
}
