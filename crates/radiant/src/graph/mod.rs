use slotmap::{SecondaryMap, SlotMap};

pub mod effect_graph;

slotmap::new_key_type! {
    pub struct NodeId;
    pub struct InputId;
    pub struct OutputId;
}

#[derive(Debug, Clone)]
pub struct Graph<N, D, V>
where
    N: NodeType,
    V: Value,
{
    nodes: SlotMap<NodeId, Node<N>>,
    inputs: SlotMap<InputId, Input<D, V>>,
    outputs: SlotMap<OutputId, Output<D, V>>,
    connections: SecondaryMap<InputId, OutputId>,
}

impl<N, D, V> Graph<N, D, V>
where
    N: NodeType<DataType = D, Value = V>,
    V: Value<DataType = D>,
{
    pub fn new() -> Self {
        Self {
            nodes: SlotMap::with_key(),
            inputs: SlotMap::with_key(),
            outputs: SlotMap::with_key(),
            connections: SecondaryMap::new(),
        }
    }

    pub fn add_node(&mut self, node_type: N) -> NodeId {
        let id = self.nodes.insert_with_key(|id| Node {
            node_type: node_type.clone(),
            id,
            inputs: Vec::new(),
            outputs: Vec::new(),
        });

        node_type.build_node(self, id);

        id
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node<N>> {
        self.nodes.values()
    }

    pub fn get_node(&self, id: NodeId) -> Option<&Node<N>> {
        self.nodes.get(id)
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node<N>> {
        self.nodes.get_mut(id)
    }

    pub fn add_input(&mut self, node: NodeId, label: String, data_type: D) -> InputId {
        let id = self.inputs.insert_with_key(|id| Input {
            id,
            node,
            value: V::initial_value(&data_type),
            data_type,
        });
        self.nodes[node].inputs.push((label, id));
        id
    }

    pub fn inputs(&self) -> impl Iterator<Item = &Input<D, V>> {
        self.inputs.values()
    }

    pub fn get_input(&self, id: InputId) -> Option<&Input<D, V>> {
        self.inputs.get(id)
    }

    pub fn get_input_mut(&mut self, id: InputId) -> Option<&mut Input<D, V>> {
        self.inputs.get_mut(id)
    }

    pub fn add_output(&mut self, node: NodeId, label: String, data_type: D) -> OutputId {
        let id = self.outputs.insert_with_key(|id| Output {
            id,
            node,
            value: V::initial_value(&data_type),
            data_type,
        });
        self.nodes[node].outputs.push((label, id));
        id
    }

    pub fn outputs(&self) -> impl Iterator<Item = &Output<D, V>> {
        self.outputs.values()
    }

    pub fn get_output(&self, id: OutputId) -> Option<&Output<D, V>> {
        self.outputs.get(id)
    }

    pub fn get_output_mut(&mut self, id: OutputId) -> Option<&mut Output<D, V>> {
        self.outputs.get_mut(id)
    }

    pub fn get_connected_output(&self, input: InputId) -> Option<&OutputId> {
        self.connections.get(input)
    }

    pub fn add_connection(&mut self, source: OutputId, target: InputId) {
        self.connections.insert(target, source);
    }

    pub fn process(&mut self, node: NodeId) {
        self.nodes[node].node_type.clone().process(self, node);
    }
}

pub trait NodeType: Clone {
    type DataType;
    type Value: Value;

    fn build_node(&self, editor: &mut Graph<Self, Self::DataType, Self::Value>, node_id: NodeId);

    fn process(&self, editor: &mut Graph<Self, Self::DataType, Self::Value>, node_id: NodeId);
}

pub trait Value {
    type DataType;

    fn initial_value(data_type: &Self::DataType) -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node<N: NodeType> {
    id: NodeId,
    inputs: Vec<(String, InputId)>,
    outputs: Vec<(String, OutputId)>,
    node_type: N,
}

impl<N> Node<N>
where
    N: NodeType,
{
    pub fn inputs(&self) -> &[(String, InputId)] {
        &self.inputs
    }

    pub fn outputs(&self) -> &[(String, OutputId)] {
        &self.outputs
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Input<D, V> {
    id: InputId,
    node: NodeId,
    data_type: D,
    value: V,
}

impl<D, V> Input<D, V> {
    pub fn id(&self) -> InputId {
        self.id
    }

    pub fn node(&self) -> NodeId {
        self.node
    }

    pub fn data_type(&self) -> &D {
        &self.data_type
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn set_value(&mut self, value: V) {
        self.value = value;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Output<D, V> {
    id: OutputId,
    node: NodeId,
    data_type: D,
    value: V,
}

impl<D, V> Output<D, V> {
    pub fn id(&self) -> OutputId {
        self.id
    }

    pub fn node(&self) -> NodeId {
        self.node
    }

    pub fn data_type(&self) -> &D {
        &self.data_type
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn set_value(&mut self, value: V) {
        self.value = value;
    }
}
