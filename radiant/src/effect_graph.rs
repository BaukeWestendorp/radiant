use flow::{Graph, Input, Node, Output, Socket, Template};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
}

impl Default for Value {
    fn default() -> Self {
        Value::Number(0.0)
    }
}

#[derive(Debug, Clone, Default)]
pub struct State {
    pub value: Value,
}

pub type EffectGraph = Graph<State, Value>;

pub fn get_graph() -> EffectGraph {
    let mut graph = EffectGraph::default();

    let new_value_id = graph.add_template(Template::new(
        "New Value",
        vec![],
        vec![Output::new("value", "Value")],
        Box::new(|_, output_values, _| {
            output_values.set_value("value", Value::Number(42.0));
        }),
    ));

    let output_value_id = graph.add_template(Template::new(
        "Output Number",
        vec![Input::new("value", "Value", Value::Number(0.0))],
        vec![],
        Box::new(|input_values, _, cx| {
            let value = input_values.get_value("value");
            cx.value = value.clone();
        }),
    ));

    let new_value_node_id = graph.add_node(Node::new(new_value_id));
    let output_value_node_id = graph.add_node(Node::new(output_value_id));

    graph.add_edge(
        Socket::new(new_value_node_id, "value".to_string()),
        Socket::new(output_value_node_id, "value".to_string()),
    );

    graph
}
