use crate::{Graph, GraphDef, InputSocket, Node, NodeId, OutputSocket};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Edge {
    target: InputSocket,
    source: OutputSocket,
}

#[derive(Serialize, Deserialize)]
struct GraphIntermediate<D: GraphDef> {
    nodes: HashMap<NodeId, Node<D>>,
    edges: Vec<Edge>,
    node_positions: HashMap<NodeId, gpui::Point<gpui::Pixels>>,
    offset: gpui::Point<gpui::Pixels>,
}

impl<'de, D> Deserialize<'de> for Graph<D>
where
    D::Value: Deserialize<'de> + 'static,
    D: GraphDef + Deserialize<'de> + 'static,
{
    fn deserialize<De: Deserializer<'de>>(deserializer: De) -> Result<Self, De::Error> {
        let GraphIntermediate { nodes, edges, node_positions, offset } =
            GraphIntermediate::<D>::deserialize(deserializer)?;

        let mut graph = Self::new();

        // Nodes
        let mut max_id = 0;
        nodes.into_iter().for_each(|(node_id, node)| {
            // Node position
            let position = node_positions.get(&node_id).cloned().unwrap_or_default();

            graph.add_node_internal(node_id, node, position);
            if node_id.0 > max_id {
                max_id = node_id.0;
            }
        });
        graph.node_id_counter = max_id + 1;

        // Edges
        edges.into_iter().for_each(|edge| graph.add_edge_internal(edge.target, edge.source));

        // Offset
        graph.set_offset(offset);

        Ok(graph)
    }
}

impl<D> Serialize for Graph<D>
where
    D::Value: Serialize + 'static,
    D: GraphDef + Serialize + Clone + 'static,
{
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let edges = self
            .edges()
            .map(|(target, source)| Edge { target: target.clone(), source: source.clone() })
            .collect();

        GraphIntermediate::<D> {
            nodes: self.nodes.clone(),
            edges,
            node_positions: self.node_positions.clone(),
            offset: self.offset,
        }
        .serialize(serializer)
    }
}
