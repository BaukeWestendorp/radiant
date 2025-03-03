use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

use crate::{Edge, Graph, GraphDef, Node, NodeId};

impl<'de, D: GraphDef + Deserialize<'de> + 'static> Deserialize<'de> for Graph<D>
where
    D::Value: Deserialize<'de> + 'static,
{
    fn deserialize<De>(deserializer: De) -> Result<Self, De::Error>
    where
        De: Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct GraphIntermediate<D: GraphDef> {
            nodes: HashMap<NodeId, Node<D>>,
            edges: Vec<Edge>,
        }

        let intermediate = GraphIntermediate::<D>::deserialize(deserializer)?;

        let mut graph = Self::new();

        for (node_id, node) in intermediate.nodes {
            graph._add_node(node_id, node);
        }

        for edge in intermediate.edges {
            graph._add_edge(edge);
        }

        Ok(graph)
    }
}
