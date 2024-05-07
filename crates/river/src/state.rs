use std::collections::HashMap;

use crate::graph::{Graph, InputId, InputKind, OutputId};

pub struct GraphState<GraphValue> {
    input_constants: HashMap<InputId, GraphValue>,
    output_values: HashMap<OutputId, GraphValue>,
}

impl<GraphValue> GraphState<GraphValue> {
    pub fn new() -> Self {
        Self {
            input_constants: HashMap::new(),
            output_values: HashMap::new(),
        }
    }

    pub fn set_output_value(&mut self, output_id: OutputId, value: GraphValue) {
        if !self.output_values.contains_key(&output_id) {
            self.output_values.insert(output_id, value);
        } else {
            *self.output_values.get_mut(&output_id).unwrap() = value;
        }
    }

    pub fn get_output_value(
        &mut self,
        output_id: &OutputId,
        graph: &Graph<GraphValue>,
    ) -> &GraphValue {
        let node_id = graph.output_parent_node(*output_id);
        graph.node(node_id).unwrap().process(graph, self);
        self.output_values.get(output_id).unwrap()
    }

    pub fn get_input_value(
        &mut self,
        input_id: InputId,
        graph: &Graph<GraphValue>,
    ) -> Option<&GraphValue> {
        let input = graph.input(input_id).unwrap();
        match input.kind() {
            InputKind::ConnectionOnly => match graph.connection(input_id) {
                Some(output_id) => {
                    let output_value = self.get_output_value(output_id, graph);
                    Some(output_value)
                }
                None => None,
            },
            InputKind::ConstantOnly => {
                let constant = self.input_constants.get(&input_id).unwrap();
                Some(constant)
            }
            InputKind::ConnectionOrConstant => match graph.connection(input_id) {
                Some(output_id) => {
                    let output_value = self.get_output_value(output_id, graph);
                    Some(output_value)
                }
                None => {
                    let constant = self.input_constants.get(&input_id).unwrap();
                    Some(constant)
                }
            },
        }
    }
}
