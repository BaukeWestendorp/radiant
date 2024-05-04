//! # Graph
//!
//! A graph data structure that represents a node graph.
//! Nodes are connected by inputs and outputs.
//! After creating a graph, you can process a node and get a computed value.

use slotmap::{SecondaryMap, SlotMap};

slotmap::new_key_type! {
    /// An id that references a graph node.
    pub struct NodeId;
    /// An id that references a node input.
    pub struct InputId;
    /// An id that references a node output.
    pub struct OutputId;
}

/// Represents all possible data types in the graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DataType {
    /// An integer value.
    Integer,
}
impl DataType {
    /// Get the display color of the data type as a hexadecimal value.
    pub fn hex_color(&self) -> u32 {
        match self {
            Self::Integer => 0x4070fb,
        }
    }
}

/// Represents all possible nodes in the graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum NodeType {
    /// Creates a new integer value.
    IntegerNew,
    /// Adds two integer values.
    IntegerAdd,
}
impl NodeType {
    /// Process a node. This will update the value of the outputs of the node.
    pub fn process(&self, graph: &mut Graph, node_id: NodeId) {
        let node = graph.get_node(node_id).unwrap();

        match self {
            Self::IntegerNew => {}
            Self::IntegerAdd => {
                let inputs = node.inputs();

                let a_id = graph
                    .get_connected_output(inputs.get(0).unwrap().1)
                    .unwrap();
                let a = graph.get_output(*a_id).unwrap();

                let b_id = graph
                    .get_connected_output(inputs.get(1).unwrap().1)
                    .unwrap();
                let b = graph.get_output(*b_id).unwrap();

                let sum = match (a.value(), b.value()) {
                    (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
                };

                graph
                    .get_output_mut(node.outputs().get(0).unwrap().1)
                    .unwrap()
                    .set_value(sum);
            }
        }
    }

    /// The label to display for the node type.
    pub fn label(&self) -> String {
        match self {
            Self::IntegerNew => "New Integer".to_string(),
            Self::IntegerAdd => "Add Integers".to_string(),
        }
    }

    fn build_node(&self, graph: &mut Graph, node_id: NodeId) {
        match self {
            Self::IntegerNew => {
                graph.add_output(node_id, "Value".to_string(), DataType::Integer);
            }
            Self::IntegerAdd => {
                graph.add_input(node_id, "A".to_string(), DataType::Integer);
                graph.add_input(node_id, "B".to_string(), DataType::Integer);
                graph.add_output(node_id, "Sum".to_string(), DataType::Integer);
            }
        }
    }
}

/// Represents a value in the graph.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Value {
    /// An integer value.
    Integer(i32),
}

/// A node graph.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Graph {
    id: usize,
    nodes: SlotMap<NodeId, Node>,
    inputs: SlotMap<InputId, Input>,
    outputs: SlotMap<OutputId, Output>,
    connections: SecondaryMap<InputId, OutputId>,
}

impl Graph {
    /// Creates a new graph with the given id.
    pub fn new(id: usize) -> Self {
        Self {
            id,
            nodes: SlotMap::with_key(),
            inputs: SlotMap::with_key(),
            outputs: SlotMap::with_key(),
            connections: SecondaryMap::new(),
        }
    }

    /// Get the id of the graph.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Add a node to the graph.
    pub fn add_node(&mut self, node_type: NodeType, x: f32, y: f32) -> NodeId {
        let id = self.nodes.insert_with_key(|id| Node {
            node_type: node_type.clone(),
            id,
            inputs: Vec::new(),
            outputs: Vec::new(),
            x,
            y,
        });

        node_type.build_node(self, id);

        id
    }

    /// Get all nodes in the graph.
    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    /// Get a node by id.
    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    /// Get a mutable node by id.
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    /// Add an input to a node.
    pub fn add_input(&mut self, node: NodeId, label: String, data_type: DataType) -> InputId {
        let id = self.inputs.insert_with_key(|id| Input {
            id,
            node,
            value: Value::initial_value(&data_type),
            data_type,
        });
        self.nodes[node].inputs.push((label, id));
        id
    }

    /// Get an input by id.
    pub fn get_input(&self, id: InputId) -> Option<&Input> {
        self.inputs.get(id)
    }

    /// Get a mutable input by id.
    pub fn get_input_mut(&mut self, id: InputId) -> Option<&mut Input> {
        self.inputs.get_mut(id)
    }

    /// Add an output to a node.
    pub fn add_output(&mut self, node: NodeId, label: String, data_type: DataType) -> OutputId {
        let id = self.outputs.insert_with_key(|id| Output {
            id,
            node,
            value: Value::initial_value(&data_type),
            data_type,
        });
        self.nodes[node].outputs.push((label, id));
        id
    }

    /// Get an output by id.
    pub fn get_output(&self, id: OutputId) -> Option<&Output> {
        self.outputs.get(id)
    }

    /// Get a mutable output by id.
    pub fn get_output_mut(&mut self, id: OutputId) -> Option<&mut Output> {
        self.outputs.get_mut(id)
    }

    /// Get the connected output of a input, if it has a connection to one.
    pub fn get_connected_output(&self, input: InputId) -> Option<&OutputId> {
        self.connections.get(input)
    }

    /// Get all outputs in the graph.
    pub fn add_connection(&mut self, source: OutputId, target: InputId) {
        self.connections.insert(target, source);
    }

    /// Process a node.
    pub fn process(&mut self, node: NodeId) {
        self.nodes[node].node_type.clone().process(self, node);
    }
}

impl Value {
    fn initial_value(data_type: &DataType) -> Self {
        match data_type {
            DataType::Integer => Self::Integer(0),
        }
    }
}

/// Represents a node in the graph.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Node {
    id: NodeId,
    inputs: Vec<(String, InputId)>,
    outputs: Vec<(String, OutputId)>,
    node_type: NodeType,
    x: f32,
    y: f32,
}

impl Node {
    /// The id of the node.
    pub fn id(&self) -> NodeId {
        self.id
    }

    /// The inputs of the node.
    pub fn inputs(&self) -> &[(String, InputId)] {
        &self.inputs
    }

    /// The outputs of the node.
    pub fn outputs(&self) -> &[(String, OutputId)] {
        &self.outputs
    }

    /// The type of the node.
    pub fn node_type(&self) -> NodeType {
        self.node_type
    }

    /// The x position of the node.
    pub fn x(&self) -> f32 {
        self.x
    }

    /// The y position of the node.
    pub fn y(&self) -> f32 {
        self.y
    }

    /// Set the position of the node.
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

/// Represents an input in the graph.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Input {
    id: InputId,
    node: NodeId,
    data_type: DataType,
    value: Value,
}

impl Input {
    /// The id of the input.
    pub fn id(&self) -> InputId {
        self.id
    }

    /// The node the input belongs to.
    pub fn node(&self) -> NodeId {
        self.node
    }

    /// The data type of the input.
    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }

    /// The value of the input.
    pub fn value(&self) -> &Value {
        &self.value
    }

    /// Set the value of the input.
    pub fn set_value(&mut self, value: Value) {
        self.value = value;
    }
}

/// Represents an output in the graph.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Output {
    id: OutputId,
    node: NodeId,
    data_type: DataType,
    value: Value,
}

impl Output {
    /// The id of the output.
    pub fn id(&self) -> OutputId {
        self.id
    }

    /// The node the output belongs to.
    pub fn node(&self) -> NodeId {
        self.node
    }

    /// The data type of the output.
    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }

    /// The value of the output.
    pub fn value(&self) -> &Value {
        &self.value
    }

    /// Set the value of the output.
    pub fn set_value(&mut self, value: Value) {
        self.value = value;
    }
}
