use std::collections::HashMap;

use crate::{Edge, Graph, Node, NodeId, ValueImpl};

impl<'de, State: Default + 'static, Value: ValueImpl + serde::Deserialize<'de> + 'static>
    serde::Deserialize<'de> for Graph<State, Value>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct GraphIntermediate<Value: ValueImpl> {
            nodes: HashMap<NodeId, Node<Value>>,
            edges: Vec<Edge>,
        }

        let intermediate = GraphIntermediate::<Value>::deserialize(deserializer)?;

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
