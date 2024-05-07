use river::{
    graph::{Graph, GraphNode, InputKind, NodeImpl, OutputKind},
    state::GraphState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphValue {
    Integer(i32),
}

pub struct NewValueNode {
    pub value: GraphValue,
}

impl NodeImpl<GraphValue> for NewValueNode {
    fn process(
        &self,
        node: &GraphNode<GraphValue>,
        _graph: &Graph<GraphValue>,
        state: &mut GraphState<GraphValue>,
    ) -> Option<GraphValue> {
        state.set_output_value(node.output("value"), self.value.clone());
        None
    }

    fn build(&self, node: &GraphNode<GraphValue>, graph: &mut Graph<GraphValue>) {
        node.add_output(
            "value".to_string(),
            graph,
            "Value".to_string(),
            OutputKind::ConstantOnly,
        );
    }
}

pub struct AddNode;

impl NodeImpl<GraphValue> for AddNode {
    fn process(
        &self,
        node: &GraphNode<GraphValue>,
        graph: &Graph<GraphValue>,
        state: &mut GraphState<GraphValue>,
    ) -> Option<GraphValue> {
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

    fn build(&self, node: &GraphNode<GraphValue>, graph: &mut Graph<GraphValue>) {
        node.add_input(
            "a".to_string(),
            graph,
            "A".to_string(),
            InputKind::ConnectionOrConstant,
        );
        node.add_input(
            "b".to_string(),
            graph,
            "B".to_string(),
            InputKind::ConnectionOrConstant,
        );
        node.add_output(
            "sum".to_string(),
            graph,
            "Sum".to_string(),
            OutputKind::CalculatedOnly,
        );
    }
}

pub struct OutputNode;

impl NodeImpl<GraphValue> for OutputNode {
    fn process(
        &self,
        node: &GraphNode<GraphValue>,
        graph: &Graph<GraphValue>,
        state: &mut GraphState<GraphValue>,
    ) -> Option<GraphValue> {
        state.get_input_value(node.input("value"), graph).cloned()
    }

    fn build(&self, node: &GraphNode<GraphValue>, graph: &mut Graph<GraphValue>) {
        node.add_input(
            "value".to_string(),
            graph,
            "Value".to_string(),
            InputKind::ConnectionOrConstant,
        );
    }
}

#[test]
fn test_simple_add() {
    let mut graph = Graph::new();
    let new_int_a_id = graph.add_node(GraphNode::new(Box::new(NewValueNode {
        value: GraphValue::Integer(27),
    })));
    let new_int_b_id = graph.add_node(GraphNode::new(Box::new(NewValueNode {
        value: GraphValue::Integer(42),
    })));
    let add_int_node_id = graph.add_node(GraphNode::new(Box::new(AddNode)));
    let output_node_id = graph.add_node(GraphNode::new(Box::new(OutputNode)));

    graph.add_connection(
        graph.node(new_int_a_id).unwrap().output("value"),
        graph.node(add_int_node_id).unwrap().input("a"),
    );
    graph.add_connection(
        graph.node(new_int_b_id).unwrap().output("value"),
        graph.node(add_int_node_id).unwrap().input("b"),
    );

    graph.add_connection(
        graph.node(add_int_node_id).unwrap().output("sum"),
        graph.node(output_node_id).unwrap().input("value"),
    );

    let mut graph_state = GraphState::<GraphValue>::new();

    let value = graph
        .node(output_node_id)
        .unwrap()
        .process(&graph, &mut graph_state);
    assert_eq!(value, Some(GraphValue::Integer(69)));
}
