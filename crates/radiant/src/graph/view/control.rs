use std::ops::RangeInclusive;

use gpui::*;
use ui::input::{NumberField, Slider, SliderEvent, TextField, TextFieldEvent};

use crate::graph::Value;

use super::node::ControlEvent;

#[derive(Debug, Clone)]
pub enum Control {
    IntField,
    FloatField,
    TextField,
    Slider {
        range: RangeInclusive<f32>,
        step: Option<f32>,
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
                    let mut int_field = NumberField::new(cx);
                    let value: i32 = initial_value
                        .try_into()
                        .expect("IntField should have i32 value");
                    int_field.set_value(value as f32, cx);
                    int_field.set_validate(Some(Box::new(|v| v.parse::<i32>().is_ok())), cx);
                    int_field
                });

                cx.subscribe(&field, |_this, _field, event: &NumberFieldEvent, cx| {
                    let NumberFieldEvent::Change(int_value) = event;
                    let value = Value::Int(*int_value as i32);
                    cx.emit(ControlEvent::Change(value));
                })
                .detach();

                field.into()
            }
            Self::FloatField => {
                let field = cx.new_view(|cx| {
                    let mut field = NumberField::new(cx);
                    let value: f32 = initial_value
                        .try_into()
                        .expect("FloatField should have f32 value");
                    field.set_value(value, cx);
                    field
                });
                cx.subscribe(&field, |_this, _field, event: &NumberFieldEvent, cx| {
                    let NumberFieldEvent::Change(float_value) = event;
                    let value = Value::Float(*float_value);
                    cx.emit(ControlEvent::Change(value));
                })
                .detach();

                field.into()
            }
            Self::TextField => {
                let field = cx.new_view(|cx| {
                    let mut field = TextField::new(cx);
                    let value: SharedString = initial_value
                        .try_into()
                        .expect("TextField should have SharedString value");
                    field.set_value(value, cx);
                    field
                });
                cx.subscribe(&field, |_this, _field, event: &TextFieldEvent, cx| {
                    if let TextFieldEvent::Change(string_value) = event {
                        let value = Value::String(string_value.clone());
                        cx.emit(ControlEvent::Change(value));
                    }
                })
                .detach();

                field.into()
            }
            Self::Slider {
                range,
                step,
                strict,
            } => {
                let slider = cx.new_view(|cx| {
                    let mut slider = Slider::new(id, cx);
                    let value: f32 = initial_value
                        .try_into()
                        .expect("Slider should have f32 value");
                    slider.set_value(value, cx);
                    slider.set_step(*step, cx);
                    slider.set_range(range.clone(), cx);
                    slider.set_strict(*strict);
                    slider
                });
                cx.subscribe(&slider, |_this, _slider, event: &SliderEvent, cx| {
                    let SliderEvent::Change(value) = event;
                    cx.emit(ControlEvent::Change(Value::Float(*value)));
                })
                .detach();

                slider.into()
            }
        }
    }
}
