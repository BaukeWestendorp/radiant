use std::{cell::RefCell, collections::HashMap};

use super::{GraphState, GraphValue, NodeKind, ValueType};
use slotmap::{SecondaryMap, SlotMap};

slotmap::new_key_type! {
    pub struct NodeId;
    pub struct InputId;
    pub struct OutputId;
    pub struct ConnectionId;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Graph {
    nodes: SlotMap<NodeId, GraphNode>,
    inputs: SlotMap<InputId, Input>,
    outputs: SlotMap<OutputId, Output>,
    connections: SecondaryMap<InputId, OutputId>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: SlotMap::default(),
            inputs: SlotMap::default(),
            outputs: SlotMap::default(),
            connections: SecondaryMap::default(),
        }
    }

    pub fn add_node(&mut self, node: GraphNode) -> NodeId {
        node.kind.build(&node, self);
        self.nodes.insert(node)
    }

    pub fn node(&self, node_id: NodeId) -> Option<&GraphNode> {
        self.nodes.get(node_id)
    }

    pub fn node_mut(&mut self, node_id: NodeId) -> Option<&mut GraphNode> {
        self.nodes.get_mut(node_id)
    }

    pub fn nodes(&self) -> &SlotMap<NodeId, GraphNode> {
        &self.nodes
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

    pub fn inputs(&self) -> &SlotMap<InputId, Input> {
        &self.inputs
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

    pub fn outputs(&self) -> &SlotMap<OutputId, Output> {
        &self.outputs
    }

    pub fn add_connection(&mut self, output_id: OutputId, input_id: InputId) {
        self.connections.insert(input_id, output_id);
    }

    pub fn connection(&self, input_id: InputId) -> Option<&OutputId> {
        self.connections.get(input_id)
    }

    pub fn connections(&self) -> &SecondaryMap<InputId, OutputId> {
        &self.connections
    }

    pub fn output_parent_node(&self, output_id: OutputId) -> NodeId {
        // FIXME: This is very ad-hoc. We should probably just store this in a secondary map?
        for (node_id, node) in &self.nodes {
            if node.outputs.borrow().iter().any(|(_, id)| *id == output_id) {
                return node_id;
            }
        }
        unreachable!()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphNode {
    kind: NodeKind,
    inputs: RefCell<HashMap<String, InputId>>,
    outputs: RefCell<HashMap<String, OutputId>>,
    x: f32,
    y: f32,
}

impl GraphNode {
    pub fn new(kind: NodeKind, x: f32, y: f32) -> Self {
        Self {
            kind,
            inputs: RefCell::new(HashMap::new()),
            outputs: RefCell::new(HashMap::new()),
            x,
            y,
        }
    }

    pub fn kind(&self) -> &NodeKind {
        &self.kind
    }

    pub fn add_input(
        &self,
        id: String,
        graph: &mut Graph,
        label: String,
        kind: InputKind,
        value_type: ValueType,
    ) {
        let input_id = graph.add_input(|input_id| Input::new(input_id, label, kind, value_type));
        self.inputs.borrow_mut().insert(id, input_id);
    }

    pub fn input(&self, id: &str) -> InputId {
        *self
            .inputs
            .borrow()
            .get(id)
            .expect("Input id did not match any input created for this node.")
    }

    // FIXME: May be not the best thing to create a new vec every time this is called.
    pub fn inputs(&self) -> Vec<InputId> {
        self.inputs.borrow().values().copied().collect()
    }

    pub fn add_output(
        &self,
        id: String,
        graph: &mut Graph,
        label: String,
        kind: OutputKind,
        value_type: ValueType,
    ) {
        let output_id =
            graph.add_output(|output_id| Output::new(output_id, label, kind, value_type));
        self.outputs.borrow_mut().insert(id, output_id);
    }

    pub fn output(&self, id: &str) -> OutputId {
        *self
            .outputs
            .borrow()
            .get(id)
            .expect("Output id did not match any output created for this node.")
    }

    // FIXME: May be not the best thing to create a new vec every time this is called.
    pub fn outputs(&self) -> Vec<OutputId> {
        self.outputs.borrow().values().copied().collect()
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn process(&self, graph: &Graph, state: &mut GraphState) -> Option<GraphValue> {
        self.kind.process(&self, graph, state)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Input {
    id: InputId,
    label: String,
    kind: InputKind,
    value_type: ValueType,
}

impl Input {
    pub fn new(id: InputId, label: String, kind: InputKind, value_type: ValueType) -> Self {
        Self {
            id,
            label,
            kind,
            value_type,
        }
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

    pub fn value_type(&self) -> &ValueType {
        &self.value_type
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum InputKind {
    ConnectionOnly,
    ConstantOnly,
    ConnectionOrConstant,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Output {
    id: OutputId,
    label: String,
    kind: OutputKind,
    value_type: ValueType,
}

impl Output {
    pub fn new(id: OutputId, label: String, kind: OutputKind, value_type: ValueType) -> Self {
        Self {
            id,
            label,
            kind,
            value_type,
        }
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

    pub fn value_type(&self) -> &ValueType {
        &self.value_type
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OutputKind {
    CalculatedOnly,
    ConstantOnly(OutputControl),
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OutputControl {
    Slider { min: f32, max: f32 },
    NumberInput,
}
