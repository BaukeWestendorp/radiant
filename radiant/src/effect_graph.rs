use flow::{
    Graph, Input, NodeControl, Output, ProcessingContext, Template, Value as _,
    gpui::{ControlEvent, ControlView},
};
use gpui::*;
use ui::{NumberField, TextInputEvent};

#[derive(Debug, Clone, flow::Value, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[value(graph_def = GraphDef, data_type = DataType)]
pub enum Value {
    #[value(color = 0xCE39FF)]
    Number(f64),
    #[value(color = 0x1361FF)]
    Boolean(bool),
}

#[derive(Debug, Clone, Default)]
pub struct State {
    pub value: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Control {
    Slider { min: f64, max: f64, step: Option<f64> },
    Float,
    Checkbox,
}

impl flow::Control<GraphDef> for Control {
    fn view(
        &self,
        value: Value,
        id: ElementId,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<ControlView> {
        ControlView::new(cx, |cx| match self {
            Control::Slider { min, max, step } => {
                let field = cx.new(|cx| {
                    let value = value.try_into().expect("should always be able to convert initial input value to the value used by it's control");

                    let mut field = NumberField::new(id, cx.focus_handle(), window, cx);
                    field.set_value(value, cx);
                    field.set_min(Some(*min));
                    field.set_max(Some(*max));
                    field.set_step(*step);

                    field
                });

                cx.subscribe(&field, |_, field, event: &TextInputEvent, cx| {
                    if let TextInputEvent::Change(_) = event {
                        let value = field.read(cx).value(cx);
                        cx.emit(ControlEvent::<GraphDef>::Change(Value::Number(value)));
                        cx.notify();
                    }
                })
                .detach();

                field.into()
            }
            Control::Float => {
                let field = cx.new(|cx| {
                    let value = value.try_into().expect("should always be able to convert initial input value to the value used by it's control");

                    let mut field = NumberField::new(id, cx.focus_handle(), window, cx);
                    field.set_value(value, cx);

                    field
                });

                cx.subscribe(&field, |_, field, event: &TextInputEvent, cx| {
                    if let TextInputEvent::Change(_) = event {
                        let value = field.read(cx).value(cx);
                        cx.emit(ControlEvent::<GraphDef>::Change(Value::Number(value)));
                        cx.notify();
                    }
                })
                .detach();

                field.into()
            }
            Control::Checkbox => cx.new(|_cx| EmptyView).into(),
        })
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphDef;

impl flow::GraphDef for GraphDef {
    type ProcessingState = State;
    type Value = Value;
    type DataType = DataType;
    type Control = Control;
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
            vec![NodeControl::new("value", "Value", Value::Number(0.0), Control::Float)],
            Box::new(|_in, cv, ov, _pcx: &mut ProcessingContext<GraphDef>| {
                let value = cv.value("value").expect("should get value from control");
                ov.set_value("value", value.clone());
            }),
        ),
        Template::new(
            "number_add",
            "Add Number",
            vec![
                Input::new("a", "A", Value::Number(0.0), Control::Float),
                Input::new("b", "B", Value::Number(0.0), Control::Float),
            ],
            vec![Output::new("sum", "Sum", DataType::Number)],
            vec![],
            Box::new(|iv, _cv, ov, _pcx: &mut ProcessingContext<GraphDef>| {
                let a = iv.value("a").expect("should get value");
                let Some(Value::Number(a)) = a.cast_to(&DataType::Number) else { panic!() };

                let b = iv.value("b").expect("should get value");
                let Some(Value::Number(b)) = b.cast_to(&DataType::Number) else { panic!() };

                ov.set_value("sum", Value::Number(a + b));
            }),
        ),
        Template::new(
            "output",
            "Output",
            vec![Input::new(
                "value",
                "Value",
                Value::Number(0.0),
                Control::Slider { min: 0.0, max: 1.0, step: None },
            )],
            vec![],
            vec![],
            Box::new(|iv, _cv, _ov, pcx: &mut ProcessingContext<GraphDef>| {
                let value = iv.value("value").expect("should get value");
                let Some(Value::Number(value)) = value.cast_to(&DataType::Number) else { panic!() };
                pcx.value = value;
            }),
        ),
        Template::new(
            "boolean_new",
            "New Boolean",
            vec![],
            vec![Output::new("value", "Value", DataType::Boolean)],
            vec![NodeControl::new("value", "Value", Value::Boolean(false), Control::Checkbox)],
            Box::new(|_iv, cv, ov, _pcx: &mut ProcessingContext<GraphDef>| {
                let value = cv.value("value").expect("should get value from control");
                ov.set_value("value", value.clone());
            }),
        ),
        Template::new(
            "number_invert",
            "Invert Number",
            vec![
                Input::new(
                    "number",
                    "Number",
                    Value::Number(0.0),
                    Control::Slider { min: 0.0, max: 100.0, step: Some(5.0) },
                ),
                Input::new(
                    "should_invert",
                    "Should Invert",
                    Value::Boolean(false),
                    Control::Checkbox,
                ),
            ],
            vec![Output::new("result", "Result", DataType::Number)],
            vec![],
            Box::new(|iv, _cv, ov, _pcx: &mut ProcessingContext<GraphDef>| {
                let Some(number) = iv.value("number") else { panic!() };
                let Some(Value::Number(number)) = number.cast_to(&DataType::Number) else {
                    panic!()
                };

                let Some(should_invert) = iv.value("should_invert") else { panic!() };
                let Some(Value::Boolean(should_invert)) = should_invert.cast_to(&DataType::Boolean)
                else {
                    panic!()
                };

                let factor = if should_invert { -1.0 } else { 1.0 };
                ov.set_value("result", Value::Number(number * factor));
            }),
        ),
    ]);

    graph
}

fn load_graph() -> EffectGraph {
    let graph_json = include_str!("effect_graph.json");
    serde_json::from_str(graph_json).unwrap()
}
