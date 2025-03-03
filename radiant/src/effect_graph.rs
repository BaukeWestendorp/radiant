use flow::{Graph, Input, Output, ProcessingContext, Template};

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
    type State = State;
    type Value = Value;
}

pub type EffectGraph = Graph<GraphDef>;

pub fn get_graph() -> EffectGraph {
    let mut graph = load_graph();

    graph.add_template(Template::new(
        "new_number",
        "New Number",
        vec![],
        vec![Output::new("number", "Number")],
        Box::new(|_, output_values, _| {
            output_values.set_value("number", Value::Number(42.0));
        }),
    ));

    graph.add_template(Template::new(
        "output_number",
        "Output Number",
        vec![Input::new("number", "Number", Value::Number(0.0))],
        vec![],
        Box::new(|input_values, _, cx| {
            let value = input_values.get_value("number").unwrap();
            cx.value = value.clone();
        }),
    ));

    let mut pcx = ProcessingContext::new();
    graph.process(&mut pcx);
    dbg!(&pcx.value);

    graph
}

fn load_graph() -> EffectGraph {
    let graph_json = include_str!("effect_graph.json");
    serde_json::from_str(graph_json).unwrap()
}
