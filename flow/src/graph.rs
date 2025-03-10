use std::collections::HashMap;

use crate::{Edge, Input, Node, NodeId, Output, Socket, Template, TemplateId};

pub trait GraphDef: Clone {
    #[cfg(feature = "serde")]
    type Value: Clone + ::serde::Serialize + for<'de> ::serde::Deserialize<'de>;
    #[cfg(not(feature = "serde"))]
    type Value: Clone;

    type DataType: DataType<Self> + Clone;

    type ProcessingState: Default;
}

pub trait DataType<D: GraphDef> {
    fn try_cast(&self, from: &D::Value) -> Option<D::Value>;

    fn default_value(&self) -> D::Value;
}

#[derive(Clone)]
pub struct Graph<D: GraphDef> {
    templates: Vec<Template<D>>,

    pub(crate) nodes: HashMap<NodeId, Node<D>>,
    pub(crate) edges: Vec<Edge>,

    /// Leaf nodes are nodes that have no outgoing edges
    /// and should be the first nodes that are processed.
    leaf_nodes: Vec<NodeId>,
    pub(crate) node_id_counter: u32,
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
            leaf_nodes: Vec::new(),
            node_id_counter: 0,

            nodes: HashMap::new(),
            edges: Vec::new(),
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
        self.edges.retain(|Edge { source, target }| {
            source.node_id != *node_id && target.node_id != *node_id
        });

        self.remove_leaf_node(node_id);

        self.nodes.remove(node_id);
    }

    pub fn edge_source(&self, target: &Socket) -> Option<&Socket> {
        self.edges.iter().find(|edge| &edge.target == target).map(|edge| &edge.source)
    }

    pub fn edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.iter()
    }

    fn validate_edge(&self, edge: &Edge) -> bool {
        let source = self.input(&edge.source);
        let target = self.output(&edge.target);

        target.data_type().try_cast(&source.data_type().default_value()).is_some()
    }

    pub fn add_edge(&mut self, edge: Edge, validate: bool) {
        if validate && !self.validate_edge(&edge) {
            return;
        }

        self.remove_leaf_node(&edge.source.node_id);
        self.edges.push(edge);
    }

    pub fn remove_edge(&mut self, source: &Socket) {
        self.edges.retain(|edge| &edge.source != source);
    }

    pub fn input(&self, socket: &Socket) -> &Input<D> {
        let template_id = self.node(&socket.node_id).template_id();
        self.template(template_id)
            .inputs()
            .iter()
            .find(|i| i.id() == socket.id)
            .expect("should have found input")
    }

    pub fn output(&self, socket: &Socket) -> &Output<D> {
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
            // If the input is connected to an edge, get the value from the edge source.
            let value =
                if let Some(source) = self.edge_source(&Socket::new(*node_id, input_id.clone())) {
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

    fn get_output_value(&self, output_socket: &Socket, pcx: &mut ProcessingContext<D>) -> D::Value {
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
    output_value_cache: HashMap<Socket, D::Value>,
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
            let socket = Socket::new(node_id, output_id.clone());
            self.output_value_cache.insert(socket, value.clone());
        }
    }

    fn get_cached_output_value(&self, output_socket: &Socket) -> Option<&D::Value> {
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
