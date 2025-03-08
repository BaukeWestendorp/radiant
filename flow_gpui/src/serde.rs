use std::collections::HashMap;

use flow::{GraphDef, NodeId};

#[derive(serde::Deserialize)]
struct GraphIntermediate<D: GraphDef + 'static> {
    graph: flow::Graph<D>,
    node_positions: HashMap<NodeId, (f32, f32)>,
}

impl<'de, D: GraphDef + serde::Deserialize<'de> + 'static> serde::Deserialize<'de>
    for crate::Graph<D>
{
    fn deserialize<De>(deserializer: De) -> Result<Self, De::Error>
    where
        De: serde::Deserializer<'de>,
    {
        let GraphIntermediate { graph, node_positions } =
            GraphIntermediate::<D>::deserialize(deserializer)?;

        let graph = crate::Graph { graph, node_positions };

        Ok(graph)
    }
}
