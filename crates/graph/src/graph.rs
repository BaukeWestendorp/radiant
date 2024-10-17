use super::error::GraphError;
use super::node::{Input, InputValue, Node, Output, OutputValue};
use crate::{DataType, NodeKind, ProcessingContext, Value};
use gpui::*;
use slotmap::{SecondaryMap, SlotMap};
use std::collections::HashMap;

slotmap::new_key_type! {
    pub struct NodeId;
    pub struct InputId;
    pub struct OutputId;
}

#[derive(Debug, Clone, Default)]
pub struct Graph {
    pub nodes: SlotMap<NodeId, Node>,
    pub inputs: SlotMap<InputId, Input>,
    pub outputs: SlotMap<OutputId, Output>,
    pub connections: SecondaryMap<InputId, OutputId>,

    graph_ends: Vec<NodeId>,
}

impl Graph {
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

    pub fn add_node(&mut self, kind: NodeKind, x: f32, y: f32) -> NodeId {
        let node_id = self.nodes.insert_with_key(|node_id| Node {
            id: node_id,
            kind: kind.clone(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            position: point(px(x), px(y)),
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

        self.remove_graph_end(&node_id);

        self.nodes.remove(node_id).expect("Node should exist");
    }

    pub fn node(&self, node_id: NodeId) -> &Node {
        &self.nodes[node_id]
    }

    pub fn node_mut(&mut self, node_id: NodeId) -> &mut Node {
        &mut self.nodes[node_id]
    }

    pub fn add_input(
        &mut self,
        node_id: NodeId,
        label: String,
        data_type: DataType,
        constant_value: InputValue,
    ) -> InputId {
        let input_id = self.inputs.insert_with_key(|input_id| Input {
            id: input_id,
            data_type,
            node: node_id,
            value: constant_value,
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

    pub fn input(&self, input_id: InputId) -> &Input {
        &self.inputs[input_id]
    }

    pub fn input_mut(&mut self, input_id: InputId) -> &mut Input {
        &mut self.inputs[input_id]
    }

    pub fn add_output(
        &mut self,
        node_id: NodeId,
        label: String,
        data_type: DataType,
        value: OutputValue,
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

    pub fn output(&self, output_id: OutputId) -> &Output {
        &self.outputs[output_id]
    }

    pub fn output_mut(&mut self, output_id: OutputId) -> &mut Output {
        &mut self.outputs[output_id]
    }

    pub fn get_output_value<'a>(
        &self,
        output_id: &OutputId,
        context: &mut ProcessingContext,
    ) -> Result<Value, GraphError> {
        let output = self.output(*output_id);
        match &output.value {
            OutputValue::Computed => {
                let node = self.node(output.node);
                let result = node.process(context, self)?;
                Ok(result
                    .get(&output_id)
                    .expect("An output value should always be generated after processing")
                    .clone())
            }
            OutputValue::Constant { value, .. } => Ok(value.clone()),
        }
    }

    pub fn add_connection(&mut self, target_id: InputId, source_id: OutputId) {
        if !self.check_connection_validity(target_id, source_id) {
            return;
        }

        self.connections.insert(target_id, source_id);

        let source_node = self.output(source_id).node;
        self.remove_graph_end(&source_node);
    }

    pub fn remove_connection(&mut self, target_id: InputId) {
        self.connections.remove(target_id);
    }

    pub fn connection_source(&self, target_id: InputId) -> Option<OutputId> {
        self.connections.get(target_id).copied()
    }

    pub fn connection_target(&self, source_id: OutputId) -> Option<InputId> {
        self.connections.iter().find_map(|(target_id, source)| {
            if *source == source_id {
                Some(target_id)
            } else {
                None
            }
        })
    }

    pub fn check_connection_validity(&self, target_id: InputId, source_id: OutputId) -> bool {
        let target_data_type = &self.input(target_id).data_type;
        let source_data_type = &self.output(source_id).data_type;
        source_data_type
            .default_value()
            .try_cast_to(&target_data_type)
            .is_ok()
    }

    pub fn process(&self, context: &mut ProcessingContext) -> Result<(), GraphError> {
        for node_id in &self.graph_ends {
            let node = self.node(*node_id);
            node.kind.process(*node_id, context, self)?;
        }

        Ok(())
    }

    fn remove_graph_end(&mut self, node_id: &NodeId) {
        self.graph_ends.retain(|id| id != node_id);
    }
}

impl EventEmitter<GraphEvent> for Graph {}

#[derive(Debug, Clone)]
pub enum GraphEvent {
    AddNode(NodeKind, Point<Pixels>),
}

pub type ProcessingResult = HashMap<OutputId, Value>;
