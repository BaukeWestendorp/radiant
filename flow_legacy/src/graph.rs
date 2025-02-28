use std::collections::HashMap;

use crate::{
    node::{Node, NodeId},
    socket::{Edge, SocketId},
    template::{NodeTemplate, Processor, TemplateId},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum DataType {
    Number,
}

pub type Value = u32;

#[derive(Debug, Default)]
pub struct GraphContext {
    pub output_value: Value,
}

pub struct NodeContext<'gcx> {
    graph_cx: &'gcx mut GraphContext,
    input_values: HashMap<SocketId, Value>,
    output_values: HashMap<SocketId, Value>,
}

impl<'gcx> NodeContext<'gcx> {
    pub fn new(
        graph_cx: &'gcx mut GraphContext,
        input_values: HashMap<SocketId, Value>,
    ) -> NodeContext<'gcx> {
        NodeContext { graph_cx, input_values, output_values: HashMap::new() }
    }

    pub fn input_value(&self, id: &str) -> Value {
        *self.input_values.get(id).expect("should always find the input")
    }

    pub fn set_output_value(&mut self, id: SocketId, value: Value) {
        self.output_values.insert(id, value);
    }

    pub fn output_value(&self, id: &SocketId) -> Value {
        *self.output_values.get(id).expect("should always find the output")
    }
}

impl std::ops::Deref for NodeContext<'_> {
    type Target = GraphContext;

    fn deref(&self) -> &Self::Target {
        &self.graph_cx
    }
}

impl std::ops::DerefMut for NodeContext<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.graph_cx
    }
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GraphState {
    nodes: HashMap<NodeId, Node>,
    edges: Vec<Edge>,
}

#[derive(Debug, Default)]
pub struct Graph {
    templates: HashMap<TemplateId, NodeTemplate>,

    state: GraphState,

    /// Leaf nodes are nodes that have no outgoing edges
    leaf_nodes: Vec<NodeId>,
}

impl Graph {
    pub fn new(templates: HashMap<TemplateId, NodeTemplate>, data: GraphState) -> Self {
        let mut this = Graph { templates, state: data, leaf_nodes: Vec::new() };
        this.recalculate_leaf_nodes();
        this
    }

    pub fn add_template(&mut self, template_id: TemplateId, template: NodeTemplate) {
        self.templates.insert(template_id, template);
    }

    pub fn get_template(&self, template_id: &str) -> &NodeTemplate {
        self.templates.get(template_id).expect("should always find the template")
    }

    pub fn register_processor(&mut self, template_id: &str, processor: Box<Processor>) {
        let template =
            self.templates.get_mut(template_id).expect("should always find the template");
        template.set_processor(Some(processor));
    }

    pub fn add_node(&mut self, node_id: NodeId, node: Node) {
        self.state.nodes.insert(node_id, node);
        self.leaf_nodes.push(node_id);
    }

    pub fn get_node(&self, node_id: &NodeId) -> &Node {
        self.state.nodes.get(node_id).expect("should always find the node")
    }

    pub fn remove_node(&mut self, node_id: NodeId) {
        // Remove all edges that are connected to this node
        self.state.edges.retain(|Edge { from, to }| from.0 != node_id && to.0 != node_id);

        self.remove_leaf_node(&node_id);

        self.state.nodes.remove(&node_id);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        if !self.check_edge_validity(&edge) {
            return;
        }

        self.remove_leaf_node(&edge.from.0);

        self.state.edges.push(edge);
    }

    pub fn remove_edge(&mut self, edge: &Edge) {
        self.state.edges.retain(|e| e != edge);
    }

    fn check_edge_validity(&self, _edge: &Edge) -> bool {
        // FIXME: Implement
        true
    }

    fn remove_leaf_node(&mut self, node_id: &NodeId) {
        self.leaf_nodes.retain(|&id| id != *node_id);
    }

    fn recalculate_leaf_nodes(&mut self) {
        self.leaf_nodes.clear();
        for node_id in self.state.nodes.keys() {
            if self.state.edges.iter().all(|edge| edge.from.0 != *node_id) {
                self.leaf_nodes.push(*node_id);
            }
        }
    }

    pub fn process(&self) -> GraphContext {
        let mut cx = GraphContext::default();
        for leaf_node in &self.leaf_nodes {
            self.process_node(*leaf_node, &mut cx);
        }
        cx
    }

    fn process_node(&self, node_id: NodeId, cx: &mut GraphContext) {
        let node = self.get_node(&node_id);
        let template = self.get_template(node.template());
        let input_values = self.get_input_values_for_node(node_id);
        let mut node_cx = NodeContext::new(cx, input_values);
        template.process(&mut node_cx);
    }

    fn get_input_values_for_node(&self, node_id: NodeId) -> HashMap<SocketId, Value> {
        let node = self.get_node(&node_id);
        let template = self.get_template(node.template());
        let mut input_values = HashMap::new();
        for (id, input) in template.inputs() {
            node.
        }
        input_values
    }
}
