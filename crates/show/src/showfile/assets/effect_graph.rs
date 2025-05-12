use flow::{
    Graph, Input, Output, ProcessingContext, Template, Value as _,
    gpui::{ControlEvent, ControlView},
};
use gpui::{App, ElementId, Entity, ReadGlobal, SharedString, Window, prelude::*};
use ui::{Field, FieldEvent, NumberField, NumberFieldImpl};

use crate::{
    assets::{Asset, FixtureGroup},
    patch::FixtureId,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FloatingDmxValue(pub f32);

impl From<FloatingDmxValue> for dmx::Value {
    fn from(value: FloatingDmxValue) -> Self {
        dmx::Value((value.0 * (u8::MAX as f32)).clamp(0.0, 1.0) as u8)
    }
}

impl NumberFieldImpl for FloatingDmxValue {
    type Value = Self;

    const MIN: Option<Self> = Some(FloatingDmxValue(0.0));
    const MAX: Option<Self> = Some(FloatingDmxValue(1.0));
    const STEP: Option<f32> = None;

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
#[value(graph_def = EffectGraphDef, data_type = DataType)]
pub enum Value {
    #[value(color = 0x1BD5FF)]
    DmxAddress(dmx::Address),

    #[value(color = 0x1361FF)]
    DmxValue(FloatingDmxValue),

    #[value(color = 0xffff00)]
    FixtureId(FixtureId),
}

#[derive(Debug, Clone)]
pub struct State {
    pub multiverse: Entity<dmx::Multiverse>,
    pub fixture_group: Entity<Asset<FixtureGroup>>,
    pub fixture_id_index: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Control {
    DmxAddress,
    DmxValue,
    FixtureId,
}

impl flow::Control<EffectGraphDef> for Control {
    fn view(
        &self,
        value: Value,
        id: ElementId,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<ControlView> {
        match self {
            Control::DmxAddress => ControlView::new(cx, |cx| {
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
                        cx.emit(ControlEvent::<EffectGraphDef>::Change(Value::DmxAddress(*value)));
                        cx.notify();
                    }
                })
                .detach();

                field.into()
            }),
            Control::DmxValue => ControlView::new(cx, |cx| {
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
                        cx.emit(ControlEvent::<EffectGraphDef>::Change(Value::DmxValue(
                            *value,
                        )));
                        cx.notify();
                    }
                })
                .detach();

                field.into()
            }),
            Control::FixtureId => ControlView::new(cx, |cx| {
                let field = cx.new(|cx| {
                    let value: FixtureId = value.try_into().expect(
                        "should convert initial input value to the value used by it's control",
                    );
                    let field = Field::<FixtureId>::new(id, cx.focus_handle(), window, cx);
                    field.set_value(&value, cx);
                    field
                });

                cx.subscribe(&field, |_, _, event: &FieldEvent<FixtureId>, cx| {
                    if let FieldEvent::Change(value) = event {
                        cx.emit(ControlEvent::<EffectGraphDef>::Change(Value::FixtureId(*value)));
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
    type ProcessingState = State;
    type Value = Value;
    type DataType = DataType;
    type Control = Control;
}

pub type EffectGraph = Graph<EffectGraphDef>;

pub fn insert_templates(graph: &mut EffectGraph) {
    graph.add_templates([
        Template::new(
            "out_set_dmx_address",
            "Set DMX Address",
            vec![
                Input::new(
                    "address",
                    "Address",
                    Value::DmxAddress(Default::default()),
                    Control::DmxAddress,
                ),
                Input::new(
                    "value",
                    "Value",
                    Value::DmxValue(Default::default()),
                    Control::DmxValue,
                ),
            ],
            vec![],
            vec![],
            Box::new(|iv, _cv, _ov, pcx: &mut ProcessingContext<EffectGraphDef>, cx| {
                // Extract address and value from inputs
                let address = iv
                    .value("address")
                    .and_then(|a| a.cast_to(&DataType::DmxAddress))
                    .and_then(|a| if let Value::DmxAddress(a) = a { Some(a) } else { None })
                    .expect("Invalid DMX address");

                let value = iv
                    .value("value")
                    .and_then(|v| v.cast_to(&DataType::DmxValue))
                    .and_then(|v| if let Value::DmxValue(v) = v { Some(v) } else { None })
                    .expect("Invalid DMX value");

                pcx.multiverse.update(cx, |multiverse, cx| {
                    multiverse.set_value(&address, value.into());
                    cx.notify();
                })
            }),
        ),
        Template::new(
            "get_current_fixture",
            "Current Fixture",
            vec![],
            vec![
                Output::new("fixture_id", "Fixture Id", DataType::FixtureId),
                Output::new("address", "Address", DataType::DmxAddress),
            ],
            vec![],
            Box::new(|_iv, _cv, ov, pcx: &mut ProcessingContext<EffectGraphDef>, cx| {
                let fixture_group = &pcx.fixture_group.read(cx).data;

                let fixture_id = pcx
                    .fixture_id_index
                    .and_then(|ix| fixture_group.fixtures.get(ix))
                    .copied()
                    .unwrap_or_else(|| panic!("No fixture selected"));

                let patch = crate::Show::global(cx).patch.read(cx);
                let fixture = patch.fixture(fixture_id).expect("Fixture not found in patch");

                ov.set_value("fixture_id", Value::FixtureId(fixture_id));
                ov.set_value("address", Value::DmxAddress(*fixture.address()));
            }),
        ),
        Template::new(
            "set_gdtf_attr_dimmer",
            "Set Attributes",
            vec![
                Input::new(
                    "fixture_id",
                    "Fixture Id",
                    Value::FixtureId(Default::default()),
                    Control::FixtureId,
                ),
                Input::new(
                    "dimmer",
                    "Dimmer",
                    Value::DmxValue(Default::default()),
                    Control::DmxValue,
                ),
                Input::new("pan", "Pan", Value::DmxValue(Default::default()), Control::DmxValue),
                Input::new("tilt", "Tilt", Value::DmxValue(Default::default()), Control::DmxValue),
                Input::new("red", "Red", Value::DmxValue(Default::default()), Control::DmxValue),
                Input::new(
                    "green",
                    "Green",
                    Value::DmxValue(Default::default()),
                    Control::DmxValue,
                ),
                Input::new("blue", "Blue", Value::DmxValue(Default::default()), Control::DmxValue),
            ],
            vec![],
            vec![],
            Box::new(|iv, _cv, _ov, pcx: &mut ProcessingContext<EffectGraphDef>, cx| {
                let fixture_id = iv
                    .value("fixture_id")
                    .and_then(|id| id.cast_to(&DataType::FixtureId))
                    .and_then(|id| if let Value::FixtureId(id) = id { Some(id) } else { None })
                    .expect("Invalid fixture ID");

                let dimmer = iv
                    .value("dimmer")
                    .and_then(|v| v.cast_to(&DataType::DmxValue))
                    .and_then(|v| if let Value::DmxValue(v) = v { Some(v) } else { None })
                    .expect("Invalid DMX value for dimmer");

                let pan = iv
                    .value("pan")
                    .and_then(|v| v.cast_to(&DataType::DmxValue))
                    .and_then(|v| if let Value::DmxValue(v) = v { Some(v) } else { None })
                    .expect("Invalid DMX value for pan");

                let tilt = iv
                    .value("tilt")
                    .and_then(|v| v.cast_to(&DataType::DmxValue))
                    .and_then(|v| if let Value::DmxValue(v) = v { Some(v) } else { None })
                    .expect("Invalid DMX value for tilt");

                let red = iv
                    .value("red")
                    .and_then(|v| v.cast_to(&DataType::DmxValue))
                    .and_then(|v| if let Value::DmxValue(v) = v { Some(v) } else { None })
                    .expect("Invalid DMX value for red");

                let green = iv
                    .value("green")
                    .and_then(|v| v.cast_to(&DataType::DmxValue))
                    .and_then(|v| if let Value::DmxValue(v) = v { Some(v) } else { None })
                    .expect("Invalid DMX value for green");

                let blue = iv
                    .value("blue")
                    .and_then(|v| v.cast_to(&DataType::DmxValue))
                    .and_then(|v| if let Value::DmxValue(v) = v { Some(v) } else { None })
                    .expect("Invalid DMX value for blue");

                let set_dmx_values_for_attribute =
                    |attr: &str, value: &FloatingDmxValue, cx: &mut App| {
                        let patch = &crate::Show::global(cx).patch.read(cx);
                        let Some(fixture) = patch.fixture(fixture_id) else { return };

                        let Some(dimmer_channel_offsets) =
                            fixture.channel_offset_for_attr(attr, patch).cloned()
                        else {
                            return;
                        };

                        set_dmx_value_at_offset(
                            &fixture.address().clone(),
                            dimmer_channel_offsets.as_slice(),
                            value.0,
                            pcx,
                            cx,
                        );
                    };

                set_dmx_values_for_attribute("Dimmer", &dimmer, cx);
                set_dmx_values_for_attribute("Pan", &pan, cx);
                set_dmx_values_for_attribute("Tilt", &tilt, cx);
                set_dmx_values_for_attribute("ColorRGB_Red", &red, cx);
                set_dmx_values_for_attribute("ColorRGB_Green", &green, cx);
                set_dmx_values_for_attribute("ColorRGB_Blue", &blue, cx);
            }),
        ),
    ]);
}

fn set_dmx_value_at_offset(
    start_address: &dmx::Address,
    offsets: &[i32],
    value: f32,
    pcx: &ProcessingContext<EffectGraphDef>,
    cx: &mut App,
) {
    let value_bytes = match offsets.len() {
        1 => {
            let byte_value = (value * 0xff as f32) as u8;
            vec![byte_value]
        }
        2 => {
            let int_value = (value * 0xffff as f32) as u16;
            vec![(int_value >> 8) as u8, (int_value & 0xFF) as u8]
        }
        3 => {
            let int_value = (value * 0xffffff as f32) as u32;
            vec![(int_value >> 16) as u8, ((int_value >> 8) & 0xFF) as u8, (int_value & 0xFF) as u8]
        }
        4 => {
            let int_value = (value * 0xffffffff_u32 as f32) as u32;
            vec![
                (int_value >> 24) as u8,
                ((int_value >> 16) & 0xFF) as u8,
                ((int_value >> 8) & 0xFF) as u8,
                (int_value & 0xFF) as u8,
            ]
        }
        _ => vec![0],
    };

    for (byte, offset) in value_bytes.iter().zip(offsets) {
        let address = start_address.with_channel_offset(*offset as u16 - 1).unwrap();
        pcx.multiverse.update(cx, |multiverse, cx| {
            multiverse.set_value(&address, dmx::Value(*byte));
            cx.notify();
        });
    }
}
