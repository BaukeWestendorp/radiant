use flow::{Input, Output, ProcessingContext, Template};

use super::{Control, DataType, Def, EffectGraph, Value};

pub fn insert_templates(graph: &mut EffectGraph) {
    insert_arithmetic(graph);
    insert_comparison(graph);
    insert_logic(graph);
    insert_trig(graph);
    insert_clamp_and_range(graph);
    insert_misc(graph);
}

fn insert_arithmetic(graph: &mut EffectGraph) {
    macro_rules! generate_arithmetic {
        ($id:expr, $label:expr, $operation_fn:expr) => {
            Template::new($id, $label, |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
                let a: f32 = iv.inner_value("a", &DataType::Float);
                let b: f32 = iv.inner_value("b", &DataType::Float);
                ov.set_value("c", Value::Float($operation_fn(a, b)));
            })
            .add_input(Input::new("a", "A", Value::Float(Default::default()), Control::Float))
            .add_input(Input::new("b", "B", Value::Float(Default::default()), Control::Float))
            .add_output(Output::new("c", "C", DataType::Float))
        };
    }

    let add = generate_arithmetic!("add", "Add", |a, b| a + b);
    let sub = generate_arithmetic!("sub", "Subtract", |a, b| a - b);
    let mul = generate_arithmetic!("mul", "Multiply", |a, b| a * b);
    let div = generate_arithmetic!("div", "Divide", |a, b| a / b);
    let r#mod = generate_arithmetic!("mod", "Modulo", |a, b| a % b);
    let pow = generate_arithmetic!("pow", "Power", |a: f32, b| a.powf(b));

    graph.add_templates([add, sub, mul, div, r#mod, pow]);
}

fn insert_comparison(graph: &mut EffectGraph) {
    macro_rules! generate {
        ($id:expr, $label:expr, $comparison_fn:expr) => {
            Template::new($id, $label, |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
                let a: f32 = iv.inner_value("a", &DataType::Float);
                let b: f32 = iv.inner_value("b", &DataType::Float);
                ov.set_value("c", Value::Bool($comparison_fn(a, b)));
            })
            .add_input(Input::new("a", "A", Value::Float(Default::default()), Control::Float))
            .add_input(Input::new("b", "B", Value::Float(Default::default()), Control::Float))
            .add_output(Output::new("c", "C", DataType::Bool))
        };
    }
    graph.add_templates([
        generate!("eq", "Equal", |a, b| a == b),
        generate!("lt", "Less Than", |a, b| a < b),
        generate!("gt", "Greater Than", |a, b| a > b),
        generate!("le", "Less Than or Equal", |a, b| a <= b),
        generate!("ge", "Greater Than or Equal", |a, b| a >= b),
        generate!("ne", "Not Equal", |a, b| a != b),
    ]);
}

fn insert_logic(graph: &mut EffectGraph) {
    macro_rules! generate_binary {
        ($id:expr, $label:expr, $logic_fn:expr) => {
            Template::new($id, $label, |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
                let a: bool = iv.inner_value("a", &DataType::Bool);
                let b: bool = iv.inner_value("b", &DataType::Bool);
                ov.set_value("c", Value::Bool($logic_fn(a, b)));
            })
            .add_input(Input::new("a", "A", Value::Bool(Default::default()), Control::Bool))
            .add_input(Input::new("b", "B", Value::Bool(Default::default()), Control::Bool))
            .add_output(Output::new("c", "C", DataType::Bool))
        };
    }

    graph.add_templates([
        generate_binary!("and", "And", |a, b| a && b),
        generate_binary!("or", "Or", |a, b| a || b),
        generate_binary!("xor", "Xor", |a, b| a ^ b),
        Template::new("not", "Not", |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let a: bool = iv.inner_value("a", &DataType::Bool);
            ov.set_value("b", Value::Bool(!a));
        })
        .add_input(Input::new("a", "B", Value::Bool(Default::default()), Control::Bool))
        .add_output(Output::new("b", "B", DataType::Bool)),
    ]);
}

fn insert_trig(graph: &mut EffectGraph) {
    macro_rules! generate_trig {
        ($id:expr, $label:expr, $trig_fn:expr) => {
            Template::new($id, $label, |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
                let x: f32 = iv.inner_value("x", &DataType::Float);
                ov.set_value("y", Value::Float($trig_fn(x)));
            })
            .add_input(Input::new("x", "X", Value::Float(Default::default()), Control::Float))
            .add_output(Output::new("y", "Y", DataType::Float))
        };
    }

    let rad_to_deg = Template::new(
        "rad_to_deg",
        "Radians to Degrees",
        |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let radians: f32 = iv.inner_value("radians", &DataType::Float);
            ov.set_value("degrees", Value::Float(radians.to_degrees()));
        },
    )
    .add_input(Input::new("radians", "Radians", Value::Float(Default::default()), Control::Float))
    .add_output(Output::new("degrees", "Degrees", DataType::Float));

    let deg_to_rad = Template::new(
        "deg_to_rad",
        "Degrees to Radians",
        |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let degrees: f32 = iv.inner_value("degrees", &DataType::Float);
            ov.set_value("radians", Value::Float(degrees.to_radians()));
        },
    )
    .add_input(Input::new("degrees", "Degrees", Value::Float(Default::default()), Control::Float))
    .add_output(Output::new("radians", "Radians", DataType::Float));

    graph.add_templates([
        generate_trig!("sin", "Sine", |x: f32| x.sin()),
        generate_trig!("cos", "Cosine", |x: f32| x.cos()),
        generate_trig!("tan", "Tangent", |x: f32| x.tan()),
        generate_trig!("arcsin", "Arcsin", |x: f32| x.asin()),
        generate_trig!("arccos", "Arccos", |x: f32| x.acos()),
        generate_trig!("arctan", "Arctan", |x: f32| x.atan()),
        rad_to_deg,
        deg_to_rad,
    ]);
}

fn insert_clamp_and_range(graph: &mut EffectGraph) {
    let clamp =
        Template::new("clamp", "Clamp", |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let x: f32 = iv.inner_value("x", &DataType::Float);
            let min: f32 = iv.inner_value("min", &DataType::Float);
            let max: f32 = iv.inner_value("max", &DataType::Float);
            ov.set_value("result", Value::Float(x.clamp(min, max)));
        })
        .add_input(Input::new("x", "X", Value::Float(Default::default()), Control::Float))
        .add_input(Input::new("min", "Min", Value::Float(0.0), Control::Float))
        .add_input(Input::new("max", "Max", Value::Float(1.0), Control::Float))
        .add_output(Output::new("result", "Result", DataType::Float));

    let saturate = Template::new(
        "saturate",
        "Saturate (0-1)",
        |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let x: f32 = iv.inner_value("x", &DataType::Float);
            ov.set_value("result", Value::Float(x.clamp(0.0, 1.0)));
        },
    )
    .add_input(Input::new("x", "X", Value::Float(Default::default()), Control::Float))
    .add_output(Output::new("result", "Result", DataType::Float));

    let remap =
        Template::new("remap", "Remap", |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let x: f32 = iv.inner_value("x", &DataType::Float);
            let in_min: f32 = iv.inner_value("in_min", &DataType::Float);
            let in_max: f32 = iv.inner_value("in_max", &DataType::Float);
            let out_min: f32 = iv.inner_value("out_min", &DataType::Float);
            let out_max: f32 = iv.inner_value("out_max", &DataType::Float);
            let t = (x - in_min) / (in_max - in_min);
            let result = out_min + t * (out_max - out_min);
            ov.set_value("result", Value::Float(result));
        })
        .add_input(Input::new("x", "X", Value::Float(Default::default()), Control::Float))
        .add_input(Input::new("in_min", "In Min", Value::Float(0.0), Control::Float))
        .add_input(Input::new("in_max", "In Max", Value::Float(100.0), Control::Float))
        .add_input(Input::new("out_min", "Out Min", Value::Float(0.0), Control::Float))
        .add_input(Input::new("out_max", "Out Max", Value::Float(1.0), Control::Float))
        .add_output(Output::new("result", "Result", DataType::Float));

    let step =
        Template::new("step", "Step", |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let edge: f32 = iv.inner_value("edge", &DataType::Float);
            let x: f32 = iv.inner_value("x", &DataType::Float);
            ov.set_value("result", Value::Float(if x < edge { 0.0 } else { 1.0 }));
        })
        .add_input(Input::new("x", "X", Value::Float(Default::default()), Control::Float))
        .add_input(Input::new("edge", "Edge", Value::Float(0.5), Control::Float))
        .add_output(Output::new("result", "Result", DataType::Float));

    let smoothstep = Template::new(
        "smoothstep",
        "Smoothstep",
        |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let x: f32 = iv.inner_value("x", &DataType::Float);
            let edge0: f32 = iv.inner_value("edge0", &DataType::Float);
            let edge1: f32 = iv.inner_value("edge1", &DataType::Float);
            let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
            let result = t * t * (3.0 - 2.0 * t);
            ov.set_value("result", Value::Float(result));
        },
    )
    .add_input(Input::new("x", "X", Value::Float(Default::default()), Control::Float))
    .add_input(Input::new("edge0", "Edge 0", Value::Float(0.0), Control::Float))
    .add_input(Input::new("edge1", "Edge 1", Value::Float(1.0), Control::Float))
    .add_output(Output::new("result", "Result", DataType::Float));

    graph.add_templates([clamp, saturate, remap, step, smoothstep]);
}

fn insert_misc(graph: &mut EffectGraph) {
    let min = Template::new("min", "Min", |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
        let a: f32 = iv.inner_value("a", &DataType::Float);
        let b: f32 = iv.inner_value("b", &DataType::Float);
        ov.set_value("result", Value::Float(a.min(b)));
    })
    .add_input(Input::new("a", "A", Value::Float(Default::default()), Control::Float))
    .add_input(Input::new("b", "B", Value::Float(Default::default()), Control::Float))
    .add_output(Output::new("result", "Result", DataType::Float));

    let max = Template::new("max", "Max", |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
        let a: f32 = iv.inner_value("a", &DataType::Float);
        let b: f32 = iv.inner_value("b", &DataType::Float);
        ov.set_value("result", Value::Float(a.max(b)));
    })
    .add_input(Input::new("a", "A", Value::Float(Default::default()), Control::Float))
    .add_input(Input::new("b", "B", Value::Float(Default::default()), Control::Float))
    .add_output(Output::new("result", "Result", DataType::Float));

    let abs = Template::new(
        "abs",
        "Absolute Value",
        |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let x: f32 = iv.inner_value("x", &DataType::Float);
            ov.set_value("result", Value::Float(x.abs()));
        },
    )
    .add_input(Input::new("x", "X", Value::Float(Default::default()), Control::Float))
    .add_output(Output::new("result", "Result", DataType::Float));

    let floor =
        Template::new("floor", "Floor", |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let x: f32 = iv.inner_value("x", &DataType::Float);
            ov.set_value("result", Value::Float(x.floor()));
        })
        .add_input(Input::new("x", "X", Value::Float(Default::default()), Control::Float))
        .add_output(Output::new("result", "Result", DataType::Float));

    let ceil =
        Template::new("ceil", "Ceil", |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let x: f32 = iv.inner_value("x", &DataType::Float);
            ov.set_value("result", Value::Float(x.ceil()));
        })
        .add_input(Input::new("x", "X", Value::Float(Default::default()), Control::Float))
        .add_output(Output::new("result", "Result", DataType::Float));

    let round =
        Template::new("round", "Round", |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let x: f32 = iv.inner_value("x", &DataType::Float);
            ov.set_value("result", Value::Float(x.round()));
        })
        .add_input(Input::new("x", "X", Value::Float(Default::default()), Control::Float))
        .add_output(Output::new("result", "Result", DataType::Float));

    let fract = Template::new(
        "fract",
        "Fractional Part",
        |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let x: f32 = iv.inner_value("x", &DataType::Float);
            ov.set_value("result", Value::Float(x.fract()));
        },
    )
    .add_input(Input::new("x", "X", Value::Float(Default::default()), Control::Float))
    .add_output(Output::new("result", "Result", DataType::Float));

    let sign =
        Template::new("sign", "Sign", |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let x: f32 = iv.inner_value("x", &DataType::Float);
            ov.set_value("result", Value::Float(x.signum()));
        })
        .add_input(Input::new("x", "X", Value::Float(Default::default()), Control::Float))
        .add_output(Output::new("result", "Result", DataType::Float));

    let mix = Template::new(
        "mix",
        "Mix (Lerp)",
        |iv, _cv, ov, _pcx: &mut ProcessingContext<Def>, _cx| {
            let a: f32 = iv.inner_value("a", &DataType::Float);
            let b: f32 = iv.inner_value("b", &DataType::Float);
            let t: f32 = iv.inner_value("t", &DataType::Float);
            ov.set_value("result", Value::Float(a + t * (b - a)));
        },
    )
    .add_input(Input::new("a", "A", Value::Float(Default::default()), Control::Float))
    .add_input(Input::new("b", "B", Value::Float(Default::default()), Control::Float))
    .add_input(Input::new("t", "T", Value::Float(Default::default()), Control::Float))
    .add_output(Output::new("result", "Result", DataType::Float));

    graph.add_templates([min, max, abs, floor, ceil, round, fract, sign, mix]);
}
