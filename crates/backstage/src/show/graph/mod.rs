pub mod graph;
pub mod state;

pub use graph::*;
pub use state::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum GraphValue {
    Integer(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ValueType {
    Integer,
}

impl ValueType {
    pub fn hex_color(&self) -> u32 {
        match self {
            Self::Integer => 0x4070fb,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum NodeKind {
    NewInteger(i32),
    Add,
    Output,
}

impl NodeKind {
    fn process(
        &self,
        node: &GraphNode,
        graph: &Graph,
        state: &mut GraphState,
    ) -> Option<GraphValue> {
        match self {
            Self::NewInteger(int) => {
                state.set_output_value(node.output("value"), GraphValue::Integer(*int));
                None
            }
            Self::Add => {
                let a = state
                    .get_input_value(node.input("a"), graph)
                    .unwrap()
                    .clone();
                let b = state
                    .get_input_value(node.input("b"), graph)
                    .unwrap()
                    .clone();
                let sum = match (a, b) {
                    (GraphValue::Integer(a), GraphValue::Integer(b)) => GraphValue::Integer(a + b),
                };
                state.set_output_value(node.output("sum"), sum);
                None
            }
            Self::Output => state.get_input_value(node.input("value"), graph).cloned(),
        }
    }

    fn build(&self, node: &GraphNode, graph: &mut Graph) {
        match self {
            Self::NewInteger(_) => {
                node.add_output(
                    "value".to_string(),
                    graph,
                    "Value".to_string(),
                    OutputKind::ConstantOnly(OutputControl::NumberInput),
                    ValueType::Integer,
                );
            }
            Self::Add => {
                node.add_input(
                    "a".to_string(),
                    graph,
                    "A".to_string(),
                    InputKind::ConnectionOrConstant,
                    ValueType::Integer,
                );
                node.add_input(
                    "b".to_string(),
                    graph,
                    "B".to_string(),
                    InputKind::ConnectionOrConstant,
                    ValueType::Integer,
                );
                node.add_output(
                    "sum".to_string(),
                    graph,
                    "Sum".to_string(),
                    OutputKind::CalculatedOnly,
                    ValueType::Integer,
                );
            }
            Self::Output => {
                node.add_input(
                    "value".to_string(),
                    graph,
                    "Value".to_string(),
                    InputKind::ConnectionOrConstant,
                    ValueType::Integer,
                );
            }
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Self::NewInteger(_) => "New Value",
            Self::Add => "Add",
            Self::Output => "Output",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::show::graph::GraphState;

    use super::{Graph, GraphNode, GraphValue, NodeKind};

    #[test]
    fn test_simple_graph() {
        let mut graph = Graph::new();
        let mut graph_state = GraphState::new();

        let a_id = graph.add_node(GraphNode::new(NodeKind::NewInteger(27), 0.0, 0.0));
        let b_id = graph.add_node(GraphNode::new(NodeKind::NewInteger(15), 0.0, 0.0));
        let add_id = graph.add_node(GraphNode::new(NodeKind::Add, 0.0, 0.0));
        let output_id = graph.add_node(GraphNode::new(NodeKind::Output, 0.0, 0.0));

        graph.add_connection(
            graph.node(a_id).unwrap().output("value"),
            graph.node(add_id).unwrap().input("a"),
        );
        graph.add_connection(
            graph.node(b_id).unwrap().output("value"),
            graph.node(add_id).unwrap().input("b"),
        );
        graph.add_connection(
            graph.node(add_id).unwrap().output("sum"),
            graph.node(output_id).unwrap().input("value"),
        );

        let output_value = graph
            .node(output_id)
            .unwrap()
            .process(&graph, &mut graph_state);

        assert_eq!(output_value, Some(GraphValue::Integer(42)));
    }
}
