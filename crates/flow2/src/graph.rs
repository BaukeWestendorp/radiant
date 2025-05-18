use std::collections::HashMap;

use crate::def::Node;

pub struct NodeGraph {
    pub nodes: HashMap<NodeId, Box<dyn Node>>,
    pub edges: Vec<Edge>,
}

impl NodeGraph {
    pub fn new() -> Self {
        Self { nodes: HashMap::new(), edges: vec![] }
    }

    pub fn add_node<N: Node + 'static>(&mut self, node: N) {
        let next_id = self.nodes.keys().max().copied().unwrap_or_default();
        self.nodes.insert(next_id, Box::new(node));
    }

    pub fn add_edge(&mut self, edge: Edge) -> anyhow::Result<()> {
        self.edges.push(edge);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge {
    input: InputId,
    output: OutputId,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NodeId(pub u64);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct InputId(pub u64);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct OutputId(pub u64);
