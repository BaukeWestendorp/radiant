use std::ops::RangeInclusive;

use gpui::*;
use ui::input::{NumberField, TextField, TextFieldEvent};

use crate::graph::Value;

use super::node::ControlEvent;

#[derive(Debug, Clone)]
pub enum Control {
    IntField,
    FloatField {
        step: f32,
    },
    TextField,
    Range {
        range: RangeInclusive<f32>,
        step: f32,
        strict: bool,
    },
}

impl Control {
    pub fn view<V: EventEmitter<ControlEvent>>(
        &self,
        id: impl Into<ElementId>,
        initial_value: Value,
        cx: &mut ViewContext<V>,
    ) -> AnyView {
        use ui::input::NumberFieldEvent;

        match self {
            Self::IntField => {
                let field = cx.new_view(|cx| {
                    let mut int_field = NumberField::new(id, cx);
                    let Value::Int(value) = initial_value else {
                        panic!("Invalid value type, expected Int, got {:?}", initial_value)
                    };
                    int_field.set_value(value as f32, cx);
                    int_field.set_step(1.0);
                    int_field
                });

                cx.subscribe(&field, |_view, _field, event: &NumberFieldEvent, cx| {
                    let NumberFieldEvent::ChangeValue(value) = event;
                    cx.emit(ControlEvent::ChangeValue(Value::Int(*value as i32)));
                })
                .detach();

                field.into()
            }
            Self::FloatField { step } => {
                let field = cx.new_view(|cx| {
                    let mut field = NumberField::new(id, cx);
                    let Value::Float(value) = initial_value else {
                        panic!("Invalid value type")
                    };
                    field.set_value(value, cx);
                    field.set_step(*step);
                    field
                });
                cx.subscribe(&field, |_view, _field, event: &NumberFieldEvent, cx| {
                    let NumberFieldEvent::ChangeValue(value) = event;
                    cx.emit(ControlEvent::ChangeValue(Value::Float(*value)));
                })
                .detach();

                field.into()
            }
            Self::TextField => {
                let field = cx.new_view(|cx| {
                    let mut field = TextField::new(cx);
                    let Value::String(value) = initial_value else {
                        panic!("Invalid value type")
                    };
                    field.set_value(value, cx);
                    field
                });
                cx.subscribe(&field, |_view, _field, event: &TextFieldEvent, cx| {
                    if let TextFieldEvent::Change(value) = event {
                        cx.emit(ControlEvent::ChangeValue(Value::String(value.clone())));
                    }
                })
                .detach();

                field.into()
            }
            Self::Range {
                range,
                step,
                strict,
            } => {
                let field = cx.new_view(|cx| {
                    let mut field = NumberField::new(id, cx);
                    let Value::Float(value) = initial_value else {
                        panic!("Invalid value type")
                    };
                    field.set_value(value, cx);
                    field.set_step(*step);
                    field.set_range(range.clone());
                    field.set_strict(*strict);
                    field
                });
                cx.subscribe(&field, |_view, _field, event: &NumberFieldEvent, cx| {
                    let NumberFieldEvent::ChangeValue(value) = event;
                    cx.emit(ControlEvent::ChangeValue(Value::Float(*value)));
                })
                .detach();

                field.into()
            }
        }
    }
}
