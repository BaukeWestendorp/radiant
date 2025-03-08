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

impl Default for Value {
    fn default() -> Self {
        Value::Number(0.0)
    }
}

#[derive(Debug, Clone, Default)]
pub struct State {
    pub value: Value,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GraphDef;

impl flow::GraphDef for GraphDef {
    type ProcessingState = State;
    type Value = Value;
}

pub type EffectGraph = Graph<GraphDef>;

pub fn get_graph() -> EffectGraph {
    let mut graph = load_graph();

    graph.add_templates([
        Template::new(
            "number_new",
            "New Number",
            vec![],
            vec![Output::new("value", "Value")],
            Box::new(|_, output_values, _| {
                output_values.set_value("value", Value::Number(42.0));
            }),
        ),
        Template::new(
            "number_add",
            "Add Number",
            vec![
                Input::new("a", "A", Value::Number(0.0)),
                Input::new("b", "B", Value::Number(0.0)),
            ],
            vec![Output::new("sum", "Sum")],
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
            vec![Input::new("value", "Value", Value::Number(0.0))],
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
