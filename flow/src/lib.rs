use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
};

#[derive(Default)]
pub struct Graph<State: Default, Value: Clone> {
    templates: HashMap<TemplateId, Template<State, Value>>,
    nodes: HashMap<NodeId, Node>,
    /// Key: target socket, Value: source socket
    edges: HashMap<Socket, Socket>,

    /// Leaf nodes are nodes that have no outgoing edges
    /// and should be the first nodes that are processed.
    leaf_nodes: Vec<NodeId>,

    id_counter: AtomicU32,
}

impl<State: Default, Value: Clone> Graph<State, Value> {
    fn next_id(&self) -> u32 {
        self.id_counter.fetch_add(1, Ordering::Relaxed)
    }

    pub fn template(&self, template_id: &TemplateId) -> &Template<State, Value> {
        self.templates
            .get(template_id)
            .expect("should always return a template for given template_id")
    }

    pub fn templates(&self) -> impl Iterator<Item = (&TemplateId, &Template<State, Value>)> {
        self.templates.iter()
    }

    pub fn add_template(&mut self, template: Template<State, Value>) -> TemplateId {
        let template_id = TemplateId(self.next_id());
        self.templates.insert(template_id, template);
        template_id
    }

    pub fn node(&self, node_id: &NodeId) -> &Node {
        self.nodes.get(node_id).expect("should always return a node for given node_id")
    }

    pub fn nodes(&self) -> impl Iterator<Item = (&NodeId, &Node)> {
        self.nodes.iter()
    }

    pub fn add_node(&mut self, node: Node) -> NodeId {
        let node_id = NodeId(self.next_id());
        self.nodes.insert(node_id, node);
        self.leaf_nodes.push(node_id);
        node_id
    }

    pub fn remove_node(&mut self, node_id: NodeId) {
        // Remove all edges that are connected to this node.
        self.edges.retain(|source, target| source.node_id != node_id && target.node_id != node_id);

        self.remove_leaf_node(&node_id);

        self.nodes.remove(&node_id);
    }

    pub fn edge_source(&self, target: &Socket) -> Option<&Socket> {
        self.edges.get(target)
    }

    pub fn edges(&self) -> impl Iterator<Item = (&Socket, &Socket)> {
        self.edges.iter()
    }

    pub fn add_edge(&mut self, source: Socket, target: Socket) {
        self.remove_leaf_node(&source.node_id);
        self.edges.insert(target, source);
    }

    pub fn remove_edge_from_source(&mut self, source: &Socket) {
        self.edges.retain(|edge_source, _| edge_source != source);
    }

    pub fn process(&self, cx: &mut ProcessingContext<State, Value>) {
        for node_id in &self.leaf_nodes {
            self.process_node(&node_id, cx);
        }
    }

    fn process_node(&self, node_id: &NodeId, cx: &mut ProcessingContext<State, Value>) {
        let node = self.node(node_id);
        let template = self.template(&node.template_id);

        // Calculate inputs.
        let mut input_values = SocketValues::new();
        for (input_id, value) in template.default_input_values().values() {
            let target = Socket::new(*node_id, input_id.to_owned());
            let value = match self.edge_source(&target) {
                Some(source) => self.get_output_value(&source, cx),
                _ => value.clone(),
            };
            input_values.set_value(input_id, value);
        }

        // Calculate outputs and update context.
        let mut output_values = SocketValues::new();
        (template.processor)(&input_values, &mut output_values, cx);

        // Update output value cache.
        cx.cache_output_values(*node_id, output_values);
    }

    fn get_output_value(
        &self,
        output_socket: &Socket,
        cx: &mut ProcessingContext<State, Value>,
    ) -> Value {
        if let Some(value) = cx.get_cached_output_value(output_socket) {
            return value.clone();
        }

        self.process_node(&output_socket.node_id, cx);
        cx.get_cached_output_value(output_socket)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TemplateId(pub u32);

pub struct Template<State: Default, Value: Clone> {
    label: String,

    inputs: Vec<Input<Value>>,
    outputs: Vec<Output>,

    processor: Box<Processor<State, Value>>,
}

impl<State: Default, Value: Clone> Template<State, Value> {
    pub fn new(
        label: impl Into<String>,
        inputs: Vec<Input<Value>>,
        outputs: Vec<Output>,
        processor: Box<Processor<State, Value>>,
    ) -> Self {
        Self { label: label.into(), inputs, outputs, processor }
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u32);

#[derive(Debug, Clone)]
pub struct Node {
    template_id: TemplateId,
}

impl Node {
    pub fn new(template_id: TemplateId) -> Self {
        Self { template_id }
    }

    pub fn template_id(&self) -> &TemplateId {
        &self.template_id
    }
}

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

#[cfg(test)]
mod tests {
    use crate::{Graph, Input, Node, Output, ProcessingContext, Socket, Template};

    #[test]
    fn smoke_test() {
        type Value = f64;

        #[derive(Default)]
        struct State {
            value: Value,
        }

        let mut graph = Graph::<State, Value>::default();

        let new_value_id = graph.add_template(Template::new(
            "New Value",
            vec![],
            vec![Output::new("value", "Value")],
            Box::new(|_, output_values, _| {
                output_values.set_value("value", 42.0);
            }),
        ));

        let output_value_id = graph.add_template(Template::new(
            "Output Value",
            vec![Input::new("value", "Value", 0.0)],
            vec![],
            Box::new(|input_values, _, cx| {
                let value = input_values.get_value("value");
                cx.value = value.clone();
            }),
        ));

        let new_value_node_id = graph.add_node(Node::new(new_value_id));
        let output_value_node_id = graph.add_node(Node::new(output_value_id));

        graph.add_edge(
            Socket::new(new_value_node_id, "value".to_string()),
            Socket::new(output_value_node_id, "value".to_string()),
        );

        let mut cx = ProcessingContext::default();
        graph.process(&mut cx);

        assert_eq!(cx.value, 42.0);
    }
}
