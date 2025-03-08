use crate::{Edge, Graph, GraphDef, Node, NodeId};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

impl<'de, D> Deserialize<'de> for Graph<D>
where
    D::Value: Deserialize<'de> + 'static,
    D::State: Deserialize<'de> + 'static,
    D: GraphDef + Deserialize<'de> + 'static,
{
    fn deserialize<De>(deserializer: De) -> Result<Self, De::Error>
    where
        De: Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct GraphIntermediate<D: GraphDef> {
            nodes: HashMap<NodeId, Node<D>>,
            edges: Vec<Edge>,
            state: D::State,
        }

        let intermediate = GraphIntermediate::<D>::deserialize(deserializer)?;

        let mut graph = Self::new();

        for (node_id, node) in intermediate.nodes {
            graph._add_node(node_id, node);
        }

        for edge in intermediate.edges {
            graph._add_edge(edge);
        }

        graph.state = intermediate.state;

        Ok(graph)
    }
}
