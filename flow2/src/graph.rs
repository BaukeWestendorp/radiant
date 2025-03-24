use std::collections::HashMap;

use crate::{Input, InputSocket, Node, NodeId, Output, OutputSocket, Template, TemplateId};

pub trait GraphDef: Clone {
    #[cfg(feature = "serde")]
    type Value: Value<Self> + Clone + ::serde::Serialize + for<'de> ::serde::Deserialize<'de>;
    #[cfg(not(feature = "serde"))]
    type Value: Value<Self> + Clone;

    type DataType: DataType<Self> + Clone;

    type ProcessingState: Default;
}

pub trait Value<D: GraphDef> {
    fn cast_to(&self, to: &D::DataType) -> Option<D::Value>;

    fn data_type(&self) -> D::DataType;
}

pub trait DataType<D: GraphDef> {
    fn default_value(&self) -> D::Value;

    fn can_cast_to(&self, target_type: &D::DataType) -> bool {
        self.default_value().cast_to(target_type).is_some()
    }
}

#[derive(Clone)]
pub struct Graph<D: GraphDef> {
    templates: Vec<Template<D>>,

    pub(crate) nodes: HashMap<NodeId, Node<D>>,
    pub(crate) edges: HashMap<InputSocket, OutputSocket>,

    pub(crate) node_id_counter: u32,
    /// Leaf nodes are nodes that have no outgoing edges
    /// and should be the first nodes that are processed.
    leaf_nodes: Vec<NodeId>,
}

impl<D: GraphDef> Default for Graph<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: GraphDef> Graph<D> {
    pub fn new() -> Self {
        Self {
            templates: Vec::new(),

            nodes: HashMap::new(),
            edges: HashMap::new(),

            leaf_nodes: Vec::new(),
            node_id_counter: 0,
        }
    }

    pub fn add_template(&mut self, template: Template<D>) {
        self.templates.push(template);
    }

    pub fn add_templates(&mut self, templates: impl IntoIterator<Item = Template<D>>) {
        self.templates.extend(templates);
    }

    pub fn template(&self, template_id: &TemplateId) -> &Template<D> {
        self.templates.iter().find(|template| template.id() == template_id).unwrap_or_else(|| {
            panic!(
                "should always return a template for given template_id: found '{}'",
                template_id.0
            )
        })
    }

    pub fn templates(&self) -> impl Iterator<Item = &Template<D>> {
        self.templates.iter()
    }

    fn next_node_id(&mut self) -> u32 {
        let id = self.node_id_counter;
        self.node_id_counter = id + 1;
        id
    }

    pub fn node(&self, node_id: &NodeId) -> &Node<D> {
        self.nodes.get(node_id).unwrap_or_else(|| {
            panic!("should always return a node for given node_id: found '{node_id:?}'")
        })
    }

    pub fn nodes(&self) -> impl Iterator<Item = (&NodeId, &Node<D>)> {
        self.nodes.iter()
    }

    pub fn node_ids(&self) -> impl Iterator<Item = &NodeId> {
        self.nodes.keys()
    }

    pub fn add_node(&mut self, node: Node<D>) -> NodeId {
        let node_id = NodeId(self.next_node_id());

        self.add_node_with_id(node_id, node);

        node_id
    }

    pub(crate) fn add_node_with_id(&mut self, node_id: NodeId, node: Node<D>) {
        self.nodes.insert(node_id, node);
        self.leaf_nodes.push(node_id);
    }

    pub fn remove_node(&mut self, node_id: &NodeId) {
        // Remove all edges that are connected to this node.
        self.edges
            .retain(|target, source| source.node_id != *node_id && target.node_id != *node_id);

        self.remove_leaf_node(node_id);

        self.nodes.remove(node_id);
    }

    pub fn edge_source(&self, target: &InputSocket) -> Option<&OutputSocket> {
        self.edges.get(target)
    }

    pub fn edges(&self) -> impl Iterator<Item = (&InputSocket, &OutputSocket)> {
        self.edges.iter()
    }

    fn validate_edge(&self, target: &InputSocket, source: &OutputSocket) -> bool {
        let output = self.output(&source);
        let input = self.input(&target);

        // An edge can't connect to itself.
        if source.node_id == target.node_id {
            return false;
        }

        // An input can't have multiple edges connected.
        if self.edges().any(|(t, _)| t == target) {
            return false;
        }

        // An edge can only exist between two castable types.
        input.default().cast_to(output.data_type()).is_some()
    }

    pub fn add_edge(&mut self, target: InputSocket, source: OutputSocket, validate: bool) {
        if validate && !self.validate_edge(&target, &source) {
            return;
        }

        self.remove_leaf_node(&source.node_id);
        self.edges.insert(target, source);
    }

    pub fn remove_edge(&mut self, target: &InputSocket) {
        self.edges.remove(target);
    }

    pub fn input(&self, socket: &InputSocket) -> &Input<D> {
        let template_id = self.node(&socket.node_id).template_id();
        self.template(template_id)
            .inputs()
            .iter()
            .find(|i| i.id() == socket.id)
            .expect("should have found input")
    }

    pub fn output(&self, socket: &OutputSocket) -> &Output<D> {
        let template_id = self.node(&socket.node_id).template_id();
        self.template(template_id)
            .outputs()
            .iter()
            .find(|o| o.id() == socket.id)
            .expect("should have found output")
    }

    pub fn process(&self, pcx: &mut ProcessingContext<D>) {
        for node_id in &self.leaf_nodes {
            self.process_node(node_id, pcx);
        }
    }

    fn process_node(&self, node_id: &NodeId, pcx: &mut ProcessingContext<D>) {
        let node = self.node(node_id);
        let template = self.template(node.template_id());

        // Calculate inputs.
        let mut input_values = SocketValues::new();
        for (input_id, default_value) in template.default_input_values().0 {
            let socket = InputSocket::new(*node_id, input_id.clone());

            // If the input is connected to an edge, get the value from the edge source.
            let value = if let Some(source) = self.edge_source(&socket) {
                self.get_output_value(source, pcx)
            }
            // Else, if the input has a value, use it.
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
        template.process(&input_values, &mut output_values, pcx);

        // Update output value cache.
        pcx.cache_output_values(*node_id, output_values);
    }

    fn get_output_value(
        &self,
        output_socket: &OutputSocket,
        pcx: &mut ProcessingContext<D>,
    ) -> D::Value {
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

#[derive(Debug, PartialEq)]
pub struct ProcessingContext<D: GraphDef> {
    state: D::ProcessingState,
    output_value_cache: HashMap<OutputSocket, D::Value>,
}

impl<D: GraphDef> Default for ProcessingContext<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: GraphDef> ProcessingContext<D> {
    pub fn new() -> Self {
        Self { state: D::ProcessingState::default(), output_value_cache: HashMap::new() }
    }

    pub fn state(&self) -> &D::ProcessingState {
        &self.state
    }

    fn cache_output_values(&mut self, node_id: NodeId, output_values: SocketValues<D>) {
        for (output_id, value) in output_values.values() {
            let socket = OutputSocket::new(node_id, output_id.clone());
            self.output_value_cache.insert(socket, value.clone());
        }
    }

    fn get_cached_output_value(&self, output_socket: &OutputSocket) -> Option<&D::Value> {
        self.output_value_cache.get(output_socket)
    }
}

impl<D: GraphDef> std::ops::Deref for ProcessingContext<D> {
    type Target = D::ProcessingState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<D: GraphDef> std::ops::DerefMut for ProcessingContext<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[derive(Clone, Default)]
pub struct SocketValues<D: GraphDef>(HashMap<String, D::Value>);

impl<D: GraphDef> SocketValues<D> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn set_value(&mut self, id: impl Into<String>, value: D::Value) {
        self.0.insert(id.into(), value);
    }

    pub fn get_value(&self, id: &str) -> Option<&D::Value> {
        self.0.get(id)
    }

    pub fn values(&self) -> impl Iterator<Item = (&String, &D::Value)> {
        self.0.iter()
    }
}
