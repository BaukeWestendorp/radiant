use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
};

#[cfg(feature = "gpui")]
pub mod gpui;
#[cfg(feature = "serde")]
pub mod serde;

pub trait ValueImpl: Clone {}

pub struct Graph<State: Default, Value: ValueImpl> {
    templates: Vec<Template<State, Value>>,
    /// Leaf nodes are nodes that have no outgoing edges
    /// and should be the first nodes that are processed.
    leaf_nodes: Vec<NodeId>,
    node_id_counter: AtomicU32,

    nodes: HashMap<NodeId, Node<Value>>,
    edges: Vec<Edge>,
}

impl<State: Default + 'static, Value: ValueImpl + 'static> Default for Graph<State, Value> {
    fn default() -> Self {
        Self::new()
    }
}

impl<State: Default + 'static, Value: ValueImpl + 'static> Graph<State, Value> {
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

    pub fn node(&self, node_id: &NodeId) -> &Node<Value> {
        self.nodes.get(node_id).expect("should always return a node for given node_id")
    }

    pub fn nodes(&self) -> impl Iterator<Item = (&NodeId, &Node<Value>)> {
        self.nodes.iter()
    }

    pub fn add_node(
        &mut self,
        node: Node<Value>,
        #[cfg(feature = "gpui")] cx: &mut ::gpui::Context<Self>,
    ) -> NodeId {
        let node_id = NodeId(self.next_node_id());
        self._add_node(node_id, node);

        #[cfg(feature = "gpui")]
        cx.emit(gpui::GraphEvent::NodeAdded(node_id));

        node_id
    }

    fn _add_node(&mut self, node_id: NodeId, node: Node<Value>) {
        self.nodes.insert(node_id, node);
        self.leaf_nodes.push(node_id);
    }

    pub fn remove_node(
        &mut self,
        node_id: NodeId,
        #[cfg(feature = "gpui")] cx: &mut ::gpui::Context<Self>,
    ) {
        // Remove all edges that are connected to this node.
        self.edges.retain(|Edge { source, target }| {
            source.node_id != node_id && target.node_id != node_id
        });

        self.remove_leaf_node(&node_id);

        self.nodes.remove(&node_id);

        #[cfg(feature = "gpui")]
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

    pub fn add_edge(
        &mut self,
        edge: Edge,
        #[cfg(feature = "gpui")] cx: &mut ::gpui::Context<Self>,
    ) {
        self._add_edge(edge.clone());
        #[cfg(feature = "gpui")]
        cx.emit(gpui::GraphEvent::EdgeAdded { edge });
    }

    fn _add_edge(&mut self, edge: Edge) {
        self.remove_leaf_node(&edge.source.node_id);
        self.edges.push(edge);
    }

    pub fn remove_edge_from_source(
        &mut self,
        source: &Socket,
        #[cfg(feature = "gpui")] cx: &mut ::gpui::Context<Self>,
    ) {
        self.edges.retain(|edge| &edge.source != source);

        #[cfg(feature = "gpui")]
        cx.emit(gpui::GraphEvent::EdgeRemoved { source: source.clone() });
    }

    pub fn process(&self, pcx: &mut ProcessingContext<State, Value>) {
        for node_id in &self.leaf_nodes {
            self.process_node(node_id, pcx);
        }
    }

    fn process_node(&self, node_id: &NodeId, pcx: &mut ProcessingContext<State, Value>) {
        let node = self.node(node_id);
        let template = self.template(&node.template_id);

        // Calculate inputs.
        let mut input_values = SocketValues::new();
        for (input_id, default_value) in template.default_input_values().0 {
            // If the input is connected to an edge, get the value from the edge source.
            let value =
                if let Some(source) = self.edge_source(&Socket::new(*node_id, input_id.clone())) {
                    self.get_output_value(source, pcx)
                }
                // Else if the input has a value, use it.
                else if let Some(value) = node.input_values().get_value(&input_id) {
                    value.clone()
                }
                // Else use the default value.
                else {
                    default_value
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

impl<State: Default, Value: ValueImpl> ProcessingContext<State, Value> {
    pub fn state(&self) -> &State {
        &self.state
    }

    fn cache_output_values(&mut self, node_id: NodeId, output_values: SocketValues<Value>) {
        for (output_id, value) in output_values.values() {
            let socket = Socket::new(node_id, output_id.clone());
            self.output_value_cache.insert(socket, value.clone());
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

#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[derive(Clone, Default)]
pub struct SocketValues<Value: ValueImpl>(HashMap<String, Value>);

impl<Value: ValueImpl> SocketValues<Value> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn set_value(&mut self, id: impl Into<String>, value: Value) {
        self.0.insert(id.into(), value);
    }

    pub fn get_value(&self, id: &str) -> Option<&Value> {
        self.0.get(id)
    }

    pub fn values(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.0.iter()
    }
}

#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
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

pub struct Template<State: Default, Value: ValueImpl> {
    id: TemplateId,

    label: String,

    inputs: Vec<Input<Value>>,
    outputs: Vec<Output>,

    processor: Box<Processor<State, Value>>,
}

impl<State: Default, Value: ValueImpl> Template<State, Value> {
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

#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u32);

#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[derive(Clone)]
pub struct Node<Value: ValueImpl> {
    template_id: TemplateId,
    #[serde(default = "SocketValues::new")]
    input_values: SocketValues<Value>,
}

impl<Value: ValueImpl> Node<Value> {
    pub fn new(template_id: impl Into<TemplateId>) -> Self {
        Self { template_id: template_id.into(), input_values: SocketValues::new() }
    }

    pub fn template_id(&self) -> &TemplateId {
        &self.template_id
    }

    pub fn input_values(&self) -> &SocketValues<Value> {
        &self.input_values
    }

    pub fn input_values_mut(&mut self) -> &mut SocketValues<Value> {
        &mut self.input_values
    }
}

#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Edge {
    source: Socket,
    target: Socket,
}

#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
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
pub struct Input<Value: ValueImpl> {
    id: String,
    label: String,
    default: Value,
}

impl<Value: ValueImpl> Input<Value> {
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
