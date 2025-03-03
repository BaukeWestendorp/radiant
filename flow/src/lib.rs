use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
};

pub mod gpui;

pub struct Graph<State: Default, Value: Clone> {
    templates: Vec<Template<State, Value>>,
    /// Leaf nodes are nodes that have no outgoing edges
    /// and should be the first nodes that are processed.
    leaf_nodes: Vec<NodeId>,
    node_id_counter: AtomicU32,

    nodes: HashMap<NodeId, Node>,
    edges: Vec<Edge>,
}

impl<'de, State: Default + 'static, Value: Clone + 'static> serde::Deserialize<'de>
    for Graph<State, Value>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Intermediate {
            nodes: HashMap<NodeId, Node>,
            edges: Vec<Edge>,
        }

        let intermediate = Intermediate::deserialize(deserializer)?;

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

impl<'t, State: Default + 'static, Value: Clone + 'static> Graph<State, Value> {
    pub fn new() -> Self {
        Self {
            templates: Vec::new(),
            leaf_nodes: Vec::new(),
            node_id_counter: AtomicU32::new(0),

            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_template(&mut self, template: Template<State, Value>) {
        self.templates.push(template);
    }

    pub fn template(&self, template_id: &TemplateId) -> &Template<State, Value> {
        self.templates
            .iter()
            .find(|template| template.id == *template_id)
            .expect("should always return a template for given template_id")
    }

    pub fn templates(&self) -> impl Iterator<Item = &Template<State, Value>> {
        self.templates.iter()
    }

    pub fn node(&self, node_id: &NodeId) -> &Node {
        self.nodes.get(node_id).expect("should always return a node for given node_id")
    }

    pub fn nodes(&self) -> impl Iterator<Item = (&NodeId, &Node)> {
        self.nodes.iter()
    }

    pub fn add_node(&mut self, node: Node, cx: &mut ::gpui::Context<Self>) -> NodeId {
        let node_id = NodeId(self.next_node_id());
        self._add_node(node_id, node);

        cx.emit(gpui::GraphEvent::NodeAdded(node_id));
        node_id
    }

    fn _add_node(&mut self, node_id: NodeId, node: Node) {
        self.nodes.insert(node_id, node);
        self.leaf_nodes.push(node_id);
    }

    pub fn remove_node(&mut self, node_id: NodeId, cx: &mut ::gpui::Context<Self>) {
        // Remove all edges that are connected to this node.
        self.edges.retain(|Edge { source, target }| {
            source.node_id != node_id && target.node_id != node_id
        });

        self.remove_leaf_node(&node_id);

        self.nodes.remove(&node_id);

        cx.emit(gpui::GraphEvent::NodeRemoved(node_id));
    }

    fn next_node_id(&self) -> u32 {
        self.node_id_counter.fetch_add(1, Ordering::Relaxed)
    }

    pub fn edge_source(&self, target: &Socket) -> Option<&Socket> {
        self.edges.iter().find(|edge| &edge.target == target).map(|edge| &edge.source)
    }

    pub fn edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.iter()
    }

    pub fn add_edge(&mut self, edge: Edge, cx: &mut ::gpui::Context<Self>) {
        self._add_edge(edge.clone());
        cx.emit(gpui::GraphEvent::EdgeAdded { edge });
    }

    fn _add_edge(&mut self, edge: Edge) {
        self.remove_leaf_node(&edge.source.node_id);
        self.edges.push(edge);
    }

    pub fn remove_edge_from_source(&mut self, source: &Socket, cx: &mut ::gpui::Context<Self>) {
        self.edges.retain(|edge| &edge.source != source);
        cx.emit(gpui::GraphEvent::EdgeRemoved { source: source.clone() });
    }

    pub fn process(&self, pcx: &mut ProcessingContext<State, Value>) {
        for node_id in &self.leaf_nodes {
            self.process_node(&node_id, pcx);
        }
    }

    fn process_node(&self, node_id: &NodeId, pcx: &mut ProcessingContext<State, Value>) {
        let node = self.node(node_id);
        let template = self.template(&node.template_id);

        // Calculate inputs.
        let mut input_values = SocketValues::new();
        for (input_id, value) in template.default_input_values().values() {
            let target = Socket::new(*node_id, input_id.to_owned());
            let value = match self.edge_source(&target) {
                Some(source) => self.get_output_value(&source, pcx),
                _ => value.clone(),
            };
            input_values.set_value(input_id, value);
        }

        // Calculate outputs and update context.
        let mut output_values = SocketValues::new();
        (template.processor)(&input_values, &mut output_values, pcx);

        // Update output value cache.
        pcx.cache_output_values(*node_id, output_values);
    }

    fn get_output_value(
        &self,
        output_socket: &Socket,
        pcx: &mut ProcessingContext<State, Value>,
    ) -> Value {
        if let Some(value) = pcx.get_cached_output_value(output_socket) {
            return value.clone();
        }

        self.process_node(&output_socket.node_id, pcx);
        pcx.get_cached_output_value(output_socket)
            .expect("output value should have been calculated by processing the node")
            .clone()
    }

    fn remove_leaf_node(&mut self, node_id: &NodeId) {
        self.leaf_nodes.retain(|id| id != node_id);
    }
}

type Processor<State, Value> =
    dyn Fn(&SocketValues<Value>, &mut SocketValues<Value>, &mut ProcessingContext<State, Value>);

#[derive(Debug, Default, PartialEq)]
pub struct ProcessingContext<State: Default, Value> {
    state: State,
    output_value_cache: HashMap<Socket, Value>,
}

impl<State: Default, Value> ProcessingContext<State, Value> {
    pub fn state(&self) -> &State {
        &self.state
    }

    fn cache_output_values(&mut self, node_id: NodeId, output_values: SocketValues<Value>) {
        for (output_id, value) in output_values.values {
            let socket = Socket::new(node_id, output_id.clone());
            self.output_value_cache.insert(socket, value);
        }
    }

    fn get_cached_output_value(&self, output_socket: &Socket) -> Option<&Value> {
        self.output_value_cache.get(output_socket)
    }
}

impl<State: Default, Value> std::ops::Deref for ProcessingContext<State, Value> {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<State: Default, Value> std::ops::DerefMut for ProcessingContext<State, Value> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

#[derive(Debug, Clone, Default)]
pub struct SocketValues<Value> {
    values: HashMap<String, Value>,
}

impl<Value> SocketValues<Value> {
    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }

    pub fn set_value(&mut self, id: impl Into<String>, value: Value) {
        self.values.insert(id.into(), value);
    }

    pub fn get_value(&self, id: &str) -> &Value {
        self.values.get(id).expect("should have socket value for id")
    }

    pub fn values(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.values.iter()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TemplateId(pub ::gpui::SharedString);

impl From<String> for TemplateId {
    fn from(id: String) -> Self {
        TemplateId(::gpui::SharedString::new(id))
    }
}

impl From<&str> for TemplateId {
    fn from(id: &str) -> Self {
        TemplateId(::gpui::SharedString::new(id.to_string()))
    }
}

pub struct Template<State: Default, Value: Clone> {
    id: TemplateId,

    label: String,

    inputs: Vec<Input<Value>>,
    outputs: Vec<Output>,

    processor: Box<Processor<State, Value>>,
}

impl<State: Default, Value: Clone> Template<State, Value> {
    pub fn new(
        id: impl Into<TemplateId>,
        label: impl Into<String>,
        inputs: Vec<Input<Value>>,
        outputs: Vec<Output>,
        processor: Box<Processor<State, Value>>,
    ) -> Self {
        Self { id: id.into(), label: label.into(), inputs, outputs, processor }
    }

    pub fn id(&self) -> &TemplateId {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn inputs(&self) -> &[Input<Value>] {
        &self.inputs
    }

    pub fn outputs(&self) -> &[Output] {
        &self.outputs
    }

    pub fn default_input_values(&self) -> SocketValues<Value> {
        let mut values = SocketValues::new();
        for input in &self.inputs {
            values.set_value(input.id.clone(), input.default.clone());
        }
        values
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u32);

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
pub struct Node {
    template_id: TemplateId,
}

impl Node {
    pub fn new(template_id: impl Into<TemplateId>) -> Self {
        Self { template_id: template_id.into() }
    }

    pub fn template_id(&self) -> &TemplateId {
        &self.template_id
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
pub struct Edge {
    source: Socket,
    target: Socket,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Socket {
    pub node_id: NodeId,
    pub id: String,
}

impl Socket {
    pub fn new(node_id: NodeId, id: String) -> Self {
        Self { node_id, id }
    }
}

#[derive(Debug, Clone)]
pub struct Input<Value: Clone> {
    id: String,
    label: String,
    default: Value,
}

impl<Value: Clone> Input<Value> {
    pub fn new(id: impl Into<String>, label: impl Into<String>, default: Value) -> Self {
        Self { id: id.into(), label: label.into(), default }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn default(&self) -> &Value {
        &self.default
    }
}

#[derive(Debug, Clone)]
pub struct Output {
    id: String,
    label: String,
}

impl Output {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self { id: id.into(), label: label.into() }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}
