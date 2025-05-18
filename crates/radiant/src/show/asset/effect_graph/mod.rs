pub mod templates;

use crate::show::asset::{Asset, FixtureGroup};
use flow::{
    Graph,
    gpui::{ControlEvent, ControlView},
};
use gpui::{App, ElementId, Entity, Window, prelude::*};
use ui::{Checkbox, CheckboxEvent, FieldEvent, NumberField, Selectable};

#[derive(Debug, Clone, flow::Value)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(graph_def = Def, data_type = DataType)]
pub enum Value {
    // Math
    #[value(color = 0x52B4FF)]
    Float(f32),
    #[value(color = 0xFF178C)]
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct State {
    pub multiverse: Entity<dmx::Multiverse>,
    pub fixture_group: Entity<Asset<FixtureGroup>>,
    pub fixture_id_index: Option<usize>,
}

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

#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Def;

impl flow::GraphDef for Def {
    type ProcessingState = State;
    type Value = Value;
    type DataType = DataType;
    type Control = Control;
}

pub type EffectGraph = Graph<Def>;
