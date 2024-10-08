use gpui::*;
use slotmap::{SecondaryMap, SlotMap};

use crate::{FlowError, Input, InputId, Node, NodeId, NodeKind, Output, OutputId, OutputValue};

#[derive(Debug, Clone, Default)]
pub struct Graph<D, V, N>
where
    N: NodeKind<DataType = D, Value = V>,
{
    pub nodes: SlotMap<NodeId, Node<D, V, N>>,
    pub inputs: SlotMap<InputId, Input<D, V>>,
    pub outputs: SlotMap<OutputId, Output<D, V>>,
    pub connections: SecondaryMap<InputId, OutputId>,

    graph_ends: Vec<NodeId>,
}

impl<D, V, N> Graph<D, V, N>
where
    N: NodeKind<DataType = D, Value = V>,
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

    pub fn add_node(&mut self, kind: N, position: Point<Pixels>) -> NodeId {
        let node_id = self.nodes.insert_with_key(|node_id| Node {
            id: node_id,
            kind: kind.clone(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            position,
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

    pub fn node(&self, node_id: NodeId) -> &Node<D, V, N> {
        &self.nodes[node_id]
    }

    pub fn node_mut(&mut self, node_id: NodeId) -> &mut Node<D, V, N> {
        &mut self.nodes[node_id]
    }

    pub fn add_input(&mut self, node_id: NodeId, label: String, data_type: D, value: V) -> InputId {
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

    pub fn input(&self, input_id: InputId) -> &Input<D, V> {
        &self.inputs[input_id]
    }

    pub fn add_output(
        &mut self,
        node_id: NodeId,
        label: String,
        data_type: D,
        value: OutputValue<V>,
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

    pub fn output(&self, output_id: OutputId) -> &Output<D, V> {
        &self.outputs[output_id]
    }

    pub fn output_mut(&mut self, output_id: OutputId) -> &mut Output<D, V> {
        &mut self.outputs[output_id]
    }

    pub fn get_output_value<'a>(
        &'a self,
        output_id: OutputId,
        context: &mut N::ProcessingContext,
        cache: &'a mut GraphProcessingCache<V>,
    ) -> Result<&V, FlowError> {
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
        context: &mut N::ProcessingContext,
        cache: &mut GraphProcessingCache<V>,
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
pub struct GraphProcessingCache<V> {
    output_value_cache: SecondaryMap<OutputId, V>,
}

impl<V> GraphProcessingCache<V> {
    pub fn get_output_value(&self, output_id: OutputId) -> Result<&V, FlowError> {
        match self.output_value_cache.get(output_id) {
            Some(value) => Ok(value),
            None => Err(FlowError::NoCachedOutputValueFor(output_id)),
        }
    }

    pub fn get_output_value_mut(&mut self, output_id: OutputId) -> Result<&mut V, FlowError> {
        match self.output_value_cache.get_mut(output_id) {
            Some(value) => Ok(value),
            None => Err(FlowError::NoCachedOutputValueFor(output_id)),
        }
    }

    pub fn set_output_value(&mut self, output_id: OutputId, value: V) {
        self.output_value_cache.insert(output_id, value);
    }
}

impl<V> Default for GraphProcessingCache<V> {
    fn default() -> Self {
        Self {
            output_value_cache: SecondaryMap::default(),
        }
    }
}
