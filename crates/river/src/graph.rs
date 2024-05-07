use std::{cell::RefCell, collections::HashMap};

use slotmap::{SecondaryMap, SlotMap};

use crate::state::GraphState;

slotmap::new_key_type! {
    pub struct NodeId;
    pub struct InputId;
    pub struct OutputId;
    pub struct ConnectionId;
}

pub struct Graph<GraphValue> {
    nodes: SlotMap<NodeId, GraphNode<GraphValue>>,
    inputs: SlotMap<InputId, Input>,
    outputs: SlotMap<OutputId, Output>,
    connections: SecondaryMap<InputId, OutputId>,
}

impl<GraphValue> Graph<GraphValue> {
    pub fn new() -> Self {
        Self {
            nodes: SlotMap::default(),
            inputs: SlotMap::default(),
            outputs: SlotMap::default(),
            connections: SecondaryMap::default(),
        }
    }

    pub fn add_node(&mut self, node: GraphNode<GraphValue>) -> NodeId {
        node.node_impl.build(&node, self);
        self.nodes.insert(node)
    }

    pub fn node(&self, node_id: NodeId) -> Option<&GraphNode<GraphValue>> {
        self.nodes.get(node_id)
    }

    pub fn add_input<F>(&mut self, f: F) -> InputId
    where
        F: FnOnce(InputId) -> Input,
    {
        self.inputs.insert_with_key(f)
    }

    pub fn input(&self, input_id: InputId) -> Option<&Input> {
        self.inputs.get(input_id)
    }

    pub fn add_output<F>(&mut self, f: F) -> OutputId
    where
        F: FnOnce(OutputId) -> Output,
    {
        self.outputs.insert_with_key(f)
    }

    pub fn output(&self, output_id: OutputId) -> Option<&Output> {
        self.outputs.get(output_id)
    }

    pub fn add_connection(&mut self, output_id: OutputId, input_id: InputId) {
        self.connections.insert(input_id, output_id);
    }

    pub fn connection(&self, input_id: InputId) -> Option<&OutputId> {
        self.connections.get(input_id)
    }

    pub(crate) fn output_parent_node(&self, output_id: OutputId) -> NodeId {
        // FIXME: This is very ad-hoc. We should probably just store this in a secondary map?
        for (node_id, node) in &self.nodes {
            if node.outputs.borrow().iter().any(|(_, id)| *id == output_id) {
                return node_id;
            }
        }
        unreachable!()
    }
}

pub struct GraphNode<GraphValue> {
    node_impl: Box<dyn NodeImpl<GraphValue>>,
    inputs: RefCell<HashMap<String, InputId>>,
    outputs: RefCell<HashMap<String, OutputId>>,
}

impl<GraphValue> GraphNode<GraphValue> {
    pub fn new(node_impl: Box<dyn NodeImpl<GraphValue>>) -> Self {
        Self {
            node_impl,
            inputs: RefCell::new(HashMap::new()),
            outputs: RefCell::new(HashMap::new()),
        }
    }

    pub fn input(&self, id: &str) -> InputId {
        *self
            .inputs
            .borrow()
            .get(id)
            .expect("Input id did not match any input created for this node.")
    }

    pub fn add_input(
        &self,
        id: String,
        graph: &mut Graph<GraphValue>,
        label: String,
        kind: InputKind,
    ) {
        let input_id = graph.add_input(|input_id| Input::new(input_id, label, kind));
        self.inputs.borrow_mut().insert(id, input_id);
    }

    pub fn output(&self, id: &str) -> OutputId {
        *self
            .outputs
            .borrow()
            .get(id)
            .expect("Output id did not match any output created for this node.")
    }

    pub fn add_output(
        &self,
        id: String,
        graph: &mut Graph<GraphValue>,
        label: String,
        kind: OutputKind,
    ) {
        let output_id = graph.add_output(|output_id| Output::new(output_id, label, kind));
        self.outputs.borrow_mut().insert(id, output_id);
    }

    pub fn process(
        &self,
        graph: &Graph<GraphValue>,
        state: &mut GraphState<GraphValue>,
    ) -> Option<GraphValue> {
        self.node_impl.process(&self, graph, state)
    }
}

pub trait NodeImpl<GraphValue> {
    fn process(
        &self,
        node: &GraphNode<GraphValue>,
        graph: &Graph<GraphValue>,
        state: &mut GraphState<GraphValue>,
    ) -> Option<GraphValue>;

    fn build(&self, node: &GraphNode<GraphValue>, graph: &mut Graph<GraphValue>);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Input {
    id: InputId,
    label: String,
    kind: InputKind,
}

impl Input {
    pub fn new(id: InputId, label: String, kind: InputKind) -> Self {
        Self { id, label, kind }
    }

    pub fn id(&self) -> &InputId {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn kind(&self) -> &InputKind {
        &self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputKind {
    ConnectionOnly,
    ConstantOnly,
    ConnectionOrConstant,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Output {
    id: OutputId,
    label: String,
    kind: OutputKind,
}

impl Output {
    pub fn new(id: OutputId, label: String, kind: OutputKind) -> Self {
        Self { id, label, kind }
    }

    pub fn id(&self) -> &OutputId {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn kind(&self) -> &OutputKind {
        &self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputKind {
    CalculatedOnly,
    ConstantOnly,
    CalculatedOrConstant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Connection {
    id: ConnectionId,
    source: OutputId,
    target: InputId,
}

impl Connection {
    pub fn new(id: ConnectionId, source: OutputId, target: InputId) -> Self {
        Self { id, source, target }
    }
}
