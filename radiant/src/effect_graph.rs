use flow_gpui::{
    Graph,
    flow::{self, Input, Output, ProcessingContext, Template, Value as _},
};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
}

impl flow::Value<GraphDef> for Value {
    fn data_type(&self) -> DataType {
        match self {
            Self::Number(_) => DataType::Number,
            Self::Boolean(_) => DataType::Boolean,
        }
    }

    fn cast_to(&self, to: &DataType) -> Option<Value> {
        match (self, to) {
            (Self::Number(_), DataType::Number) => Some(self.clone()),
            (Self::Boolean(_), DataType::Boolean) => Some(self.clone()),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
    Number,
    Boolean,
}

impl flow_gpui::DataType<GraphDef> for DataType {
    fn color(&self) -> gpui::Hsla {
        match self {
            Self::Number => gpui::rgb(0xCE39FF).into(),
            Self::Boolean => gpui::rgb(0x1361FF).into(),
        }
    }
}

impl flow::DataType<GraphDef> for DataType {
    fn default_value(&self) -> <GraphDef as flow::GraphDef>::Value {
        match self {
            Self::Number => Value::Number(Default::default()),
            Self::Boolean => Value::Boolean(Default::default()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub value: f64,
}

impl Default for State {
    fn default() -> Self {
        Self { value: 0.0 }
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
            Box::new(|input_values, output_values, _: &mut ProcessingContext<GraphDef>| {
                let a = input_values.get_value("a").expect("should get value");
                let Some(Value::Number(a)) = a.cast_to(&DataType::Number) else { panic!() };

                let b = input_values.get_value("b").expect("should get value");
                let Some(Value::Number(b)) = b.cast_to(&DataType::Number) else { panic!() };

                output_values.set_value("sum", Value::Number(a + b));
            }),
        ),
        Template::new(
            "output",
            "Output",
            vec![Input::new("value", "Value", Value::Number(0.0), DataType::Number)],
            vec![],
            Box::new(|input_values, _, cx: &mut ProcessingContext<GraphDef>| {
                let value = input_values.get_value("value").expect("should get value");
                let Some(Value::Number(value)) = value.cast_to(&DataType::Number) else { panic!() };
                cx.value = value;
            }),
        ),
        Template::new(
            "boolean_new",
            "New Boolean",
            vec![],
            vec![Output::new("value", "Value", DataType::Boolean)],
            Box::new(|_, output_values, _| {
                output_values.set_value("value", Value::Boolean(true));
            }),
        ),
        Template::new(
            "number_invert",
            "Invert Number",
            vec![
                Input::new("number", "Number", Value::Number(0.0), DataType::Number),
                Input::new(
                    "should_invert",
                    "Should Invert",
                    Value::Boolean(false),
                    DataType::Boolean,
                ),
            ],
            vec![Output::new("result", "Result", DataType::Number)],
            Box::new(|input_values, output_values, _| {
                let Some(number) = input_values.get_value("number") else { panic!() };
                let Some(Value::Number(number)) = number.cast_to(&DataType::Number) else {
                    panic!()
                };

                let Some(should_invert) = input_values.get_value("should_invert") else { panic!() };
                let Some(Value::Boolean(should_invert)) = should_invert.cast_to(&DataType::Boolean)
                else {
                    panic!()
                };

                let factor = if should_invert { -1.0 } else { 1.0 };
                output_values.set_value("result", Value::Number(number * factor));
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
