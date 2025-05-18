use flow::gpui::{ControlEvent, ControlView};
use gpui::{App, ElementId, Entity, Window, prelude::*};
use ui::{Checkbox, CheckboxEvent, FieldEvent, NumberField, Selectable};

use super::{Def, Value};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Control {
    // Math
    Float,
    Bool,
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
                    let mut field = NumberField::<f32>::new(id, cx.focus_handle(), window, cx);
                    field.set_value(value, cx);
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
            Control::Bool => ControlView::new(cx, |cx| {
                let checkbox = cx.new(|_cx| {
                    let value: bool = value.try_into().unwrap();
                    Checkbox::new(id).selected(value)
                });

                cx.subscribe(&checkbox, |_, _, event: &CheckboxEvent, cx| {
                    let CheckboxEvent::Change(value) = event;
                    cx.emit(ControlEvent::<Def>::Change(Value::Bool(*value)));
                    cx.notify();
                })
                .detach();

                checkbox.into()
            }),
        }
    }
}
