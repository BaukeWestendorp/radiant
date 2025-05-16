use flow::{
    Input, Output, ProcessingContext, Template, Value as _,
    gpui::{ControlEvent, ControlView},
};
use gpui::{App, ElementId, Entity, Window, prelude::*};
use ui::{Field, FieldEvent};

#[derive(Debug, Clone, flow::Value)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(graph_def = Def, data_type = DataType)]
pub enum Value {
    #[value(color = 0x1BD5FF)]
    Float(f32),
}

#[derive(Debug, Clone)]
pub struct State {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Control {
    Float,
}

impl flow::Control<Def> for Control {
    fn view(
        &self,
        value: Value,
        id: ElementId,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<ControlView> {
        match self {
            Control::Float => ControlView::new(cx, |cx| {
                let field = cx.new(|cx| {
                    let value: f32 = value.try_into().unwrap();
                    let field = Field::<f32>::new(id, cx.focus_handle(), window, cx);
                    field.set_value(&value, cx);
                    field
                });

                cx.subscribe(&field, |_, _, event: &FieldEvent<f32>, cx| {
                    if let FieldEvent::Change(value) = event {
                        cx.emit(ControlEvent::<Def>::Change(Value::Float(*value)));
                        cx.notify();
                    }
                })
                .detach();

                field.into()
            }),
        }
    }
}

#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Def;

impl flow::GraphDef for Def {
    type ProcessingState = State;
    type Value = Value;
    type DataType = DataType;
    type Control = Control;
}

pub type EffectGraph = flow::Graph<Def>;

pub fn insert_templates(graph: &mut EffectGraph) {
    let add_float = Template::new(
        "add_float",
        "Add (float)",
        |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let a = iv
                .value("a")
                .and_then(|a| a.cast_to(&DataType::Float))
                .and_then(|a| if let Value::Float(a) = a { Some(a) } else { None })
                .expect("Invalid float");

            let b = iv
                .value("b")
                .and_then(|v| v.cast_to(&DataType::Float))
                .and_then(|v| if let Value::Float(v) = v { Some(v) } else { None })
                .expect("Invalid float");

            ov.set_value("c", Value::Float(a + b));
        },
    )
    .add_input(Input::new("a", "A", Value::Float(Default::default()), Control::Float))
    .add_input(Input::new("b", "B", Value::Float(Default::default()), Control::Float))
    .add_output(Output::new("c", "C", DataType::Float));

    let sub_float = Template::new(
        "sub_float",
        "Subtract (float)",
        |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let a = iv
                .value("a")
                .and_then(|a| a.cast_to(&DataType::Float))
                .and_then(|a| if let Value::Float(a) = a { Some(a) } else { None })
                .expect("Invalid float");

            let b = iv
                .value("b")
                .and_then(|v| v.cast_to(&DataType::Float))
                .and_then(|v| if let Value::Float(v) = v { Some(v) } else { None })
                .expect("Invalid float");

            ov.set_value("c", Value::Float(a - b));
        },
    )
    .add_input(Input::new("a", "A", Value::Float(Default::default()), Control::Float))
    .add_input(Input::new("b", "B", Value::Float(Default::default()), Control::Float))
    .add_output(Output::new("c", "C", DataType::Float));

    let mul_float = Template::new(
        "mul_float",
        "Multiply (float)",
        |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let a = iv
                .value("a")
                .and_then(|a| a.cast_to(&DataType::Float))
                .and_then(|a| if let Value::Float(a) = a { Some(a) } else { None })
                .expect("Invalid float");

            let b = iv
                .value("b")
                .and_then(|v| v.cast_to(&DataType::Float))
                .and_then(|v| if let Value::Float(v) = v { Some(v) } else { None })
                .expect("Invalid float");

            ov.set_value("c", Value::Float(a * b));
        },
    )
    .add_input(Input::new("a", "A", Value::Float(Default::default()), Control::Float))
    .add_input(Input::new("b", "B", Value::Float(Default::default()), Control::Float))
    .add_output(Output::new("c", "C", DataType::Float));

    let div_float = Template::new(
        "div_float",
        "Divide (float)",
        |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let a = iv
                .value("a")
                .and_then(|a| a.cast_to(&DataType::Float))
                .and_then(|a| if let Value::Float(a) = a { Some(a) } else { None })
                .expect("Invalid float");

            let b = iv
                .value("b")
                .and_then(|v| v.cast_to(&DataType::Float))
                .and_then(|v| if let Value::Float(v) = v { Some(v) } else { None })
                .expect("Invalid float");

            ov.set_value("c", Value::Float(a / b));
        },
    )
    .add_input(Input::new("a", "A", Value::Float(Default::default()), Control::Float))
    .add_input(Input::new("b", "B", Value::Float(Default::default()), Control::Float))
    .add_output(Output::new("c", "C", DataType::Float));

    graph.add_templates([add_float, sub_float, mul_float, div_float]);
}
