use super::{Graph, NodeId, NodeType, Value};

#[derive(Debug, Clone)]
pub enum GraphDataType {
    Integer,
}

#[derive(Debug, Clone)]
pub enum GraphNodeType {
    IntegerNew,
    IntegerAdd,
}

#[derive(Debug, Clone)]
pub enum GraphValue {
    Integer(i32),
}

impl Value for GraphValue {
    type DataType = GraphDataType;

    fn initial_value(data_type: &Self::DataType) -> Self {
        match data_type {
            Self::DataType::Integer => Self::Integer(0),
        }
    }
}

impl NodeType for GraphNodeType {
    type DataType = GraphDataType;
    type Value = GraphValue;

    fn build_node(&self, graph: &mut Graph<Self, Self::DataType, Self::Value>, node_id: NodeId) {
        match self {
            Self::IntegerNew => {
                graph.add_output(node_id, "Value".to_string(), Self::DataType::Integer);
            }
            Self::IntegerAdd => {
                graph.add_input(node_id, "A".to_string(), Self::DataType::Integer);
                graph.add_input(node_id, "B".to_string(), Self::DataType::Integer);
                graph.add_output(node_id, "Sum".to_string(), Self::DataType::Integer);
            }
        }
    }

    fn process(&self, graph: &mut Graph<Self, Self::DataType, Self::Value>, node_id: NodeId) {
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
                    (GraphValue::Integer(a), GraphValue::Integer(b)) => GraphValue::Integer(a + b),
                };

                graph
                    .get_output_mut(node.outputs().get(0).unwrap().1)
                    .unwrap()
                    .set_value(sum);
            }
        }
    }
}
