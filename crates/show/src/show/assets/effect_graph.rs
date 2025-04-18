use flow::{
    Graph, Input, NodeControl, Output, ProcessingContext, Template, Value as _,
    gpui::{ControlEvent, ControlView},
};
use gpui::*;
use ui::{NumberField, TextInputEvent};

use crate::define_asset;

define_asset!(EffectGraph, EffectGraphAsset, EffectGraphId);

#[derive(Debug, Clone, flow::Value)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[value(graph_def = EffectGraphDef, data_type = DataType)]
pub enum EffectGraphValue {
    #[value(color = 0xCE39FF)]
    Number(f64),
    #[value(color = 0x1361FF)]
    Boolean(bool),
}

#[derive(Debug, Clone, Default)]
pub struct EffectGraphState {
    pub value: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EffectGraphControl {
    Slider { min: f64, max: f64, step: Option<f64> },
    Float,
    Checkbox,
}

impl flow::Control<EffectGraphDef> for EffectGraphControl {
    fn view(
        &self,
        value: EffectGraphValue,
        id: ElementId,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<ControlView> {
        ControlView::new(cx, |cx| match self {
            EffectGraphControl::Slider { min, max, step } => {
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
                        cx.emit(ControlEvent::<EffectGraphDef>::Change(EffectGraphValue::Number(
                            value,
                        )));
                        cx.notify();
                    }
                })
                .detach();

                field.into()
            }
            EffectGraphControl::Float => {
                let field = cx.new(|cx| {
                    let value = value.try_into().expect("should always be able to convert initial input value to the value used by it's control");

                    let mut field = NumberField::new(id, cx.focus_handle(), window, cx);
                    field.set_value(value, cx);

                    field
                });

                cx.subscribe(&field, |_, field, event: &TextInputEvent, cx| {
                    if let TextInputEvent::Change(_) = event {
                        let value = field.read(cx).value(cx);
                        cx.emit(ControlEvent::<EffectGraphDef>::Change(EffectGraphValue::Number(
                            value,
                        )));
                        cx.notify();
                    }
                })
                .detach();

                field.into()
            }
            EffectGraphControl::Checkbox => cx.new(|_cx| EmptyView).into(),
        })
    }
}

#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EffectGraphDef;

impl flow::GraphDef for EffectGraphDef {
    type ProcessingState = EffectGraphState;
    type Value = EffectGraphValue;
    type DataType = DataType;
    type Control = EffectGraphControl;
}

pub type EffectGraph = Graph<EffectGraphDef>;

pub fn insert_templates(graph: &mut EffectGraph) {
    graph.add_templates([
        Template::new(
            "number_new",
            "New Number",
            vec![],
            vec![Output::new("value", "Value", DataType::Number)],
            vec![NodeControl::new(
                "value",
                "Value",
                EffectGraphValue::Number(0.0),
                EffectGraphControl::Float,
            )],
            Box::new(|_in, cv, ov, _pcx: &mut ProcessingContext<EffectGraphDef>| {
                let value = cv.value("value").expect("should get value from control");
                ov.set_value("value", value.clone());
            }),
        ),
        Template::new(
            "number_add",
            "Add Number",
            vec![
                Input::new("a", "A", EffectGraphValue::Number(0.0), EffectGraphControl::Float),
                Input::new("b", "B", EffectGraphValue::Number(0.0), EffectGraphControl::Float),
            ],
            vec![Output::new("sum", "Sum", DataType::Number)],
            vec![],
            Box::new(|iv, _cv, ov, _pcx: &mut ProcessingContext<EffectGraphDef>| {
                let a = iv.value("a").expect("should get value");
                let Some(EffectGraphValue::Number(a)) = a.cast_to(&DataType::Number) else {
                    panic!()
                };

                let b = iv.value("b").expect("should get value");
                let Some(EffectGraphValue::Number(b)) = b.cast_to(&DataType::Number) else {
                    panic!()
                };

                ov.set_value("sum", EffectGraphValue::Number(a + b));
            }),
        ),
        Template::new(
            "output",
            "Output",
            vec![Input::new(
                "value",
                "Value",
                EffectGraphValue::Number(0.0),
                EffectGraphControl::Slider { min: 0.0, max: 1.0, step: None },
            )],
            vec![],
            vec![],
            Box::new(|iv, _cv, _ov, pcx: &mut ProcessingContext<EffectGraphDef>| {
                let value = iv.value("value").expect("should get value");
                let Some(EffectGraphValue::Number(value)) = value.cast_to(&DataType::Number) else {
                    panic!()
                };
                pcx.value = value;
            }),
        ),
        Template::new(
            "boolean_new",
            "New Boolean",
            vec![],
            vec![Output::new("value", "Value", DataType::Boolean)],
            vec![NodeControl::new(
                "value",
                "Value",
                EffectGraphValue::Boolean(false),
                EffectGraphControl::Checkbox,
            )],
            Box::new(|_iv, cv, ov, _pcx: &mut ProcessingContext<EffectGraphDef>| {
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
                    EffectGraphValue::Number(0.0),
                    EffectGraphControl::Slider { min: 0.0, max: 100.0, step: Some(5.0) },
                ),
                Input::new(
                    "should_invert",
                    "Should Invert",
                    EffectGraphValue::Boolean(false),
                    EffectGraphControl::Checkbox,
                ),
            ],
            vec![Output::new("result", "Result", DataType::Number)],
            vec![],
            Box::new(|iv, _cv, ov, _pcx: &mut ProcessingContext<EffectGraphDef>| {
                let Some(number) = iv.value("number") else { panic!() };
                let Some(EffectGraphValue::Number(number)) = number.cast_to(&DataType::Number)
                else {
                    panic!()
                };

                let Some(should_invert) = iv.value("should_invert") else { panic!() };
                let Some(EffectGraphValue::Boolean(should_invert)) =
                    should_invert.cast_to(&DataType::Boolean)
                else {
                    panic!()
                };

                let factor = if should_invert { -1.0 } else { 1.0 };
                ov.set_value("result", EffectGraphValue::Number(number * factor));
            }),
        ),
    ]);
}
