use std::collections::HashMap;

use gpui::*;

use crate::{
    Input, InputSocket, Node, NodeId, Output, OutputSocket, Template, TemplateId,
    gpui::{ControlView, GraphEvent},
};

pub trait GraphDef: Clone {
    #[cfg(feature = "serde")]
    type Value: Value<Self> + Clone + ::serde::Serialize + for<'de> ::serde::Deserialize<'de>;
    #[cfg(not(feature = "serde"))]
    type Value: Value<Self> + Clone;

    type DataType: DataType<Self> + PartialEq + Clone;
    type Control: Control<Self> + Clone;

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

    fn color(&self) -> gpui::Hsla;
}

pub trait Control<D: GraphDef> {
    fn build_view(
        &self,
        value: D::Value,
        id: gpui::ElementId,
        window: &mut gpui::Window,
        cx: &mut gpui::App,
    ) -> Entity<ControlView>;
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

    pub(crate) node_positions: HashMap<NodeId, Point<Pixels>>,
    pub(crate) offset: Point<Pixels>,
    pub(crate) dragged_node_position: Option<(NodeId, Point<Pixels>)>,
}

impl<D: GraphDef + 'static> Default for Graph<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: GraphDef + 'static> Graph<D> {
    pub fn new() -> Self {
        Self {
            templates: Vec::new(),

            nodes: HashMap::new(),
            edges: HashMap::new(),

            leaf_nodes: Vec::new(),
            node_id_counter: 0,

            node_positions: HashMap::new(),
            offset: Point::default(),
            dragged_node_position: None,
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

    pub fn node_mut(&mut self, node_id: &NodeId) -> &mut Node<D> {
        self.nodes.get_mut(node_id).unwrap_or_else(|| {
            panic!("should always return a node for given node_id: found '{node_id:?}'")
        })
    }

    pub fn nodes(&self) -> impl Iterator<Item = (&NodeId, &Node<D>)> {
        self.nodes.iter()
    }

    pub fn node_ids(&self) -> impl Iterator<Item = &NodeId> {
        self.nodes.keys()
    }

    pub fn add_node(
        &mut self,
        node: Node<D>,
        position: Point<Pixels>,
        cx: &mut Context<Self>,
    ) -> NodeId {
        let node_id = NodeId(self.next_node_id());
        self.add_node_internal(node_id, node, position);
        cx.emit(GraphEvent::NodeAdded(node_id));
        node_id
    }

    pub(crate) fn add_node_internal(
        &mut self,
        node_id: NodeId,
        node: Node<D>,
        position: Point<Pixels>,
    ) {
        self.nodes.insert(node_id, node);
        self.leaf_nodes.push(node_id);
        self.node_positions.insert(node_id, position);
    }

    pub fn remove_node(&mut self, node_id: &NodeId, cx: &mut Context<Self>) {
        // Remove all edges that are connected to this node.
        self.edges
            .retain(|target, source| source.node_id != *node_id && target.node_id != *node_id);
        self.remove_leaf_node(node_id);
        self.node_positions.remove(node_id);
        self.nodes.remove(node_id);
        cx.emit(GraphEvent::NodeRemoved(*node_id));
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

    pub fn add_edge(&mut self, target: InputSocket, source: OutputSocket, cx: &mut Context<Self>) {
        if !self.validate_edge(&target, &source) {
            return;
        }

        self.add_edge_internal(target.clone(), source.clone());

        cx.emit(GraphEvent::EdgeAdded { target, source })
    }

    pub(crate) fn add_edge_internal(&mut self, target: InputSocket, source: OutputSocket) {
        self.remove_leaf_node(&source.node_id);
        self.edges.insert(target, source);
    }

    pub fn remove_edge(&mut self, target: &InputSocket, cx: &mut Context<Self>) {
        self.edges.remove(target);
        cx.emit(GraphEvent::EdgeRemoved { target: target.clone() });
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
        let mut input_values = Values::new();
        for (input_id, value) in node.input_values().values() {
            let socket = InputSocket::new(*node_id, input_id.clone());

            let value = if let Some(source) = self.edge_source(&socket) {
                self.get_output_value(source, pcx)
            } else {
                value.clone()
            };

            input_values.set_value(input_id, value);
        }

        // Control values.
        let control_values = node.control_values();

        // Calculate outputs and update context.
        let mut output_values = Values::new();
        template.process(&input_values, &control_values, &mut output_values, pcx);

        // Update output value cache.
        pcx.cache_output_values(*node_id, output_values);
    }

    pub fn input_value(&self, socket: &InputSocket) -> &D::Value {
        self.node(&socket.node_id)
            .input_values()
            .value(&socket.id)
            .expect("should always have a value for an input")
    }

    pub fn set_input_value(&mut self, socket: InputSocket, value: D::Value) {
        self.node_mut(&socket.node_id).input_values_mut().set_value(socket.id.clone(), value);
    }

    pub fn node_control_value(&self, node_id: &NodeId, id: &str) -> &D::Value {
        self.node(node_id)
            .control_values()
            .value(id)
            .expect("should always have a value for a node control")
    }

    pub fn set_control_value(&mut self, node_id: &NodeId, id: String, value: D::Value) {
        self.node_mut(&node_id).control_values_mut().set_value(id, value)
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

    pub fn node_position(&self, node_id: &NodeId) -> &Point<Pixels> {
        self.node_positions.get(node_id).expect("should have a position for every NodeId")
    }

    pub fn set_node_position(&mut self, node_id: NodeId, position: Point<Pixels>) {
        self.node_positions.insert(node_id, position);
    }

    pub fn visual_node_position(&self, node_id: &NodeId) -> &Point<Pixels> {
        match &self.dragged_node_position {
            Some((dragged_node_id, position)) if dragged_node_id == node_id => position,
            _ => self.node_position(node_id),
        }
    }

    pub fn update_visual_node_position(&mut self, position: Option<(NodeId, Point<Pixels>)>) {
        self.dragged_node_position = position;
    }

    pub fn offset(&self) -> &Point<Pixels> {
        &self.offset
    }

    pub fn set_offset(&mut self, offset: Point<Pixels>) {
        self.offset = offset;
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

    fn cache_output_values(&mut self, node_id: NodeId, output_values: Values<D>) {
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
pub struct Values<D: GraphDef>(HashMap<String, D::Value>);

impl<D: GraphDef> Values<D> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn set_value(&mut self, id: impl Into<String>, value: D::Value) {
        self.0.insert(id.into(), value);
    }

    pub fn value(&self, id: &str) -> Option<&D::Value> {
        self.0.get(id)
    }

    pub fn values(&self) -> impl Iterator<Item = (&String, &D::Value)> {
        self.0.iter()
    }
}
