use flow::{NodeControl, Output, ProcessingContext, Template};

use crate::show::{
    asset::effect_graph::{Control, DataType, Def, EffectGraph, Value},
    attr::AnyPresetAssetId,
};

pub fn insert_templates(graph: &mut EffectGraph) {
    insert_presets(graph);
}

fn insert_presets(graph: &mut EffectGraph) {
    let get_preset = Template::new(
        "get_preset",
        "Get Preset",
        |_iv, cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let preset_id: Option<AnyPresetAssetId> = cv.inner_value("preset", &DataType::Preset);
            ov.set_value("preset", Value::Preset(preset_id));
        },
    )
    .add_control(NodeControl::new(
        "preset",
        "Preset",
        Value::Preset(Default::default()),
        Control::Preset,
    ))
    .add_output(Output::new("preset", "Preset", DataType::Preset));

    graph.add_templates([get_preset]);
}
