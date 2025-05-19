use flow::gpui::{ControlEvent, ControlView};
use gpui::{App, ElementId, Entity, Window, prelude::*};
use ui::{Checkbox, CheckboxEvent, Field, FieldEvent, NumberField, Selectable};

use crate::{
    show::{FloatingDmxValue, attr::AnyPresetAssetId, patch::FixtureId},
    ui::input::{PresetSelector, PresetSelectorEvent},
};

use super::{Def, Value};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Control {
    // Asset
    Preset,

    // DMX
    DmxValue,
    DmxAddress,

    // Math
    Float,
    Bool,

    FixtureId,
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
            Control::Preset => ControlView::new(cx, |cx| {
                let field = cx.new(|cx| {
                    let value: Option<AnyPresetAssetId> = value.try_into().unwrap();
                    let mut field = PresetSelector::new(id, cx.focus_handle(), window, cx);
                    field.set_value(value, cx);
                    field
                });

                cx.subscribe(&field, |_, _, event: &PresetSelectorEvent, cx| {
                    let PresetSelectorEvent::Change(value) = event;
                    cx.emit(ControlEvent::<Def>::Change(Value::Preset(*value)));
                    cx.notify();
                })
                .detach();

                field.into()
            }),

            Control::DmxValue => ControlView::new(cx, |cx| {
                let field = cx.new(|cx| {
                    let value: FloatingDmxValue = value.try_into().unwrap();
                    let mut field =
                        NumberField::<FloatingDmxValue>::new(id, cx.focus_handle(), window, cx);
                    field.set_value(value, cx);
                    field
                });

                cx.subscribe(&field, |_, _, event: &FieldEvent<FloatingDmxValue>, cx| {
                    if let FieldEvent::Change(value) = event {
                        cx.emit(ControlEvent::<Def>::Change(Value::DmxValue(*value)));
                        cx.notify();
                    }
                })
                .detach();

                field.into()
            }),
            Control::DmxAddress => ControlView::new(cx, |cx| {
                let field = cx.new(|cx| {
                    let value: dmx::Address = value.try_into().unwrap();
                    let field = Field::<dmx::Address>::new(id, cx.focus_handle(), window, cx);
                    field.set_value(&value, cx);
                    field
                });

                cx.subscribe(&field, |_, _, event: &FieldEvent<dmx::Address>, cx| {
                    if let FieldEvent::Change(value) = event {
                        cx.emit(ControlEvent::<Def>::Change(Value::DmxAddress(*value)));
                        cx.notify();
                    }
                })
                .detach();

                field.into()
            }),

            Control::Float => ControlView::new(cx, |cx| {
                let field = cx.new(|cx| {
                    let value: f64 = value.try_into().unwrap();
                    let mut field = NumberField::<f64>::new(id, cx.focus_handle(), window, cx);
                    field.set_value(value, cx);
                    field
                });

                cx.subscribe(&field, |_, _, event: &FieldEvent<f64>, cx| {
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

            Control::FixtureId => ControlView::new(cx, |cx| {
                let field = cx.new(|cx| {
                    let value: FixtureId = value.try_into().unwrap();
                    let mut field =
                        NumberField::<FixtureId>::new(id, cx.focus_handle(), window, cx);
                    field.set_value(value, cx);
                    field
                });

                cx.subscribe(&field, |_, _, event: &FieldEvent<FixtureId>, cx| {
                    if let FieldEvent::Change(value) = event {
                        cx.emit(ControlEvent::<Def>::Change(Value::FixtureId(*value)));
                        cx.notify();
                    }
                })
                .detach();

                field.into()
            }),
        }
    }
}
