use flow_gpui::{
    Graph,
    flow::{self, Input, Output, ProcessingContext, Template},
};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
    Number,
}

impl flow_gpui::DataType for DataType {
    fn color(&self) -> gpui::Hsla {
        match self {
            Self::Number => gpui::rgb(0xCE39FF).into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub value: Value,
}

impl Default for State {
    fn default() -> Self {
        Self { value: Value::Number(0.0) }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
pub struct GraphDef;

impl flow::GraphDef for GraphDef {
    type ProcessingState = State;
    type Value = Value;
    type DataType = DataType;
}

pub type EffectGraph = Graph<GraphDef>;

pub fn get_graph() -> EffectGraph {
    let mut graph = load_graph();

    graph.add_templates([
        Template::new(
            "number_new",
            "New Number",
            vec![],
            vec![Output::new("value", "Value", DataType::Number)],
            Box::new(|_, output_values, _| {
                output_values.set_value("value", Value::Number(42.0));
            }),
        ),
        Template::new(
            "number_add",
            "Add Number",
            vec![
                Input::new("a", "A", Value::Number(0.0), DataType::Number),
                Input::new("b", "B", Value::Number(0.0), DataType::Number),
            ],
            vec![Output::new("sum", "Sum", DataType::Number)],
            Box::new(|input_values, output_values, _| {
                let Some(Value::Number(a)) = input_values.get_value("a") else {
                    panic!("Invalid type for 'a'")
                };
                let Some(Value::Number(b)) = input_values.get_value("b") else {
                    panic!("Invalid type for 'b'")
                };
                output_values.set_value("sum", Value::Number(a + b));
            }),
        ),
        Template::new(
            "output",
            "Output",
            vec![Input::new("value", "Value", Value::Number(0.0), DataType::Number)],
            vec![],
            Box::new(|input_values, _, cx: &mut ProcessingContext<GraphDef>| {
                let value = input_values.get_value("value").unwrap();
                cx.value = value.clone();
            }),
        ),
    ]);

    let mut state = ProcessingContext::new();
    graph.process(&mut state);
    dbg!(&state.value);

    graph
}

fn load_graph() -> EffectGraph {
    let graph_json = include_str!("effect_graph.json");
    serde_json::from_str(graph_json).unwrap()
}
