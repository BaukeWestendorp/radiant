use crate::error::GraphError;
use crate::graph_def;
use crate::graph_def::{DataType, NodeKind, Value};
use crate::node::{Node, NodeInputParameter, NodeOutputParameter};
use graph_def::GraphDefinition;
use slotmap::{SecondaryMap, SlotMap};

#[derive(Clone)]
pub struct Graph<Def: GraphDefinition> {
    nodes: SlotMap<crate::NodeId, Node<Def>>,
    input_parameters: SlotMap<crate::InputId, crate::Input<Def>>,
    output_parameters: SlotMap<crate::OutputId, crate::Output<Def>>,
    edges: SecondaryMap<crate::InputId, crate::OutputId>,

    graph_ends: Vec<crate::NodeId>,
}

impl<Def: GraphDefinition> Graph<Def> {
    pub fn new() -> Self {
        Self {
            nodes: SlotMap::default(),
            input_parameters: SlotMap::default(),
            output_parameters: SlotMap::default(),
            edges: SecondaryMap::default(),
            graph_ends: Vec::new(),
        }
    }

    pub fn node(&self, node_id: crate::NodeId) -> &Node<Def> {
        &self.nodes[node_id]
    }

    pub fn node_mut(&mut self, node_id: crate::NodeId) -> &mut Node<Def> {
        &mut self.nodes[node_id]
    }

    pub fn node_ids(&self) -> impl Iterator<Item = crate::NodeId> + '_ {
        self.nodes.keys()
    }

    pub fn add_node(&mut self, kind: Def::NodeKind, data: Def::NodeData) -> crate::NodeId {
        let node_id = self
            .nodes
            .insert_with_key(|id| Node::new(id, kind.clone(), data));

        self.graph_ends.push(node_id);

        kind.build(self, node_id);

        node_id
    }

    pub fn remove_node(&mut self, node_id: crate::NodeId) {
        self.edges.retain(|target_id, source_id| {
            self.output_parameters[*source_id].node_id == node_id
                || self.input_parameters[target_id].node_id == node_id
        });

        for input in self.nodes[node_id].input_ids().collect::<Vec<_>>() {
            self.input_parameters.remove(input);
        }

        for output in self.nodes[node_id].output_ids().collect::<Vec<_>>() {
            self.output_parameters.remove(output);
        }

        self.remove_graph_end(&node_id);

        self.nodes.remove(node_id).expect("Node should exist");
    }

    pub fn input(&self, input_id: crate::InputId) -> &crate::Input<Def> {
        &self.input_parameters[input_id]
    }

    pub fn input_mut(&mut self, input_id: crate::InputId) -> &mut crate::Input<Def> {
        &mut self.input_parameters[input_id]
    }

    pub fn input_ids(&self) -> impl Iterator<Item = crate::InputId> + '_ {
        self.input_parameters.keys()
    }

    pub fn add_input(
        &mut self,
        node_id: crate::NodeId,
        label: String,
        data_type: Def::DataType,
        kind: crate::InputParameterKind<Def>,
    ) -> crate::InputId {
        let parameter_id = self.input_parameters.insert_with_key(|id| crate::Input {
            id,
            node_id,
            data_type,
            kind,
        });

        self.nodes[node_id].inputs.push(NodeInputParameter {
            label,
            id: parameter_id,
        });

        parameter_id
    }

    pub fn remove_input(&mut self, input_id: crate::InputId) {
        let node = self.input_parameters[input_id].node_id;
        self.nodes[node].inputs.retain(|param| param.id != input_id);
        self.input_parameters.remove(input_id);
        self.edges.retain(|target_id, _| target_id != input_id)
    }

    pub fn output(&self, output_id: crate::OutputId) -> &crate::Output<Def> {
        &self.output_parameters[output_id]
    }

    pub fn output_mut(&mut self, output_id: crate::OutputId) -> &mut crate::Output<Def> {
        &mut self.output_parameters[output_id]
    }

    pub fn output_ids(&self) -> impl Iterator<Item = crate::OutputId> + '_ {
        self.output_parameters.keys()
    }

    pub fn add_output(
        &mut self,
        node_id: crate::NodeId,
        label: String,
        data_type: Def::DataType,
        kind: crate::OutputParameterKind<Def>,
    ) -> crate::OutputId {
        let parameter_id = self.output_parameters.insert_with_key(|id| crate::Output {
            id,
            node_id,
            data_type,
            kind,
        });

        self.nodes[node_id].outputs.push(NodeOutputParameter {
            label,
            id: parameter_id,
        });

        parameter_id
    }

    pub fn remove_output(&mut self, output_id: crate::OutputId) {
        let node = self.output_parameters[output_id].node_id;
        self.nodes[node]
            .outputs
            .retain(|param| param.id != output_id);
        self.output_parameters.remove(output_id);
        self.edges.retain(|_, source_id| *source_id != output_id);
    }

    pub fn edge_source(&self, target_id: crate::InputId) -> Option<crate::OutputId> {
        self.edges.get(target_id).copied()
    }

    pub fn edge_target(&self, source_id: crate::OutputId) -> Option<crate::InputId> {
        self.edges.iter().find_map(|(target_id, source)| {
            if *source == source_id {
                Some(target_id)
            } else {
                None
            }
        })
    }

    pub fn edges(&self) -> impl Iterator<Item = (crate::InputId, &crate::OutputId)> {
        self.edges.iter()
    }

    pub fn add_edge(&mut self, source_id: crate::OutputId, target_id: crate::InputId) {
        if !self.check_edge_validity(source_id, target_id) {
            return;
        }

        self.edges.insert(target_id, source_id);

        let source_node = self.output(source_id).node_id;
        self.remove_graph_end(&source_node);
    }

    pub fn remove_edge(&mut self, target_id: crate::InputId) {
        self.edges.remove(target_id);
    }

    pub fn process(
        &mut self,
        context: &mut <Def::NodeKind as NodeKind<Def>>::ProcessingContext,
    ) -> Result<(), GraphError> {
        for node_id in &self.graph_ends {
            let node = self.node(*node_id);
            node.kind.process(*node_id, context, self)?;
        }

        Ok(())
    }

    pub fn check_edge_validity(
        &self,
        source_id: crate::OutputId,
        target_id: crate::InputId,
    ) -> bool {
        let target_data_type = &self.input(target_id).data_type;
        let source_data_type = &self.output(source_id).data_type;
        source_data_type
            .default_value()
            .try_cast_to(target_data_type)
            .is_ok()
    }

    fn remove_graph_end(&mut self, node_id: &crate::NodeId) {
        self.graph_ends.retain(|id| id != node_id);
    }
}
