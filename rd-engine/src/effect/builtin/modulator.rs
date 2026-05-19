use zeevonk::AttributeName;

use crate::{OnUpdateContext, Parameter, ParameterValue};
use std::f64::consts::TAU;

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Waveform {
    Sine,
    Triangle,
    Sawtooth,
    Square { duty_cycle: f64 },
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Options {
    pub attributes: Vec<AttributeName>,
    pub waveform: Waveform,
    pub frequency_hz: f64,
    pub phase_spread_deg: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub exponent: f64,
}

pub fn on_update(mut cx: OnUpdateContext, options: &Options) {
    let base_t = cx.time_seconds() * options.frequency_hz;
    let fixture_ids = cx.fixture_ids();
    let total_fixtures = fixture_ids.len() as f64;

    for (index, fid) in fixture_ids.into_iter().enumerate() {
        let phase_offset = if total_fixtures > 1.0 {
            (index as f64 / (total_fixtures - 1.0)) * options.phase_spread_deg.to_radians()
        } else {
            0.0
        };

        let local_t = (base_t + (phase_offset / TAU)).rem_euclid(1.0);

        let mut raw_value = match &options.waveform {
            Waveform::Sine => (f64::sin(local_t * TAU) + 1.0) / 2.0,
            Waveform::Triangle => {
                if local_t < 0.5 {
                    local_t * 2.0
                } else {
                    2.0 - (local_t * 2.0)
                }
            }
            Waveform::Sawtooth => local_t,
            Waveform::Square { duty_cycle } => {
                if local_t < *duty_cycle {
                    1.0
                } else {
                    0.0
                }
            }
        };

        if !matches!(options.waveform, Waveform::Square { .. }) && options.exponent != 1.0 {
            raw_value = raw_value.powf(options.exponent);
        }

        let final_value = options.min_value + (raw_value * (options.max_value - options.min_value));

        for attribute_name in &options.attributes {
            cx.set_parameter(
                fid,
                Parameter::raw(attribute_name.clone(), ParameterValue::clamped(final_value as f32)),
            );
        }
    }
}
