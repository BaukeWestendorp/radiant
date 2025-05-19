use flow::{Input, ProcessingContext, Template};

use crate::show::{
    FloatingDmxValue,
    asset::effect_graph::{Control, DataType, Def, EffectGraph, Value},
    attr::AnyPresetAssetId,
    patch::FixtureId,
};

pub fn insert_templates(graph: &mut EffectGraph) {
    insert_static_modification(graph);
    insert_dynamic_modifications(graph);
}

fn insert_static_modification(graph: &mut EffectGraph) {
    let set_addr = Template::new(
        "pipeline_set_addr",
        "Set DMX Value at Address",
        |iv, _cv, _ov, pcx: &mut ProcessingContext<Def>, cx| {
            let value: FloatingDmxValue = iv.inner_value("value", &DataType::DmxValue);
            let address: dmx::Address = iv.inner_value("address", &DataType::DmxAddress);
            pcx.pipeline().update(cx, |pipeline, _cx| {
                pipeline.apply_value(address, value.into());
            });
        },
    )
    .add_input(Input::new("value", "Value", Value::DmxValue(Default::default()), Control::DmxValue))
    .add_input(Input::new(
        "address",
        "Address",
        Value::DmxAddress(Default::default()),
        Control::DmxAddress,
    ));

    graph.add_templates([set_addr]);
}

fn insert_dynamic_modifications(graph: &mut EffectGraph) {
    let apply_preset = Template::new(
        "pipeline_apply_preset",
        "Apply Preset",
        |iv, _cv, _ov, pcx: &mut ProcessingContext<Def>, cx| {
            let Some(preset_id) =
                iv.inner_value::<Option<AnyPresetAssetId>>("preset", &DataType::Preset)
            else {
                return;
            };

            let fixture_id: FixtureId = iv.inner_value("fixture_id", &DataType::FixtureId);

            pcx.pipeline().update(cx, |pipeline, cx| {
                pipeline.apply_preset(preset_id, fixture_id, cx);
            });
        },
    )
    .add_input(Input::new("preset", "Preset", Value::Preset(Default::default()), Control::Preset))
    .add_input(Input::new(
        "fixture_id",
        "FixtureId",
        Value::FixtureId(Default::default()),
        Control::FixtureId,
    ));

    graph.add_templates([apply_preset]);
}
