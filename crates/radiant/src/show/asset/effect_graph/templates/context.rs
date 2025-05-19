use std::time::{SystemTime, UNIX_EPOCH};

use flow::{Input, Output, ProcessingContext, Template};

use crate::show::asset::effect_graph::{Control, DataType, Def, EffectGraph, Value};

pub fn insert_templates(graph: &mut EffectGraph) {
    insert_time(graph);
    insert_fixture_info(graph);
}

fn insert_time(graph: &mut EffectGraph) {
    let time =
        Template::new("cx_time", "Time", |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let scale: f64 = iv.inner_value("scale", &DataType::Float);
            let scaled_unix_epoch =
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs_f64()
                    * scale;

            ov.set_value("unix_epoch", Value::Float(scaled_unix_epoch));

            let unix_fract = scaled_unix_epoch.fract();
            ov.set_value("unix_fract", Value::Float(unix_fract));

            let sin = scaled_unix_epoch.sin();
            ov.set_value("sin", Value::Float(sin));

            let cos = scaled_unix_epoch.cos();
            ov.set_value("cos", Value::Float(cos));

            let ping_pong = ((scaled_unix_epoch % 2.0) - 1.0).abs();
            ov.set_value("ping_pong", Value::Float(ping_pong));
        })
        .add_input(Input::new("scale", "Time Scale", Value::Float(1.0), Control::Float))
        .add_output(Output::new("unix_epoch", "UNIX Epoch", DataType::Float))
        .add_output(Output::new("unix_fract", "Fraction", DataType::Float))
        .add_output(Output::new("sin", "Sin", DataType::Float))
        .add_output(Output::new("cos", "Cos", DataType::Float))
        .add_output(Output::new("ping_pong", "Ping Pong", DataType::Float));

    graph.add_templates([time]);
}

fn insert_fixture_info(graph: &mut EffectGraph) {
    let fixture_group = Template::new(
        "cx_fixture_group",
        "Fixture Group",
        |_iv, _cv, ov, pcx: &mut ProcessingContext<Def>, _cx| {
            let total_fixtures = pcx.group_len();
            ov.set_value("total_fixtures", Value::Float(total_fixtures as f64));
        },
    )
    .add_output(Output::new("total_fixtures", "Total Fixtures", DataType::Float));

    let current_fixture = Template::new(
        "cx_current_fixture",
        "Current Fixture",
        |_iv, _cv, ov, pcx: &mut ProcessingContext<Def>, cx| {
            let address = pcx.fixture(cx).address().clone();
            ov.set_value("address", Value::DmxAddress(address));

            let id = pcx.fixture_id();
            ov.set_value("id", Value::FixtureId(id));

            let index = pcx.group_index();
            ov.set_value("index", Value::Float(index as f64));

            let progress = pcx.group_index() as f64 / pcx.group_len() as f64;
            ov.set_value("progress", Value::Float(progress));
        },
    )
    .add_output(Output::new("address", "Address", DataType::DmxAddress))
    .add_output(Output::new("id", "FixtureId", DataType::FixtureId))
    .add_output(Output::new("index", "Index", DataType::Float))
    .add_output(Output::new("progress", "Progress", DataType::Float));

    graph.add_templates([fixture_group, current_fixture]);
}
