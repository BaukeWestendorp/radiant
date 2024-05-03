use river::{Graph, NodeId, NodeType, Value};

fn main() {
    let mut editor = Graph::<ExampleNodeType, ExampleDataType, ExampleValue>::new();
    let a = editor.add_node(ExampleNodeType::IntegerNew);
    editor
        .get_output_mut(editor.get_node(a).unwrap().outputs().get(0).unwrap().1)
        .unwrap()
        .set_value(ExampleValue::Integer(42));
    let b = editor.add_node(ExampleNodeType::IntegerNew);
    editor
        .get_output_mut(editor.get_node(b).unwrap().outputs().get(0).unwrap().1)
        .unwrap()
        .set_value(ExampleValue::Integer(27));

    let adder = editor.add_node(ExampleNodeType::IntegerAdd);
    editor.add_connection(
        editor.get_node(a).unwrap().outputs().get(0).unwrap().1,
        editor.get_node(adder).unwrap().inputs().get(0).unwrap().1,
    );
    editor.add_connection(
        editor.get_node(b).unwrap().outputs().get(0).unwrap().1,
        editor.get_node(adder).unwrap().inputs().get(1).unwrap().1,
    );

    editor.process(adder);

    dbg!(editor
        .get_output(editor.get_node(adder).unwrap().outputs().get(0).unwrap().1)
        .unwrap()
        .value());
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

    fn build_node(&self, editor: &mut Graph<Self, Self::DataType, Self::Value>, node_id: NodeId) {
        match self {
            Self::IntegerNew => {
                editor.add_output(node_id, "Value".to_string(), Self::DataType::Integer);
            }
            Self::IntegerAdd => {
                editor.add_input(node_id, "A".to_string(), Self::DataType::Integer);
                editor.add_input(node_id, "B".to_string(), Self::DataType::Integer);
                editor.add_output(node_id, "Sum".to_string(), Self::DataType::Integer);
            }
        }
    }

    fn process(&self, editor: &mut Graph<Self, Self::DataType, Self::Value>, node_id: NodeId) {
        let node = editor.get_node(node_id).unwrap();

        match self {
            Self::IntegerNew => {}
            Self::IntegerAdd => {
                let inputs = node.inputs();

                let a_id = editor
                    .get_connected_output(inputs.get(0).unwrap().1)
                    .unwrap();
                let a = editor.get_output(*a_id).unwrap();

                let b_id = editor
                    .get_connected_output(inputs.get(1).unwrap().1)
                    .unwrap();
                let b = editor.get_output(*b_id).unwrap();

                let sum = match (a.value(), b.value()) {
                    (ExampleValue::Integer(a), ExampleValue::Integer(b)) => {
                        ExampleValue::Integer(a + b)
                    }
                };

                editor
                    .get_output_mut(node.outputs().get(0).unwrap().1)
                    .unwrap()
                    .set_value(sum);
            }
        }
    }
}
