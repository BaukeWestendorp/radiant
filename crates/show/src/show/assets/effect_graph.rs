use flow::{
    Graph, Input, ProcessingContext, Template, Value as _,
    gpui::{ControlEvent, ControlView},
};
use gpui::*;
use ui::{DmxAddressField, NumberField, TextInputEvent};

use crate::define_asset;

define_asset!(EffectGraph, EffectGraphAsset, EffectGraphId);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FloatingDmxValue(pub f64);

impl From<FloatingDmxValue> for dmx::Value {
    fn from(value: FloatingDmxValue) -> Self {
        dmx::Value((value.0 * (u8::MAX as f64)) as u8)
    }
}

#[derive(Debug, Clone, flow::Value)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(graph_def = EffectGraphDef, data_type = EffectGraphDataType)]
pub enum EffectGraphValue {
    #[value(color = 0xCE39FF)]
    #[cast(
        target = DmxValue,
        map = |value: &f64| {
            let value = value.clamp(dmx::Value::MIN.0 as f64, dmx::Value::MAX.0 as f64);
            FloatingDmxValue(value)
        }
    )]
    Number(f64),

    #[value(color = 0x1BD5FF)]
    DmxAddress(dmx::Address),

    #[value(color = 0x1361FF)]
    #[cast(
        target = Number,
        map = |value: &FloatingDmxValue| value.0
    )]
    DmxValue(FloatingDmxValue),
}

#[derive(Debug, Clone, Default)]
pub struct EffectGraphState {
    pub multiverse: dmx::Multiverse,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EffectGraphControl {
    Slider { min: f64, max: f64, step: Option<f64> },
    Float,
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
            EffectGraphControl::Slider { min, max, step } => ControlView::new(cx, |cx| {
                let field = cx.new(|cx| {
                    let value = value.cast_to(&EffectGraphDataType::Number)
                        .expect("should always be able to cast initial value to EffectGraphDataType::Number")
                        .try_into().expect("should always be able to convert initial input value to the value used by it's control");

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
            }),
            EffectGraphControl::Float => ControlView::new(cx, |cx| {
                let field = cx.new(|cx| {
                    let value = value.try_into().expect(
                        "should convert initial input value to the value used by it's control",
                    );

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
            }),
            EffectGraphControl::DmxAddress => ControlView::new(cx, |cx| {
                let field = cx.new(|cx| {
                    let value = value.try_into().expect(
                        "should convert initial input value to the value used by it's control",
                    );

                    let mut field = DmxAddressField::new(id, window, cx);
                    field.set_value(value, cx);
                    field
                });

                cx.subscribe(&field, |_, field, event: &TextInputEvent, cx| {
                    if let TextInputEvent::Change(_) = event {
                        let value = field.read(cx).value(cx);
                        cx.emit(ControlEvent::<EffectGraphDef>::Change(
                            EffectGraphValue::DmxAddress(value),
                        ));
                        cx.notify();
                    }
                })
                .detach();

                field.into()
            }),
            EffectGraphControl::DmxValue => Self::Slider { min: 0.0, max: 1.0_f64, step: None }
                .view(value, id, window, cx),
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
    graph.add_templates([Template::new(
        "out_set_dmx_address",
        "Set DMX Address",
        vec![
            Input::new(
                "address",
                "Address",
                EffectGraphValue::DmxAddress(Default::default()),
                EffectGraphControl::DmxAddress,
            ),
            Input::new(
                "value",
                "Value",
                EffectGraphValue::DmxValue(Default::default()),
                EffectGraphControl::DmxValue,
            ),
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
