use crate::{Edge, Graph, GraphDef, Node, NodeId};
use serde::{Deserialize, Deserializer};
use std::{collections::HashMap, sync::atomic::Ordering};

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

        let mut max_id = 0;
        for (node_id, node) in intermediate.nodes {
            graph._add_node(node_id, node);
            if node_id.0 > max_id {
                max_id = node_id.0;
            }
        }
        graph.node_id_counter.store(max_id + 1, Ordering::Relaxed);

        for edge in intermediate.edges {
            graph._add_edge(edge);
        }

        graph.state = intermediate.state;

        Ok(graph)
    }
}
