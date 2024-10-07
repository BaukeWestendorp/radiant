use slotmap::{SecondaryMap, SlotMap};

use crate::FlowError;

slotmap::new_key_type! {
    pub struct NodeId;
    pub struct InputId;
    pub struct OutputId;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Node<DataType, Value, NodeKind>
where
    NodeKind: GraphNodeKind<DataType = DataType, Value = Value>,
{
    pub id: NodeId,
    pub kind: NodeKind,
    pub inputs: Vec<(String, InputId)>,
    pub outputs: Vec<(String, OutputId)>,
}

impl<DataType, Value, NodeKind> Node<DataType, Value, NodeKind>
where
    NodeKind: GraphNodeKind<DataType = DataType, Value = Value>,
{
    pub fn inputs<'a>(
        &'a self,
        graph: &'a Graph<DataType, Value, NodeKind>,
    ) -> impl Iterator<Item = &Input<DataType, Value>> + 'a {
        self.input_ids().map(|id| graph.input(id))
    }

    pub fn outputs<'a>(
        &'a self,
        graph: &'a Graph<DataType, Value, NodeKind>,
    ) -> impl Iterator<Item = &Output<DataType, Value>> + 'a {
        self.output_ids().map(|id| graph.output(id))
    }

    pub fn input_ids(&self) -> impl Iterator<Item = InputId> + '_ {
        self.inputs.iter().map(|(_name, id)| *id)
    }

    pub fn output_ids(&self) -> impl Iterator<Item = OutputId> + '_ {
        self.outputs.iter().map(|(_name, id)| *id)
    }

    pub fn input(&self, name: &str) -> Result<InputId, FlowError> {
        self.inputs
            .iter()
            .find(|(param_name, _id)| param_name == name)
            .map(|x| x.1)
            .ok_or_else(|| FlowError::NoSocketNamed(self.id, name.into()))
    }

    pub fn output(&self, name: &str) -> Result<OutputId, FlowError> {
        self.outputs
            .iter()
            .find(|(param_name, _id)| param_name == name)
            .map(|x| x.1)
            .ok_or_else(|| FlowError::NoSocketNamed(self.id, name.into()))
    }

    pub fn process(
        &self,
        context: &mut NodeKind::ProcessingContext,
        graph: &Graph<DataType, Value, NodeKind>,
        cache: &mut GraphProcessingCache<Value>,
    ) -> Result<(), FlowError> {
        self.kind.process(self.id, context, graph, cache)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Input<DataType, Value> {
    pub id: InputId,
    pub data_type: DataType,
    pub value: Value,
    pub node: NodeId,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Output<DataType, Value> {
    pub id: OutputId,
    pub node: NodeId,
    pub data_type: DataType,
    pub value: OutputValue<Value>,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum OutputValue<Value> {
    Computed,
    Constant(Value),
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Graph<DataType, Value, NodeKind>
where
    NodeKind: GraphNodeKind<DataType = DataType, Value = Value>,
{
    pub nodes: SlotMap<NodeId, Node<DataType, Value, NodeKind>>,
    pub inputs: SlotMap<InputId, Input<DataType, Value>>,
    pub outputs: SlotMap<OutputId, Output<DataType, Value>>,
    pub connections: SecondaryMap<InputId, OutputId>,

    graph_ends: Vec<NodeId>,
}

impl<DataType, Value, NodeKind> Graph<DataType, Value, NodeKind>
where
    NodeKind: GraphNodeKind<DataType = DataType, Value = Value>,
{
    pub fn new() -> Self {
        Self {
            nodes: SlotMap::default(),
            inputs: SlotMap::default(),
            outputs: SlotMap::default(),
            connections: SecondaryMap::default(),
            graph_ends: Vec::new(),
        }
    }

    pub fn node_ids(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.nodes.keys()
    }

    pub fn add_node(&mut self, kind: NodeKind) -> NodeId {
        let node_id = self.nodes.insert_with_key(|node_id| Node {
            id: node_id,
            kind: kind.clone(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        });

        self.graph_ends.push(node_id);

        kind.build(self, node_id);

        node_id
    }

    pub fn remove_node(&mut self, node_id: NodeId) {
        self.connections.retain(|target_id, source_id| {
            self.outputs[*source_id].node == node_id || self.inputs[target_id].node == node_id
        });

        for input in self.nodes[node_id].input_ids().collect::<Vec<_>>() {
            self.inputs.remove(input);
        }

        for output in self.nodes[node_id].output_ids().collect::<Vec<_>>() {
            self.outputs.remove(output);
        }

        self.remove_graph_end(node_id);

        self.nodes.remove(node_id).expect("Node should exist");
    }

    pub fn node(&self, node_id: NodeId) -> &Node<DataType, Value, NodeKind> {
        &self.nodes[node_id]
    }

    pub fn add_input(
        &mut self,
        node_id: NodeId,
        label: String,
        data_type: DataType,
        value: Value,
    ) -> InputId {
        let input_id = self.inputs.insert_with_key(|input_id| Input {
            id: input_id,
            data_type,
            value,
            node: node_id,
        });

        self.nodes[node_id].inputs.push((label, input_id));

        input_id
    }

    pub fn remove_input(&mut self, input_id: InputId) {
        let node = self.inputs[input_id].node;
        self.nodes[node].inputs.retain(|(_, id)| *id != input_id);
        self.inputs.remove(input_id);
        self.connections
            .retain(|target_id, _| target_id != input_id)
    }

    pub fn input(&self, input_id: InputId) -> &Input<DataType, Value> {
        &self.inputs[input_id]
    }

    pub fn add_output(
        &mut self,
        node_id: NodeId,
        label: String,
        data_type: DataType,
        value: OutputValue<Value>,
    ) -> OutputId {
        let output_id = self.outputs.insert_with_key(|output_id| Output {
            id: output_id,
            node: node_id,
            value,
            data_type,
        });

        self.nodes[node_id].outputs.push((label, output_id));

        output_id
    }

    pub fn remove_output(&mut self, output_id: OutputId) {
        let node = self.outputs[output_id].node;
        self.nodes[node].outputs.retain(|(_, id)| *id != output_id);
        self.outputs.remove(output_id);
        self.connections
            .retain(|_, source_id| *source_id != output_id);
    }

    pub fn output(&self, output_id: OutputId) -> &Output<DataType, Value> {
        &self.outputs[output_id]
    }

    pub fn output_mut(&mut self, output_id: OutputId) -> &mut Output<DataType, Value> {
        &mut self.outputs[output_id]
    }

    pub fn get_output_value<'a>(
        &'a self,
        output_id: OutputId,
        context: &mut NodeKind::ProcessingContext,
        cache: &'a mut GraphProcessingCache<Value>,
    ) -> Result<&Value, FlowError> {
        let output = self.output(output_id);
        match &output.value {
            OutputValue::Computed => {
                let node = self.node(output.node);
                node.process(context, self, cache)?;
                let value = cache
                    .get_output_value(output_id)
                    .expect("A calculated value should always have a value");
                Ok(value)
            }
            OutputValue::Constant(value) => Ok(value),
        }
    }

    pub fn add_connection(&mut self, target_id: InputId, source_id: OutputId) {
        self.connections.insert(target_id, source_id);

        let source_node = self.output(source_id).node;
        self.remove_graph_end(source_node);
    }

    pub fn remove_connection(&mut self, target_id: InputId) {
        self.connections.remove(target_id);
    }

    pub fn connection(&self, target_id: InputId) -> Option<OutputId> {
        self.connections.get(target_id).copied()
    }

    pub fn process(
        &self,
        context: &mut NodeKind::ProcessingContext,
        cache: &mut GraphProcessingCache<Value>,
    ) -> Result<(), FlowError> {
        for node_id in &self.graph_ends {
            let node = self.node(*node_id);
            node.kind.process(*node_id, context, self, cache)?;
        }

        Ok(())
    }

    fn remove_graph_end(&mut self, node_id: NodeId) {
        self.graph_ends.retain(|id| *id != node_id);
    }
}

#[derive(Debug, Clone)]
pub struct GraphProcessingCache<Value> {
    output_value_cache: SecondaryMap<OutputId, Value>,
}

impl<Value> Default for GraphProcessingCache<Value> {
    fn default() -> Self {
        Self {
            output_value_cache: SecondaryMap::default(),
        }
    }
}

impl<Value> GraphProcessingCache<Value> {
    pub fn get_output_value(&self, output_id: OutputId) -> Result<&Value, FlowError> {
        match self.output_value_cache.get(output_id) {
            Some(value) => Ok(value),
            None => Err(FlowError::NoCachedOutputValueFor(output_id)),
        }
    }

    pub fn get_output_value_mut(&mut self, output_id: OutputId) -> Result<&mut Value, FlowError> {
        match self.output_value_cache.get_mut(output_id) {
            Some(value) => Ok(value),
            None => Err(FlowError::NoCachedOutputValueFor(output_id)),
        }
    }

    pub fn set_output_value(&mut self, output_id: OutputId, value: Value) {
        self.output_value_cache.insert(output_id, value);
    }
}

pub trait GraphNodeKind: Clone {
    type DataType;
    type Value;
    type ProcessingContext;

    fn build(&self, graph: &mut Graph<Self::DataType, Self::Value, Self>, node_id: NodeId)
    where
        Self: Sized;

    fn process(
        &self,
        node_id: NodeId,
        context: &mut Self::ProcessingContext,
        graph: &Graph<Self::DataType, Self::Value, Self>,
        cache: &mut GraphProcessingCache<Self::Value>,
    ) -> Result<(), FlowError>
    where
        Self: Sized;
}

pub trait GraphValue {
    type DataType;
    fn try_cast_to(self, target_type: &Self::DataType) -> Result<Self, FlowError>
    where
        Self: Sized;
}
