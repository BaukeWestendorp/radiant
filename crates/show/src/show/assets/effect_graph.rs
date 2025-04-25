use crate::define_asset;
use flow::{
    Graph, Input, ProcessingContext, Template, Value as _,
    gpui::{ControlEvent, ControlView},
};
use gpui::*;
use ui::{Field, FieldEvent, NumberField, NumberFieldImpl};

define_asset!(EffectGraph, EffectGraphAsset, EffectGraphId);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FloatingDmxValue(pub f32);

impl From<FloatingDmxValue> for dmx::Value {
    fn from(value: FloatingDmxValue) -> Self {
        dmx::Value((value.0 * (u8::MAX as f32)) as u8)
    }
}

impl NumberFieldImpl for FloatingDmxValue {
    type Value = Self;

    const MIN: Option<Self> = Some(FloatingDmxValue(0.0));
    const MAX: Option<Self> = Some(FloatingDmxValue(1.0));
    const STEP: Option<Self> = None;

    fn from_str_or_default(s: &str) -> Self::Value {
        Self(s.parse().unwrap_or_default())
    }

    fn to_shared_string(value: &Self::Value) -> SharedString {
        value.0.to_string().into()
    }

    fn as_f32(value: &Self::Value) -> f32 {
        value.0
    }

    fn from_f32(v: f32) -> Self::Value {
        Self(v)
    }
}

#[derive(Debug, Clone, flow::Value)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(graph_def = EffectGraphDef, data_type = EffectGraphDataType)]
pub enum EffectGraphValue {
    #[value(color = 0x1BD5FF)]
    DmxAddress(dmx::Address),

    #[value(color = 0x1361FF)]
    DmxValue(FloatingDmxValue),
}

#[derive(Debug, Clone, Default)]
pub struct EffectGraphState {
    pub multiverse: dmx::Multiverse,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EffectGraphControl {
    DmxAddress,
    DmxValue,
}

impl flow::Control<EffectGraphDef> for EffectGraphControl {
    fn view(
        &self,
        value: EffectGraphValue,
        id: ElementId,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<ControlView> {
        match self {
            EffectGraphControl::DmxAddress => ControlView::new(cx, |cx| {
                let field = cx.new(|cx| {
                    let value: dmx::Address = value.try_into().expect(
                        "should convert initial input value to the value used by it's control",
                    );
                    let field = Field::<dmx::Address>::new(id, cx.focus_handle(), window, cx);
                    field.set_value(&value, cx);
                    field
                });

                cx.subscribe(&field, |_, _, event: &FieldEvent<dmx::Address>, cx| {
                    if let FieldEvent::Change(value) = event {
                        cx.emit(ControlEvent::<EffectGraphDef>::Change(
                            EffectGraphValue::DmxAddress(*value),
                        ));
                        cx.notify();
                    }
                })
                .detach();

                field.into()
            }),
            EffectGraphControl::DmxValue => ControlView::new(cx, |cx| {
                let field = cx.new(|cx| {
                    let value: FloatingDmxValue = value.try_into().expect(
                        "should convert initial input value to the value used by it's control",
                    );
                    let mut field =
                        NumberField::<FloatingDmxValue>::new(id, cx.focus_handle(), window, cx);
                    field.set_value(value, cx);
                    field
                });

                cx.subscribe(&field, |_, _, event: &FieldEvent<FloatingDmxValue>, cx| {
                    if let FieldEvent::Change(value) = event {
                        cx.emit(ControlEvent::<EffectGraphDef>::Change(
                            EffectGraphValue::DmxValue(value.clone()),
                        ));
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
pub struct EffectGraphDef;

impl flow::GraphDef for EffectGraphDef {
    type ProcessingState = EffectGraphState;
    type Value = EffectGraphValue;
    type DataType = EffectGraphDataType;
    type Control = EffectGraphControl;
}

pub type EffectGraph = Graph<EffectGraphDef>;

pub fn insert_templates(graph: &mut EffectGraph) {
    #[rustfmt::skip]
    graph.add_templates([
        Template::new(
            "out_set_dmx_address",
            "Set DMX Address",
            vec![
                Input::new("address", "Address", EffectGraphValue::DmxAddress(Default::default()), EffectGraphControl::DmxAddress),
                Input::new("value", "Value", EffectGraphValue::DmxValue(Default::default()), EffectGraphControl::DmxValue),
            ],
            vec![],
            vec![],
            Box::new(|iv, _cv, _ov, pcx: &mut ProcessingContext<EffectGraphDef>| {
                let address = iv.value("address").expect("should get address");
                let value = iv.value("value").expect("should get value");

                let Some(EffectGraphValue::DmxAddress(address)) =
                    address.cast_to(&EffectGraphDataType::DmxAddress)
                    else {
                        panic!()
                    };

                let Some(EffectGraphValue::DmxValue(value)) =
                    value.cast_to(&EffectGraphDataType::DmxValue)
                    else {
                        panic!()
                    };

                pcx.multiverse.set_value(&address, value.into());
            }),
    )]);
}
