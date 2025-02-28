use std::collections::HashMap;

use crate::{
    graph::Graph,
    node::{Node, NodeId},
    socket::Edge,
    template::{NodeTemplate, TemplateId},
};

impl<'de> serde::Deserialize<'de> for Graph {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Intermediate {
            templates: HashMap<TemplateId, NodeTemplate>,
            nodes: HashMap<NodeId, Node>,
            edges: Vec<Edge>,
        }

        let intermediate = Intermediate::deserialize(deserializer)?;

        let mut graph = Graph::default();

        for (id, template) in intermediate.templates {
            graph.add_template(id, template);
        }

        for (id, node) in intermediate.nodes {
            graph.add_node(id, node);
        }

        for edge in intermediate.edges {
            graph.add_edge(edge);
        }

        Ok(graph)
    }
}
