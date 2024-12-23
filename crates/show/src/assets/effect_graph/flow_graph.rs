use crate::{AttributeValue, Effect, Fixture, FixtureId, Group, Patch, Show, Template};

use dmx::{DmxAddress, DmxChannel, DmxOutput, DmxUniverseId};
use flow::gpui::{ControlEvent, VisualControl, VisualDataType, VisualNodeData, VisualNodeKind};
use flow::{FlowError, Graph};
use gpui::{
    rgb, AnyView, AppContext, ElementId, EventEmitter, Hsla, Model, SharedString, ViewContext,
    VisualContext,
};
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use strum::IntoEnumIterator;
use ui::{NumberField, Slider, SliderEvent, TextField, TextFieldEvent};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphDefinition;

impl flow::GraphDefinition for GraphDefinition {
    type NodeKind = NodeKind;
    type NodeData = NodeData;
    type Value = Value;
    type DataType = DataType;
    type Control = Control;
}

pub type FlowEffectGraph = Graph<GraphDefinition>;

#[derive(
    Debug, Clone, PartialEq, strum::EnumIter, serde::Serialize, serde::Deserialize, flow::NodeKind,
)]
#[rustfmt::skip::attributes(node, input, constant_output, computed_output)]
#[node_kind(
    category = "NodeCategory",
    graph_definition = "GraphDefinition",
    processing_context = "ProcessingContext"
)]
pub enum NodeKind {
    // New Values
    #[node(name = "New Fixture Id", category = "NodeCategory::NewValue")]
    #[constant_output(label = "id", data_type = "DataType::FixtureId", control = "Control::FixtureId")]
    NewFixtureId,

    #[node(name = "New Attribute Value", category = "NodeCategory::NewValue")]
    #[constant_output(label = "value", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    NewAttributeValue,

    #[node(name = "New DMX Address", category = "NodeCategory::NewValue", processor = "processor::new_dmx_address")]
    #[input(label = "universe", data_type = "DataType::DmxUniverse", control = "Control::DmxUniverse")]
    #[input(label = "channel", data_type = "DataType::DmxChannel", control = "Control::DmxChannel")]
    #[computed_output(label = "address", data_type = "DataType::DmxAddress")]
    NewDmxAddress,

    // Math
    #[node(name = "Add", category = "NodeCategory::Math", processor = "processor::add")]
    #[input(label = "a", data_type = "DataType::Float", control = "Control::Float")]
    #[input(label = "b", data_type = "DataType::Float", control = "Control::Float")]
    #[computed_output(label = "sum", data_type = "DataType::Float")]
    Add,

    #[node(name = "Subtract", category = "NodeCategory::Math", processor = "processor::subtract")]
    #[input(label = "a", data_type = "DataType::Float", control = "Control::Float")]
    #[input(label = "b", data_type = "DataType::Float", control = "Control::Float")]
    #[computed_output(label = "difference", data_type = "DataType::Float")]
    Subtract,

    #[node(name = "Multiply", category = "NodeCategory::Math", processor = "processor::multiply")]
    #[input(label = "a", data_type = "DataType::Float", control = "Control::Float")]
    #[input(label = "b", data_type = "DataType::Float", control = "Control::Float")]
    #[computed_output(label = "product", data_type = "DataType::Float")]
    Multiply,

    #[node(name = "Divide", category = "NodeCategory::Math", processor = "processor::divide")]
    #[input(label = "a", data_type = "DataType::Float", control = "Control::Float")]
    #[input(label = "b", data_type = "DataType::Float", control = "Control::Float")]
    #[computed_output(label = "quotient", data_type = "DataType::Float")]
    Divide,

    #[node(name = "Sine", category = "NodeCategory::Math", processor = "processor::sine")]
    #[input(label = "a", data_type = "DataType::Float", control = "Control::Float")]
    #[computed_output(label = "b", data_type = "DataType::Float")]
    Sine,

    #[node(name = "Cosine", category = "NodeCategory::Math", processor = "processor::cosine")]
    #[input(label = "a", data_type = "DataType::Float", control = "Control::Float")]
    #[computed_output(label = "b", data_type = "DataType::Float")]
    Cosine,

    #[node(name = "Floor", category = "NodeCategory::Math", processor = "processor::floor")]
    #[input(label = "value", data_type = "DataType::Float", control = "Control::Float")]
    #[computed_output(label = "floored", data_type = "DataType::Int")]
    Floor,

    #[node(name = "Round", category = "NodeCategory::Math", processor = "processor::round")]
    #[input(label = "value", data_type = "DataType::Float", control = "Control::Float")]
    #[computed_output(label = "rounded", data_type = "DataType::Int")]
    Round,

    #[node(name = "Ceil", category = "NodeCategory::Math", processor = "processor::ceil")]
    #[input(label = "value", data_type = "DataType::Float", control = "Control::Float")]
    #[computed_output(label = "ceiled", data_type = "DataType::Int")]
    Ceil,

    // Context
    #[node(name = "Get Fixture", category = "NodeCategory::Context", processor = "processor::get_fixture")]
    #[computed_output(label = "index", data_type = "DataType::Int")]
    #[computed_output(label = "id", data_type = "DataType::FixtureId")]
    #[computed_output(label = "address", data_type = "DataType::DmxAddress")]
    GetFixture,

    #[node(name = "Get Group", category = "NodeCategory::Context", processor = "processor::get_group")]
    #[computed_output(label = "size", data_type = "DataType::Int")]
    GetGroup,

    // Utilities
    #[node(name = "Split Address", category = "NodeCategory::Utilities", processor = "processor::split_address")]
    #[input(label = "address", data_type = "DataType::DmxAddress", control = "Control::DmxAddress")]
    #[computed_output(label = "universe", data_type = "DataType::DmxUniverse")]
    #[computed_output(label = "channel", data_type = "DataType::DmxChannel")]
    SplitAddress,

    #[node(name = "Random Float", category = "NodeCategory::Utilities", processor = "processor::random_float")]
    #[computed_output(label = "value", data_type = "DataType::Float")]
    Random,

    #[node(name = "Time", category = "NodeCategory::Utilities", processor = "processor::time")]
    #[computed_output(label = "seconds", data_type = "DataType::Float")]
    Time,

    #[node(name = "Remap", category = "NodeCategory::Utilities", processor = "processor::remap")]
    #[input(label = "value", data_type = "DataType::Float", control = "Control::Float")]
    #[input(label = "from min", data_type = "DataType::Float", control = "Control::Float")]
    #[input(label = "from max", data_type = "DataType::Float", control = "Control::Float")]
    #[input(label = "to min", data_type = "DataType::Float", control = "Control::Float")]
    #[input(label = "to max", data_type = "DataType::Float", control = "Control::Float")]
    #[computed_output(label = "remapped", data_type = "DataType::Float")]
    Remap,

    // Output
    #[node(name = "Set Address Value", category = "NodeCategory::Output", processor = "processor::set_channel_value")]
    #[input(label = "address", data_type = "DataType::DmxAddress", control = "Control::DmxAddress")]
    #[input(label = "value", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    SetChannelValue,

    #[node(name = "Set Fixture Attribute", category = "NodeCategory::Output", processor = "processor::set_fixture_attribute")]
    #[input(label = "fixture", data_type = "DataType::FixtureId", control = "Control::FixtureId")]
    #[input(label = "attribute", data_type = "DataType::String", control = "Control::String")]
    #[input(label = "value", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    SetFixtureAttribute,

    #[node(name = "Set Dimmer", category = "NodeCategory::Output", processor = "processor::set_dimmer")]
    #[input(label = "Fixture", data_type = "DataType::FixtureId", control = "Control::FixtureId")]
    #[input(label = "dimmer", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    SetDimmer,

    #[node(name = "Set Color", category = "NodeCategory::Output", processor = "processor::set_color")]
    #[input(label = "Fixture", data_type = "DataType::FixtureId", control = "Control::FixtureId")]
    #[input(label = "red", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    #[input(label = "green", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    #[input(label = "blue", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    SetColor,

    #[node(name = "Set Pan & Tilt", category = "NodeCategory::Output", processor = "processor::set_pan_tilt")]
    #[input(label = "Fixture", data_type = "DataType::FixtureId", control = "Control::FixtureId")]
    #[input(label = "pan", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    #[input(label = "tilt", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    #[input(label = "pan rot", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    #[input(label = "tilt rot", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    #[input(label = "pos fx", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    #[input(label = "pos fx rate", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    #[input(label = "pos fx fade", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    SetPanTilt,

    #[node(name = "Set XYZ", category = "NodeCategory::Output", processor = "processor::set_xyz")]
    #[input(label = "Fixture", data_type = "DataType::FixtureId", control = "Control::FixtureId")]
    #[input(label = "x", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    #[input(label = "y", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    #[input(label = "z", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    #[input(label = "x rot", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    #[input(label = "y rot", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    #[input(label = "z rot", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    #[input(label = "x scale", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    #[input(label = "y scale", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    #[input(label = "z scale", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    #[input(label = "xyz scale", data_type = "DataType::AttributeValue", control = "Control::AttributeValue")]
    SetXyz,
}

mod processor {
    #![allow(clippy::too_many_arguments)]

    use std::time::{SystemTime, UNIX_EPOCH};

    use dmx::DmxAddress;

    use super::*;

    // New Values

    pub fn new_dmx_address(
        universe: DmxUniverseId,
        channel: DmxChannel,
        _pctx: &mut ProcessingContext,
        _cx: &mut AppContext,
    ) -> NewDmxAddressProcessingOutput {
        NewDmxAddressProcessingOutput {
            address: Value::from(DmxAddress::new(universe, channel)),
        }
    }

    // Math

    pub fn add(
        a: f64,
        b: f64,
        _pctx: &mut ProcessingContext,
        _cx: &mut AppContext,
    ) -> AddProcessingOutput {
        AddProcessingOutput {
            sum: Value::from(a + b),
        }
    }

    pub fn subtract(
        a: f64,
        b: f64,
        _pctx: &mut ProcessingContext,
        _cx: &mut AppContext,
    ) -> SubtractProcessingOutput {
        SubtractProcessingOutput {
            difference: Value::from(a - b),
        }
    }

    pub fn multiply(
        a: f64,
        b: f64,
        _pctx: &mut ProcessingContext,
        _cx: &mut AppContext,
    ) -> MultiplyProcessingOutput {
        MultiplyProcessingOutput {
            product: Value::from(a * b),
        }
    }

    pub fn divide(
        a: f64,
        b: f64,
        _pctx: &mut ProcessingContext,
        _cx: &mut AppContext,
    ) -> DivideProcessingOutput {
        DivideProcessingOutput {
            quotient: Value::from(a / b),
        }
    }

    pub fn sine(
        a: f64,
        _pctx: &mut ProcessingContext,
        _cx: &mut AppContext,
    ) -> SineProcessingOutput {
        SineProcessingOutput {
            b: Value::from(a.sin()),
        }
    }

    pub fn cosine(
        a: f64,
        _pctx: &mut ProcessingContext,
        _cx: &mut AppContext,
    ) -> CosineProcessingOutput {
        CosineProcessingOutput {
            b: Value::from(a.cos()),
        }
    }

    pub fn floor(
        value: f64,
        _pctx: &mut ProcessingContext,
        _cx: &mut AppContext,
    ) -> FloorProcessingOutput {
        FloorProcessingOutput {
            floored: Value::from(value.floor() as i64),
        }
    }

    pub fn round(
        value: f64,
        _pctx: &mut ProcessingContext,
        _cx: &mut AppContext,
    ) -> RoundProcessingOutput {
        RoundProcessingOutput {
            rounded: Value::from(value.round() as i64),
        }
    }

    pub fn ceil(
        value: f64,
        _pctx: &mut ProcessingContext,
        _cx: &mut AppContext,
    ) -> CeilProcessingOutput {
        CeilProcessingOutput {
            ceiled: Value::from(value.ceil() as i64),
        }
    }

    // Context

    pub fn get_fixture(
        pctx: &mut ProcessingContext,
        cx: &mut AppContext,
    ) -> GetFixtureProcessingOutput {
        GetFixtureProcessingOutput {
            index: Value::from(pctx.current_fixture_index as i64),
            id: Value::FixtureId(pctx.current_fixture_id(cx)),
            address: Value::DmxAddress(*pctx.current_fixture(cx).dmx_address()),
        }
    }

    pub fn get_group(
        pctx: &mut ProcessingContext,
        cx: &mut AppContext,
    ) -> GetGroupProcessingOutput {
        GetGroupProcessingOutput {
            size: Value::from(pctx.group(cx).fixtures.len() as i64),
        }
    }

    // Utilities

    pub fn split_address(
        address: DmxAddress,
        _pctx: &mut ProcessingContext,
        _cx: &mut AppContext,
    ) -> SplitAddressProcessingOutput {
        SplitAddressProcessingOutput {
            universe: Value::from(address.universe.value() as i64),
            channel: Value::from(address.channel.value() as i64),
        }
    }

    pub fn random_float(
        _pctx: &mut ProcessingContext,
        _cx: &mut AppContext,
    ) -> RandomProcessingOutput {
        RandomProcessingOutput {
            value: Value::from(rand::random::<f64>()),
        }
    }

    pub fn time(_pctx: &mut ProcessingContext, _cx: &mut AppContext) -> TimeProcessingOutput {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        TimeProcessingOutput {
            seconds: Value::from(time.as_secs_f64()),
        }
    }

    pub fn remap(
        value: f64,
        from_min: f64,
        from_max: f64,
        to_min: f64,
        to_max: f64,
        _pctx: &mut ProcessingContext,
        _cx: &mut AppContext,
    ) -> RemapProcessingOutput {
        let remapped = (value - from_min) / (from_max - from_min) * (to_max - to_min) + to_min;
        RemapProcessingOutput {
            remapped: Value::from(remapped),
        }
    }

    // Output

    pub fn set_channel_value(
        address: DmxAddress,
        value: AttributeValue,
        pctx: &mut ProcessingContext,
        _cx: &mut AppContext,
    ) -> SetChannelValueProcessingOutput {
        pctx.dmx_output
            .borrow_mut()
            .set_channel_value(address, value.byte());

        SetChannelValueProcessingOutput {}
    }

    pub fn set_fixture_attribute(
        fixture: FixtureId,
        attribute: SharedString,
        value: AttributeValue,
        pctx: &mut ProcessingContext,
        cx: &mut AppContext,
    ) -> SetFixtureAttributeProcessingOutput {
        let patch = &pctx.patch(cx);

        let Some(fixture) = patch.fixture(fixture) else {
            log::debug!("Fixture with id `{fixture}` not found");
            return SetFixtureAttributeProcessingOutput {};
        };

        let Some(offset) = fixture.channel_offset_for_attribute(attribute.as_ref(), patch) else {
            return SetFixtureAttributeProcessingOutput {};
        };

        let address = fixture
            .dmx_address()
            .with_channel_offset(*offset.first().unwrap() as u16 - 1);

        set_channel_value(address, value, pctx, cx);

        SetFixtureAttributeProcessingOutput {}
    }

    pub fn set_dimmer(
        fixture: FixtureId,
        dimmer: AttributeValue,
        pctx: &mut ProcessingContext,
        cx: &mut AppContext,
    ) -> SetDimmerProcessingOutput {
        use crate::AttributeDefinition as AD;

        set_fixture_attribute(fixture, AD::Dimmer.to_string().into(), dimmer, pctx, cx);

        SetDimmerProcessingOutput {}
    }

    pub fn set_color(
        fixture: FixtureId,
        red: AttributeValue,
        green: AttributeValue,
        blue: AttributeValue,
        pctx: &mut ProcessingContext,
        cx: &mut AppContext,
    ) -> SetColorProcessingOutput {
        use crate::AttributeDefinition as AD;

        set_fixture_attribute(fixture, AD::ColorAddR.to_string().into(), red, pctx, cx);
        set_fixture_attribute(
            fixture,
            AD::ColorSubC.to_string().into(),
            red.inverted(),
            pctx,
            cx,
        );
        set_fixture_attribute(fixture, AD::ColorRgbRed.to_string().into(), red, pctx, cx);
        set_fixture_attribute(fixture, AD::ColorAddG.to_string().into(), green, pctx, cx);
        set_fixture_attribute(
            fixture,
            AD::ColorSubM.to_string().into(),
            green.inverted(),
            pctx,
            cx,
        );
        set_fixture_attribute(
            fixture,
            AD::ColorRgbGreen.to_string().into(),
            green,
            pctx,
            cx,
        );
        set_fixture_attribute(fixture, AD::ColorAddB.to_string().into(), blue, pctx, cx);
        set_fixture_attribute(
            fixture,
            AD::ColorSubY.to_string().into(),
            blue.inverted(),
            pctx,
            cx,
        );
        set_fixture_attribute(fixture, AD::ColorRgbBlue.to_string().into(), blue, pctx, cx);

        SetColorProcessingOutput {}
    }

    pub fn set_pan_tilt(
        fixture: FixtureId,
        pan: AttributeValue,
        tilt: AttributeValue,
        pan_rotate: AttributeValue,
        tilt_rotate: AttributeValue,
        position_effect: AttributeValue,
        position_effect_rate: AttributeValue,
        position_effect_fade: AttributeValue,
        pctx: &mut ProcessingContext,
        cx: &mut AppContext,
    ) -> SetPanTiltProcessingOutput {
        use crate::AttributeDefinition as AD;

        set_fixture_attribute(fixture, AD::Pan.to_string().into(), pan, pctx, cx);
        set_fixture_attribute(fixture, AD::Tilt.to_string().into(), tilt, pctx, cx);
        set_fixture_attribute(
            fixture,
            AD::PanRotate.to_string().into(),
            pan_rotate,
            pctx,
            cx,
        );
        set_fixture_attribute(
            fixture,
            AD::TiltRotate.to_string().into(),
            tilt_rotate,
            pctx,
            cx,
        );
        set_fixture_attribute(
            fixture,
            AD::PositionEffect.to_string().into(),
            position_effect,
            pctx,
            cx,
        );
        set_fixture_attribute(
            fixture,
            AD::PositionEffectRate.to_string().into(),
            position_effect_rate,
            pctx,
            cx,
        );
        set_fixture_attribute(
            fixture,
            AD::PositionEffectFade.to_string().into(),
            position_effect_fade,
            pctx,
            cx,
        );

        SetPanTiltProcessingOutput {}
    }

    pub fn set_xyz(
        fixture: FixtureId,
        x: AttributeValue,
        y: AttributeValue,
        z: AttributeValue,
        x_rot: AttributeValue,
        y_rot: AttributeValue,
        z_rot: AttributeValue,
        x_scale: AttributeValue,
        y_scale: AttributeValue,
        z_scale: AttributeValue,
        xyz_scale: AttributeValue,
        pctx: &mut ProcessingContext,
        cx: &mut AppContext,
    ) -> SetXyzProcessingOutput {
        use crate::AttributeDefinition as AD;

        set_fixture_attribute(fixture, AD::XyzX.to_string().into(), x, pctx, cx);
        set_fixture_attribute(fixture, AD::XyzY.to_string().into(), y, pctx, cx);
        set_fixture_attribute(fixture, AD::XyzZ.to_string().into(), z, pctx, cx);
        set_fixture_attribute(fixture, AD::RotX.to_string().into(), x_rot, pctx, cx);
        set_fixture_attribute(fixture, AD::RotY.to_string().into(), y_rot, pctx, cx);
        set_fixture_attribute(fixture, AD::RotZ.to_string().into(), z_rot, pctx, cx);
        set_fixture_attribute(fixture, AD::ScaleX.to_string().into(), x_scale, pctx, cx);
        set_fixture_attribute(fixture, AD::ScaleY.to_string().into(), y_scale, pctx, cx);
        set_fixture_attribute(fixture, AD::ScaleZ.to_string().into(), z_scale, pctx, cx);
        set_fixture_attribute(
            fixture,
            AD::ScaleXyz.to_string().into(),
            xyz_scale,
            pctx,
            cx,
        );

        SetXyzProcessingOutput {}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, strum::EnumIter)]
pub enum NodeCategory {
    NewValue,
    Math,
    Utilities,
    Context,
    Output,
}

impl flow::gpui::NodeCategory for NodeCategory {
    fn all() -> impl Iterator<Item = Self> {
        Self::iter()
    }
}

impl Display for NodeCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            NodeCategory::NewValue => "New Value",
            NodeCategory::Math => "Math",
            NodeCategory::Utilities => "Utilities",
            NodeCategory::Context => "Context",
            NodeCategory::Output => "Output",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct NodeData {
    pub position: flow::Point,
}

impl VisualNodeData for NodeData {
    fn position(&self) -> &flow::Point {
        &self.position
    }

    fn set_position(&mut self, position: flow::Point) {
        self.position = position;
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, flow::Value)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(SharedString),

    FixtureId(FixtureId),
    AttributeValue(AttributeValue),
    DmxChannel(DmxChannel),
    DmxUniverse(DmxUniverseId),
    DmxAddress(DmxAddress),
}

impl flow::Value<GraphDefinition> for Value {
    fn try_cast_to(&self, target: &DataType) -> Result<Self, FlowError> {
        use DataType as DT;
        let result = match (self, target) {
            (Self::Int(_), DT::Int) => Ok(self.clone()),
            (Self::Int(v), DT::Float) => Ok(Self::Float(*v as f64)),
            (Self::Int(v), DT::FixtureId) => Ok(Self::FixtureId(FixtureId(*v as u32))),
            (Self::Int(v), DT::DmxChannel) => {
                Ok(Self::DmxChannel(DmxChannel::new_clamped(*v as u16)))
            }
            (Self::Int(v), DT::DmxUniverse) => {
                Ok(Self::DmxUniverse(DmxUniverseId::new_clamped(*v as u16)))
            }
            (Self::Int(v), DT::AttributeValue) => {
                Ok(Self::AttributeValue(AttributeValue::new(*v as f32)))
            }

            (Self::Float(_), DT::Float) => Ok(self.clone()),
            (Self::Float(v), DT::Int) => Ok(Self::Int(*v as i64)),
            (Self::Float(v), DT::FixtureId) => Ok(Self::FixtureId(FixtureId(*v as u32))),
            (Self::Float(v), DT::DmxChannel) => {
                Ok(Self::DmxChannel(DmxChannel::new_clamped(*v as u16)))
            }
            (Self::Float(v), DT::DmxUniverse) => {
                Ok(Self::DmxUniverse(DmxUniverseId::new_clamped(*v as u16)))
            }
            (Self::Float(v), DT::AttributeValue) => {
                Ok(Self::AttributeValue(AttributeValue::new(*v as f32)))
            }

            (Self::String(_), DT::String) => Ok(self.clone()),

            (Self::FixtureId(_), DT::FixtureId) => Ok(self.clone()),
            (Self::FixtureId(v), DT::Int) => Ok(Self::Int(v.0 as i64)),
            (Self::FixtureId(v), DT::Float) => Ok(Self::Float(v.0 as f64)),
            (Self::FixtureId(v), DT::DmxChannel) => {
                Ok(Self::DmxChannel(DmxChannel::new_clamped(v.0 as u16)))
            }
            (Self::FixtureId(v), DT::DmxUniverse) => {
                Ok(Self::DmxUniverse(DmxUniverseId::new_clamped(v.0 as u16)))
            }

            (Self::AttributeValue(_), DT::AttributeValue) => Ok(self.clone()),
            (Self::AttributeValue(v), DT::Int) => Ok(Self::Int(v.byte() as i64)),
            (Self::AttributeValue(v), DT::Float) => Ok(Self::Float(v.byte() as f64)),

            (Self::DmxChannel(_), DT::DmxChannel) => Ok(self.clone()),
            (Self::DmxChannel(v), DT::Int) => Ok(Self::Int(v.value() as i64)),
            (Self::DmxChannel(v), DT::Float) => Ok(Self::Float(v.value() as f64)),
            (Self::DmxChannel(v), DT::DmxUniverse) => {
                Ok(Self::DmxUniverse(DmxUniverseId::new_clamped(v.value())))
            }

            (Self::DmxAddress(_), DT::DmxAddress) => Ok(self.clone()),
            (Self::DmxAddress(v), DT::DmxChannel) => Ok(Self::DmxChannel(v.channel)),
            (Self::DmxAddress(v), DT::DmxUniverse) => Ok(Self::DmxUniverse(v.universe)),

            (Self::DmxUniverse(v), DT::DmxUniverse) => Ok(Self::DmxUniverse(*v)),
            (Self::DmxUniverse(v), DT::Int) => Ok(Self::Int(v.value() as i64)),
            (Self::DmxUniverse(v), DT::Float) => Ok(Self::Float(v.value() as f64)),

            _ => Err(FlowError::CastFailed),
        };
        match result {
            Ok(v) => Ok(v),
            Err(err) => {
                log::warn!(
                    "Failed to cast value '{:?}' to type '{:?}': {:?}",
                    self,
                    target,
                    err
                );
                Err(err)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DataType {
    Int,
    Float,
    String,

    FixtureId,
    AttributeValue,
    DmxChannel,
    DmxAddress,
    DmxUniverse,
}

impl flow::DataType<GraphDefinition> for DataType {
    fn default_value(&self) -> Value {
        match self {
            Self::Int => Value::Int(i64::default()),
            Self::Float => Value::Float(f64::default()),
            Self::FixtureId => Value::FixtureId(FixtureId::default()),
            Self::String => Value::String(SharedString::default()),
            Self::AttributeValue => Value::AttributeValue(AttributeValue::default()),
            Self::DmxChannel => Value::DmxChannel(DmxChannel::default()),
            Self::DmxUniverse => Value::DmxUniverse(DmxUniverseId::default()),
            Self::DmxAddress => Value::DmxAddress(DmxAddress::default()),
        }
    }
}

impl VisualDataType for DataType {
    fn color(&self) -> Hsla {
        match self {
            Self::Int => rgb(0xC741FF).into(),
            Self::Float => rgb(0xFF3C59).into(),
            Self::String => rgb(0xFF6E1B).into(),

            Self::FixtureId => rgb(0x146EBD).into(),
            Self::AttributeValue => rgb(0xFFC917).into(),
            Self::DmxChannel => rgb(0xAAFF00).into(),
            Self::DmxUniverse => rgb(0x00FF44).into(),
            Self::DmxAddress => rgb(0x00D9E4).into(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Control {
    Int,
    Float,
    String,

    FixtureId,
    AttributeValue,
    DmxChannel,
    DmxUniverse,
    DmxAddress,
}

impl VisualControl<GraphDefinition> for Control {
    fn view<View: EventEmitter<ControlEvent<GraphDefinition>>>(
        &self,
        id: impl Into<ElementId>,
        initial_value: Value,
        cx: &mut ViewContext<View>,
    ) -> AnyView {
        use ui::NumberFieldEvent;

        match self {
            Self::Int => {
                let field = cx.new_view(|cx| {
                    let value: i64 = initial_value
                        .try_into()
                        .expect("Int field expects an i64 value");
                    let field = NumberField::new(id, value as f64, cx);
                    field.set_validate(Some(Box::new(|v| v.parse::<i64>().is_ok())), cx);
                    field
                });

                cx.subscribe(&field, |_this, _field, event: &NumberFieldEvent, cx| {
                    let NumberFieldEvent::Change(float_value) = event;
                    let value = Value::Int(*float_value as i64);
                    cx.emit(ControlEvent::Change(value));
                })
                .detach();

                field.into()
            }
            Self::Float => {
                let field = cx.new_view(|cx| {
                    let value: f64 = initial_value
                        .try_into()
                        .expect("Float field expects an f64 value");
                    let field = NumberField::new(id, value, cx);
                    field.set_validate(Some(Box::new(|v| v.parse::<f64>().is_ok())), cx);
                    field
                });

                cx.subscribe(&field, |_this, _field, event: &NumberFieldEvent, cx| {
                    let NumberFieldEvent::Change(float_value) = event;
                    let value = Value::Float(*float_value);
                    cx.emit(ControlEvent::Change(value));
                })
                .detach();

                field.into()
            }
            Self::String => {
                let field = cx.new_view(|cx| {
                    let value: SharedString = initial_value
                        .try_into()
                        .expect("String field expects a String value");
                    TextField::new(id, value, cx)
                });

                cx.subscribe(&field, |_this, _field, event: &TextFieldEvent, cx| {
                    if let TextFieldEvent::Change(string_value) = event {
                        let value = Value::String(string_value.clone());
                        cx.emit(ControlEvent::Change(value));
                    }
                })
                .detach();

                field.into()
            }
            Self::FixtureId => {
                let field = cx.new_view(|cx| {
                    let value: FixtureId = initial_value
                        .try_into()
                        .expect("FixtureId field expects a FixtureId value");
                    let field = NumberField::new(id, value.0 as f64, cx);
                    field.set_validate(Some(Box::new(|v| v.parse::<FixtureId>().is_ok())), cx);
                    field
                });

                cx.subscribe(&field, |_this, _field, event: &NumberFieldEvent, cx| {
                    let NumberFieldEvent::Change(float_value) = event;
                    let value = Value::FixtureId(FixtureId(*float_value as u32));
                    cx.emit(ControlEvent::Change(value));
                })
                .detach();

                field.into()
            }
            Self::AttributeValue => {
                let slider = cx.new_view(|cx| {
                    let value: AttributeValue = initial_value
                        .try_into()
                        .expect("AttributeValue field expects a AttributeValue value");
                    let mut slider = Slider::new(id, value.relative_value() as f64, cx);
                    slider.set_range(0.0..=1.0);
                    slider.set_strict(true);
                    slider
                });

                cx.subscribe(&slider, |_this, _slider, event: &SliderEvent, cx| {
                    let SliderEvent::Change(float_value) = event;
                    cx.emit(ControlEvent::Change(Value::AttributeValue(
                        AttributeValue::new(*float_value as f32),
                    )));
                })
                .detach();

                slider.into()
            }
            Self::DmxChannel => {
                let field = cx.new_view(|cx| {
                    let channel: DmxChannel = initial_value
                        .try_into()
                        .expect("DmxChannel field expects a DmxChannel value");
                    let field = NumberField::new(id, channel.value() as f64, cx);
                    field.set_validate(Some(Box::new(|v| v.parse::<DmxChannel>().is_ok())), cx);
                    field
                });

                cx.subscribe(&field, |_this, _field, event: &NumberFieldEvent, cx| {
                    let NumberFieldEvent::Change(float_value) = event;
                    let value =
                        Value::DmxChannel(DmxChannel::new(*float_value as u16).unwrap_or_default());
                    cx.emit(ControlEvent::Change(value));
                })
                .detach();

                field.into()
            }
            Self::DmxUniverse => {
                let field = cx.new_view(|cx| {
                    let universe: DmxUniverseId = initial_value
                        .try_into()
                        .expect("DmxUniverse field expects a DmxUniverse value");
                    let field = NumberField::new(id, universe.value() as f64, cx);
                    field.set_validate(Some(Box::new(|v| v.parse::<DmxUniverseId>().is_ok())), cx);
                    field
                });

                cx.subscribe(&field, |_this, _field, event: &NumberFieldEvent, cx| {
                    let NumberFieldEvent::Change(float_value) = event;
                    let value = Value::DmxUniverse(
                        DmxUniverseId::new(*float_value as u16).unwrap_or_default(),
                    );
                    cx.emit(ControlEvent::Change(value));
                })
                .detach();

                field.into()
            }
            Self::DmxAddress => {
                let field = cx.new_view(|cx| {
                    let address: DmxAddress = initial_value
                        .try_into()
                        .expect("DmxAddress field expects a DmxAddress value");
                    let mut field = TextField::new(id, address.to_string().into(), cx);
                    field.set_validate(Some(Box::new(|v| v.parse::<DmxAddress>().is_ok())));
                    field
                });

                cx.subscribe(&field, |_this, _field, event: &TextFieldEvent, cx| {
                    if let TextFieldEvent::Change(string_value) = event {
                        let address: DmxAddress = string_value.parse().unwrap_or_default();
                        let value = Value::DmxAddress(address);
                        cx.emit(ControlEvent::Change(value));
                    }
                })
                .detach();

                field.into()
            }
        }
    }
}

pub struct ProcessingContext {
    pub dmx_output: Rc<RefCell<DmxOutput>>,

    template: Template,
    show: Model<Show>,

    current_fixture_index: usize,
}

impl ProcessingContext {
    pub fn new(show: Model<Show>, template: Template, dmx_output: Rc<RefCell<DmxOutput>>) -> Self {
        Self {
            dmx_output,

            template,
            show,

            current_fixture_index: 0,
        }
    }

    pub fn group<'a>(&self, cx: &'a AppContext) -> &'a Group {
        self.show
            .read(cx)
            .assets
            .groups
            .read(cx)
            .get(&self.template.group)
            .unwrap()
    }

    pub fn graph<'a>(&self, cx: &'a AppContext) -> &'a FlowEffectGraph {
        let Effect::Graph(id) = &self.template.effect;
        &self
            .show
            .read(cx)
            .assets
            .effect_graphs
            .read(cx)
            .get(id)
            .unwrap()
            .graph
    }

    pub fn patch<'a>(&self, cx: &'a AppContext) -> &'a Patch {
        self.show.read(cx).patch.read(cx)
    }

    pub fn process_frame(&mut self, cx: &mut AppContext) -> Result<(), FlowError> {
        self.current_fixture_index = 0;
        while self.current_fixture_index < self.group(cx).len() {
            if self
                .patch(cx)
                .fixture(self.current_fixture_id(cx))
                .is_some()
            {
                self.graph(cx).clone().process(self, cx)?;
            } else {
                log::warn!(
                    "Tried to process effect graph with invalid FixtureId. Skipping fixture."
                );
            }

            self.current_fixture_index += 1;
        }
        Ok(())
    }

    pub fn current_fixture<'a>(&self, cx: &'a AppContext) -> &'a Fixture {
        self.patch(cx).fixture(self.current_fixture_id(cx)).unwrap()
    }

    pub fn current_fixture_id(&self, cx: &AppContext) -> FixtureId {
        self.group(cx).fixtures[self.current_fixture_index]
    }
}
