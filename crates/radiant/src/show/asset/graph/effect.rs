use crate::{
    show::{
        FloatingDmxValue,
        asset::{AnyPresetId, Asset, FixtureGroup},
        patch::FixtureId,
    },
    ui::input::{PresetSelector, PresetSelectorEvent},
};
use flow::{
    Graph, ProcessingContext,
    gpui::{ControlEvent, ControlView},
};
use gpui::{App, ElementId, Entity, Window, prelude::*};
use ui::{Field, FieldEvent, NumberField};

#[derive(Debug, Clone, flow::Value)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(graph_def = EffectGraphDef, data_type = EffectGraphDataType)]
pub enum EffectGraphValue {
    #[value(color = 0x1BD5FF)]
    DmxAddress(dmx::Address),
    #[value(color = 0x1361FF)]
    DmxValue(FloatingDmxValue),
    #[value(color = 0xffff00)]
    FixtureId(FixtureId),
    #[value(color = 0xff0000)]
    Preset(Option<AnyPresetId>),
}

#[derive(Debug, Clone)]
pub struct EffectGraphState {
    pub multiverse: Entity<dmx::Multiverse>,
    pub fixture_group: Entity<Asset<FixtureGroup>>,
    pub fixture_id_index: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EffectGraphControl {
    DmxAddress,
    DmxValue,
    FixtureId,
    Preset,
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
                            EffectGraphValue::DmxValue(*value),
                        ));
                        cx.notify();
                    }
                })
                .detach();

                field.into()
            }),
            EffectGraphControl::FixtureId => ControlView::new(cx, |cx| {
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
                        cx.emit(ControlEvent::<EffectGraphDef>::Change(
                            EffectGraphValue::FixtureId(*value),
                        ));
                        cx.notify();
                    }
                })
                .detach();

                field.into()
            }),
            EffectGraphControl::Preset => ControlView::new(cx, |cx| {
                let preset_selector = cx.new(|cx| {
                    let value: Option<AnyPresetId> = value.try_into().expect(
                        "should convert initial input value to the value used by it's control",
                    );
                    let mut field = PresetSelector::new(id, cx.focus_handle(), window, cx);
                    field.set_value(value, cx);
                    field
                });

                cx.subscribe(&preset_selector, |_, _, event: &PresetSelectorEvent, cx| {
                    let PresetSelectorEvent::Change(value) = event;
                    cx.emit(ControlEvent::<EffectGraphDef>::Change(EffectGraphValue::Preset(
                        *value,
                    )));
                    cx.notify();
                })
                .detach();

                preset_selector.into()
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
    // graph.add_templates([
    //     Template::new(
    //         "out_set_dmx_address",
    //         "Set DMX Address",
    //         vec![
    //             Input::new(
    //                 "address",
    //                 "Address",
    //                 EffectGraphValue::DmxAddress(Default::default()),
    //                 EffectGraphControl::DmxAddress,
    //             ),
    //             Input::new(
    //                 "value",
    //                 "Value",
    //                 EffectGraphValue::DmxValue(Default::default()),
    //                 EffectGraphControl::DmxValue,
    //             ),
    //         ],
    //         vec![],
    //         vec![],
    //         Box::new(|iv, _cv, _ov, pcx: &mut ProcessingContext<EffectGraphDef>, cx| {
    //             // Extract address and value from inputs
    //             let address = iv
    //                 .value("address")
    //                 .and_then(|a| a.cast_to(&EffectGraphDataType::DmxAddress))
    //                 .and_then(
    //                     |a| if let EffectGraphValue::DmxAddress(a) = a { Some(a) } else { None },
    //                 )
    //                 .expect("Invalid DMX address");

    //             let value = iv
    //                 .value("value")
    //                 .and_then(|v| v.cast_to(&EffectGraphDataType::DmxValue))
    //                 .and_then(
    //                     |v| if let EffectGraphValue::DmxValue(v) = v { Some(v) } else { None },
    //                 )
    //                 .expect("Invalid DMX value");

    //             pcx.multiverse.update(cx, |multiverse, cx| {
    //                 multiverse.set_value(&address, value.into());
    //                 cx.notify();
    //             })
    //         }),
    //     ),
    //     Template::new(
    //         "get_current_fixture",
    //         "Current Fixture",
    //         vec![],
    //         vec![
    //             Output::new("fixture_id", "Fixture Id", EffectGraphDataType::FixtureId),
    //             Output::new("address", "Address", EffectGraphDataType::DmxAddress),
    //         ],
    //         vec![],
    //         Box::new(|_iv, _cv, ov, pcx: &mut ProcessingContext<EffectGraphDef>, cx| {
    //             let fixture_group = &pcx.fixture_group.read(cx).data;

    //             let fixture_id = pcx
    //                 .fixture_id_index
    //                 .and_then(|ix| fixture_group.fixtures.get(ix))
    //                 .copied()
    //                 .unwrap_or_else(|| panic!("No fixture selected"));

    //             let patch = Show::global(cx).patch.read(cx);
    //             let fixture = patch.fixture(fixture_id).expect("Fixture not found in patch");

    //             ov.set_value("fixture_id", EffectGraphValue::FixtureId(fixture_id));
    //             ov.set_value("address", EffectGraphValue::DmxAddress(*fixture.address()));
    //         }),
    //     ),
    //     Template::new(
    //         "apply_preset",
    //         "Apply Preset",
    //         vec![Input::new(
    //             "preset",
    //             "Preset",
    //             EffectGraphValue::Preset(Default::default()),
    //             EffectGraphControl::Preset,
    //         )],
    //         vec![],
    //         vec![],
    //         Box::new(|_iv, _cv, _ov, _pcx: &mut ProcessingContext<EffectGraphDef>, _cx| {}),
    //     ),
    // ]);
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
