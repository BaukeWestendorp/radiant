use gpui::{App, AppContext, WindowOptions};
use river::{Graph, NodeId, NodeType, Value};

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            let graph = create_graph();

            Graph::<ExampleNodeType, ExampleDataType, ExampleValue>::build_view(graph, cx)
        });
    });
}
fn create_graph() -> Graph<ExampleNodeType, ExampleDataType, ExampleValue> {
    let mut graph = Graph::<ExampleNodeType, ExampleDataType, ExampleValue>::new();
    let a = graph.add_node(ExampleNodeType::IntegerNew);
    graph
        .get_output_mut(graph.get_node(a).unwrap().outputs().get(0).unwrap().1)
        .unwrap()
        .set_value(ExampleValue::Integer(42));
    let b = graph.add_node(ExampleNodeType::IntegerNew);
    graph
        .get_output_mut(graph.get_node(b).unwrap().outputs().get(0).unwrap().1)
        .unwrap()
        .set_value(ExampleValue::Integer(27));

    let adder = graph.add_node(ExampleNodeType::IntegerAdd);
    graph.add_connection(
        graph.get_node(a).unwrap().outputs().get(0).unwrap().1,
        graph.get_node(adder).unwrap().inputs().get(0).unwrap().1,
    );
    graph.add_connection(
        graph.get_node(b).unwrap().outputs().get(0).unwrap().1,
        graph.get_node(adder).unwrap().inputs().get(1).unwrap().1,
    );

    graph.process(adder);

    graph
}

#[derive(Debug, Clone)]
pub enum ExampleDataType {
    Integer,
}

#[derive(Debug, Clone)]
pub enum ExampleNodeType {
    IntegerNew,
    IntegerAdd,
}

#[derive(Debug, Clone)]
pub enum ExampleValue {
    Integer(i32),
}

impl Value for ExampleValue {
    type DataType = ExampleDataType;

    fn initial_value(data_type: &Self::DataType) -> Self {
        match data_type {
            Self::DataType::Integer => Self::Integer(0),
        }
    }
}

impl NodeType for ExampleNodeType {
    type DataType = ExampleDataType;
    type Value = ExampleValue;

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
                    (ExampleValue::Integer(a), ExampleValue::Integer(b)) => {
                        ExampleValue::Integer(a + b)
                    }
                };

                graph
                    .get_output_mut(node.outputs().get(0).unwrap().1)
                    .unwrap()
                    .set_value(sum);
            }
        }
    }
}
