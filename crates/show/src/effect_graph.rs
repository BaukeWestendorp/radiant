use crate::attr_def::AttributeDefinition;
use crate::fixture::{AttributeValue, FixtureId};
use crate::patch::PatchedFixture;
use crate::{FixtureGroup, Show};

use dmx::{DmxAddress, DmxChannel, DmxOutput, DmxUniverseId};
use flow::gpui::{ControlEvent, VisualControl, VisualDataType, VisualNodeData, VisualNodeKind};
use flow::{FlowError, Graph};
use gpui::{rgb, AnyView, ElementId, EventEmitter, Hsla, SharedString, ViewContext, VisualContext};
use std::fmt::Display;
use strum::IntoEnumIterator;
use ui::input::{NumberField, Slider, SliderEvent, TextField, TextFieldEvent};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphDefinition;

impl flow::GraphDefinition for GraphDefinition {
    type NodeKind = NodeKind;
    type NodeData = NodeData;
    type Value = Value;
    type DataType = DataType;
    type Control = Control;
}

pub type EffectGraph = Graph<GraphDefinition>;

#[derive(
    Debug, Clone, PartialEq, strum::EnumIter, serde::Serialize, serde::Deserialize, flow::NodeKind,
)]
#[node_kind(
    category = "NodeCategory",
    graph_definition = "GraphDefinition",
    processing_context = "ProcessingContext"
)]
pub enum NodeKind {
    // New Values
    #[node(name = "New Fixture Id", category = "NodeCategory::NewValue")]
    #[constant_output(
        label = "id",
        data_type = "DataType::FixtureId",
        control = "Control::FixtureId"
    )]
    NewFixtureId,

    #[node(name = "New Attribute Value", category = "NodeCategory::NewValue")]
    #[constant_output(
        label = "value",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    NewAttributeValue,

    #[node(
        name = "New DMX Address",
        category = "NodeCategory::NewValue",
        processor = "processor::new_dmx_address"
    )]
    #[input(
        label = "universe",
        data_type = "DataType::DmxUniverse",
        control = "Control::DmxUniverse"
    )]
    #[input(
        label = "channel",
        data_type = "DataType::DmxChannel",
        control = "Control::DmxChannel"
    )]
    #[computed_output(label = "address", data_type = "DataType::DmxAddress")]
    NewDmxAddress,

    // Math
    #[node(
        name = "Add",
        category = "NodeCategory::Math",
        processor = "processor::add"
    )]
    #[input(label = "a", data_type = "DataType::Float", control = "Control::Float")]
    #[input(label = "b", data_type = "DataType::Float", control = "Control::Float")]
    #[computed_output(label = "sum", data_type = "DataType::Float")]
    Add,

    #[node(
        name = "Subtract",
        category = "NodeCategory::Math",
        processor = "processor::subtract"
    )]
    #[input(label = "a", data_type = "DataType::Float", control = "Control::Float")]
    #[input(label = "b", data_type = "DataType::Float", control = "Control::Float")]
    #[computed_output(label = "difference", data_type = "DataType::Float")]
    Subtract,

    #[node(
        name = "Multiply",
        category = "NodeCategory::Math",
        processor = "processor::multiply"
    )]
    #[input(label = "a", data_type = "DataType::Float", control = "Control::Float")]
    #[input(label = "b", data_type = "DataType::Float", control = "Control::Float")]
    #[computed_output(label = "product", data_type = "DataType::Float")]
    Multiply,

    #[node(
        name = "Divide",
        category = "NodeCategory::Math",
        processor = "processor::divide"
    )]
    #[input(label = "a", data_type = "DataType::Float", control = "Control::Float")]
    #[input(label = "b", data_type = "DataType::Float", control = "Control::Float")]
    #[computed_output(label = "quotient", data_type = "DataType::Float")]
    Divide,

    #[node(
        name = "Floor",
        category = "NodeCategory::Math",
        processor = "processor::floor"
    )]
    #[input(
        label = "value",
        data_type = "DataType::Float",
        control = "Control::Float"
    )]
    #[computed_output(label = "floored", data_type = "DataType::Int")]
    Floor,

    #[node(
        name = "Round",
        category = "NodeCategory::Math",
        processor = "processor::round"
    )]
    #[input(
        label = "value",
        data_type = "DataType::Float",
        control = "Control::Float"
    )]
    #[computed_output(label = "rounded", data_type = "DataType::Int")]
    Round,

    #[node(
        name = "Ceil",
        category = "NodeCategory::Math",
        processor = "processor::ceil"
    )]
    #[input(
        label = "value",
        data_type = "DataType::Float",
        control = "Control::Float"
    )]
    #[computed_output(label = "ceiled", data_type = "DataType::Int")]
    Ceil,

    // Context
    #[node(
        name = "Get Fixture",
        category = "NodeCategory::Context",
        processor = "processor::get_fixture"
    )]
    #[computed_output(label = "index", data_type = "DataType::Int")]
    #[computed_output(label = "id", data_type = "DataType::FixtureId")]
    #[computed_output(label = "address", data_type = "DataType::DmxAddress")]
    GetFixture,

    #[node(
        name = "Get Group",
        category = "NodeCategory::Context",
        processor = "processor::get_group"
    )]
    #[computed_output(label = "size", data_type = "DataType::Int")]
    GetGroup,

    // Utilities
    #[node(
        name = "Split Address",
        category = "NodeCategory::Utilities",
        processor = "processor::split_address"
    )]
    #[input(
        label = "address",
        data_type = "DataType::DmxAddress",
        control = "Control::DmxAddress"
    )]
    #[computed_output(label = "universe", data_type = "DataType::DmxUniverse")]
    #[computed_output(label = "channel", data_type = "DataType::DmxChannel")]
    SplitAddress,

    // Output
    #[node(
        name = "Set Address Value",
        category = "NodeCategory::Output",
        processor = "processor::set_channel_value"
    )]
    #[input(
        label = "address",
        data_type = "DataType::DmxAddress",
        control = "Control::DmxAddress"
    )]
    #[input(
        label = "value",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    SetChannelValue,

    #[node(
        name = "Set Fixture Attribute",
        category = "NodeCategory::Output",
        processor = "processor::set_fixture_attribute"
    )]
    #[input(
        label = "fixture",
        data_type = "DataType::FixtureId",
        control = "Control::FixtureId"
    )]
    #[input(
        label = "attribute",
        data_type = "DataType::String",
        control = "Control::String"
    )]
    #[input(
        label = "value",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    SetFixtureAttribute,

    #[node(
        name = "Set Dimmer",
        category = "NodeCategory::Output",
        processor = "processor::set_dimmer"
    )]
    #[input(
        label = "Fixture",
        data_type = "DataType::FixtureId",
        control = "Control::FixtureId"
    )]
    #[input(
        label = "dimmer",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    SetDimmer,

    #[node(
        name = "Set Color",
        category = "NodeCategory::Output",
        processor = "processor::set_color"
    )]
    #[input(
        label = "Fixture",
        data_type = "DataType::FixtureId",
        control = "Control::FixtureId"
    )]
    #[input(
        label = "red",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    #[input(
        label = "green",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    #[input(
        label = "blue",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    SetColor,

    #[node(
        name = "Set Pan & Tilt",
        category = "NodeCategory::Output",
        processor = "processor::set_pan_tilt"
    )]
    #[input(
        label = "Fixture",
        data_type = "DataType::FixtureId",
        control = "Control::FixtureId"
    )]
    #[input(
        label = "pan",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    #[input(
        label = "tilt",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    #[input(
        label = "pan rot",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    #[input(
        label = "tilt rot",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    #[input(
        label = "position fx",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    #[input(
        label = "position fx rate",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    #[input(
        label = "position fx fade",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    SetPanTilt,

    #[node(
        name = "Set XYZ",
        category = "NodeCategory::Output",
        processor = "processor::set_xyz"
    )]
    #[input(
        label = "Fixture",
        data_type = "DataType::FixtureId",
        control = "Control::FixtureId"
    )]
    #[input(
        label = "x",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    #[input(
        label = "y",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    #[input(
        label = "z",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    #[input(
        label = "x rot",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    #[input(
        label = "y rot",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    #[input(
        label = "z rot",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    #[input(
        label = "x scale",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    #[input(
        label = "y scale",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    #[input(
        label = "z scale",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    #[input(
        label = "xyz scale",
        data_type = "DataType::AttributeValue",
        control = "Control::AttributeValue"
    )]
    SetXyz,
}

mod processor {
    use dmx::DmxAddress;

    use super::*;

    pub fn new_dmx_address(
        universe: DmxUniverseId,
        channel: DmxChannel,
        _context: &mut ProcessingContext,
    ) -> NewDmxAddressProcessingOutput {
        NewDmxAddressProcessingOutput {
            address: Value::from(DmxAddress::new(universe, channel)),
        }
    }

    pub fn add(a: f32, b: f32, _ctx: &mut ProcessingContext) -> AddProcessingOutput {
        AddProcessingOutput {
            sum: Value::from(a + b),
        }
    }

    pub fn subtract(a: f32, b: f32, _ctx: &mut ProcessingContext) -> SubtractProcessingOutput {
        SubtractProcessingOutput {
            difference: Value::from(a - b),
        }
    }

    pub fn multiply(a: f32, b: f32, _ctx: &mut ProcessingContext) -> MultiplyProcessingOutput {
        MultiplyProcessingOutput {
            product: Value::from(a * b),
        }
    }

    pub fn divide(a: f32, b: f32, _ctx: &mut ProcessingContext) -> DivideProcessingOutput {
        DivideProcessingOutput {
            quotient: Value::from(a / b),
        }
    }

    pub fn floor(value: f32, _ctx: &mut ProcessingContext) -> FloorProcessingOutput {
        FloorProcessingOutput {
            floored: Value::from(value.floor() as i32),
        }
    }

    pub fn round(value: f32, _ctx: &mut ProcessingContext) -> RoundProcessingOutput {
        RoundProcessingOutput {
            rounded: Value::from(value.round() as i32),
        }
    }

    pub fn ceil(value: f32, _ctx: &mut ProcessingContext) -> CeilProcessingOutput {
        CeilProcessingOutput {
            ceiled: Value::from(value.ceil() as i32),
        }
    }

    pub fn get_fixture(ctx: &mut ProcessingContext) -> GetFixtureProcessingOutput {
        GetFixtureProcessingOutput {
            index: Value::from(ctx.current_fixture_index as i32),
            id: Value::FixtureId(ctx.current_fixture_id()),
            address: Value::DmxAddress(ctx.current_fixture().dmx_address),
        }
    }

    pub fn get_group(ctx: &mut ProcessingContext) -> GetGroupProcessingOutput {
        GetGroupProcessingOutput {
            size: Value::from(ctx.group().fixtures().len() as i32),
        }
    }

    pub fn split_address(
        address: DmxAddress,
        _ctx: &mut ProcessingContext,
    ) -> SplitAddressProcessingOutput {
        SplitAddressProcessingOutput {
            universe: Value::from(address.universe.value() as i32),
            channel: Value::from(address.channel.value() as i32),
        }
    }

    pub fn set_channel_value(
        address: DmxAddress,
        value: AttributeValue,
        ctx: &mut ProcessingContext,
    ) -> SetChannelValueProcessingOutput {
        ctx.dmx_output.set_channel_value(address, value.byte());

        SetChannelValueProcessingOutput {}
    }

    pub fn set_fixture_attribute(
        fixture: FixtureId,
        attribute: SharedString,
        value: AttributeValue,
        ctx: &mut ProcessingContext,
    ) -> SetFixtureAttributeProcessingOutput {
        let patch = ctx.show.patch();

        let Some(fixture) = ctx.show.patch().fixture(fixture) else {
            log::debug!("Fixture with id `{fixture}` not found");
            return SetFixtureAttributeProcessingOutput {};
        };

        let Some(offset) = fixture.channel_offset_for_attribute(&attribute.to_string(), patch)
        else {
            return SetFixtureAttributeProcessingOutput {};
        };

        let address = fixture
            .dmx_address
            .with_channel_offset(*offset.first().unwrap() as u16 - 1);

        set_channel_value(address, value, ctx);

        SetFixtureAttributeProcessingOutput {}
    }

    pub fn set_dimmer(
        fixture: FixtureId,
        dimmer: AttributeValue,
        ctx: &mut ProcessingContext,
    ) -> SetDimmerProcessingOutput {
        use AttributeDefinition as AD;

        set_fixture_attribute(fixture, AD::Dimmer.to_string().into(), dimmer, ctx);

        SetDimmerProcessingOutput {}
    }

    pub fn set_color(
        fixture: FixtureId,
        red: AttributeValue,
        green: AttributeValue,
        blue: AttributeValue,
        ctx: &mut ProcessingContext,
    ) -> SetColorProcessingOutput {
        use AttributeDefinition as AD;

        set_fixture_attribute(fixture, AD::ColorAddR.to_string().into(), red, ctx);
        set_fixture_attribute(
            fixture,
            AD::ColorSubC.to_string().into(),
            red.inverted(),
            ctx,
        );
        set_fixture_attribute(fixture, AD::ColorRgbRed.to_string().into(), red, ctx);
        set_fixture_attribute(fixture, AD::ColorAddG.to_string().into(), green, ctx);
        set_fixture_attribute(
            fixture,
            AD::ColorSubM.to_string().into(),
            green.inverted(),
            ctx,
        );
        set_fixture_attribute(fixture, AD::ColorRgbGreen.to_string().into(), green, ctx);
        set_fixture_attribute(fixture, AD::ColorAddB.to_string().into(), blue, ctx);
        set_fixture_attribute(
            fixture,
            AD::ColorSubY.to_string().into(),
            blue.inverted(),
            ctx,
        );
        set_fixture_attribute(fixture, AD::ColorRgbBlue.to_string().into(), blue, ctx);

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
        ctx: &mut ProcessingContext,
    ) -> SetPanTiltProcessingOutput {
        use AttributeDefinition as AD;

        set_fixture_attribute(fixture, AD::Pan.to_string().into(), pan, ctx);
        set_fixture_attribute(fixture, AD::Tilt.to_string().into(), tilt, ctx);
        set_fixture_attribute(fixture, AD::PanRotate.to_string().into(), pan_rotate, ctx);
        set_fixture_attribute(fixture, AD::TiltRotate.to_string().into(), tilt_rotate, ctx);
        set_fixture_attribute(
            fixture,
            AD::PositionEffect.to_string().into(),
            position_effect,
            ctx,
        );
        set_fixture_attribute(
            fixture,
            AD::PositionEffectRate.to_string().into(),
            position_effect_rate,
            ctx,
        );
        set_fixture_attribute(
            fixture,
            AD::PositionEffectFade.to_string().into(),
            position_effect_fade,
            ctx,
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
        ctx: &mut ProcessingContext,
    ) -> SetXyzProcessingOutput {
        use AttributeDefinition as AD;

        set_fixture_attribute(fixture, AD::XyzX.to_string().into(), x, ctx);
        set_fixture_attribute(fixture, AD::XyzY.to_string().into(), y, ctx);
        set_fixture_attribute(fixture, AD::XyzZ.to_string().into(), z, ctx);
        set_fixture_attribute(fixture, AD::RotX.to_string().into(), x_rot, ctx);
        set_fixture_attribute(fixture, AD::RotY.to_string().into(), y_rot, ctx);
        set_fixture_attribute(fixture, AD::RotZ.to_string().into(), z_rot, ctx);
        set_fixture_attribute(fixture, AD::ScaleX.to_string().into(), x_scale, ctx);
        set_fixture_attribute(fixture, AD::ScaleY.to_string().into(), y_scale, ctx);
        set_fixture_attribute(fixture, AD::ScaleZ.to_string().into(), z_scale, ctx);
        set_fixture_attribute(fixture, AD::ScaleXyz.to_string().into(), xyz_scale, ctx);

        SetXyzProcessingOutput {}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, strum::EnumIter)]
pub enum NodeCategory {
    NewValue,
    Math,
    Context,
    Output,
    Utilities,
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
            NodeCategory::Context => "Context",
            NodeCategory::Output => "Output",
            NodeCategory::Utilities => "Utilities",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct NodeData {
    pub position: geo::Point,
}

impl VisualNodeData for NodeData {
    fn position(&self) -> &geo::Point {
        &self.position
    }

    fn set_position(&mut self, position: geo::Point) {
        self.position = position;
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, flow::Value)]
pub enum Value {
    Int(i32),
    Float(f32),
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
            (Self::Int(v), DT::Float) => Ok(Self::Float(*v as f32)),
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
            (Self::Float(v), DT::Int) => Ok(Self::Int(*v as i32)),
            (Self::Float(v), DT::FixtureId) => Ok(Self::FixtureId(FixtureId(*v as u32))),
            (Self::Float(v), DT::DmxChannel) => {
                Ok(Self::DmxChannel(DmxChannel::new_clamped(*v as u16)))
            }
            (Self::Float(v), DT::DmxUniverse) => {
                Ok(Self::DmxUniverse(DmxUniverseId::new_clamped(*v as u16)))
            }
            (Self::Float(v), DT::AttributeValue) => {
                Ok(Self::AttributeValue(AttributeValue::new(*v)))
            }

            (Self::String(_), DT::String) => Ok(self.clone()),

            (Self::FixtureId(_), DT::FixtureId) => Ok(self.clone()),
            (Self::FixtureId(v), DT::Int) => Ok(Self::Int(v.0 as i32)),
            (Self::FixtureId(v), DT::Float) => Ok(Self::Float(v.0 as f32)),
            (Self::FixtureId(v), DT::DmxChannel) => {
                Ok(Self::DmxChannel(DmxChannel::new_clamped(v.id() as u16)))
            }
            (Self::FixtureId(v), DT::DmxUniverse) => {
                Ok(Self::DmxUniverse(DmxUniverseId::new_clamped(v.id() as u16)))
            }

            (Self::AttributeValue(_), DT::AttributeValue) => Ok(self.clone()),
            (Self::AttributeValue(v), DT::Int) => Ok(Self::Int(v.byte() as i32)),
            (Self::AttributeValue(v), DT::Float) => Ok(Self::Float(v.byte() as f32)),

            (Self::DmxChannel(_), DT::DmxChannel) => Ok(self.clone()),
            (Self::DmxChannel(v), DT::Int) => Ok(Self::Int(v.value() as i32)),
            (Self::DmxChannel(v), DT::Float) => Ok(Self::Float(v.value() as f32)),
            (Self::DmxChannel(v), DT::DmxUniverse) => Ok(Self::DmxUniverse(
                DmxUniverseId::new_clamped(v.value() as u16),
            )),

            (Self::DmxAddress(_), DT::DmxAddress) => Ok(self.clone()),
            (Self::DmxAddress(v), DT::DmxChannel) => Ok(Self::DmxChannel(v.channel)),
            (Self::DmxAddress(v), DT::DmxUniverse) => Ok(Self::DmxUniverse(v.universe)),

            (Self::DmxUniverse(v), DT::DmxUniverse) => Ok(Self::DmxUniverse(*v)),
            (Self::DmxUniverse(v), DT::Int) => Ok(Self::Int(v.value() as i32)),
            (Self::DmxUniverse(v), DT::Float) => Ok(Self::Float(v.value() as f32)),

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
            Self::Int => Value::Int(i32::default()),
            Self::Float => Value::Float(f32::default()),
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

            Self::FixtureId => rgb(0x080AFF).into(),
            Self::AttributeValue => rgb(0xFFAE18).into(),
            Self::DmxChannel => rgb(0xFF0000).into(),
            Self::DmxUniverse => rgb(0x00FFFF).into(),
            Self::DmxAddress => rgb(0x00FF00).into(),
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
        use ui::input::NumberFieldEvent;

        match self {
            Self::Int => {
                let field = cx.new_view(|cx| {
                    let mut field = NumberField::new(cx);
                    let value: i32 = initial_value
                        .try_into()
                        .expect("Int field expects an i32 value");
                    field.set_value(value as f32, cx);
                    field.set_validate(Some(Box::new(|v| v.parse::<i32>().is_ok())), cx);
                    field
                });

                cx.subscribe(&field, |_this, _field, event: &NumberFieldEvent, cx| {
                    let NumberFieldEvent::Change(float_value) = event;
                    let value = Value::Int(*float_value as i32);
                    cx.emit(ControlEvent::Change(value));
                })
                .detach();

                field.into()
            }
            Self::Float => {
                let field = cx.new_view(|cx| {
                    let mut field = NumberField::new(cx);
                    let value: f32 = initial_value
                        .try_into()
                        .expect("Float field expects an f32 value");
                    field.set_value(value, cx);
                    field.set_validate(Some(Box::new(|v| v.parse::<f32>().is_ok())), cx);
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
                    let mut field = TextField::new(cx);
                    let value: SharedString = initial_value
                        .try_into()
                        .expect("String field expects a String value");
                    field.set_value(value, cx);
                    field
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
                    let mut field = NumberField::new(cx);
                    let value: FixtureId = initial_value
                        .try_into()
                        .expect("FixtureId field expects a FixtureId value");
                    field.set_value(value.0 as f32, cx);
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
                    let mut slider = Slider::new(id, cx);
                    let value: AttributeValue = initial_value
                        .try_into()
                        .expect("AttributeValue field expects a AttributeValue value");
                    slider.set_value(value.relative_value(), cx);
                    slider.set_range(0.0..=1.0, cx);
                    slider.set_strict(true);
                    slider
                });

                cx.subscribe(&slider, |_this, _slider, event: &SliderEvent, cx| {
                    let SliderEvent::Change(float_value) = event;
                    cx.emit(ControlEvent::Change(Value::AttributeValue(
                        AttributeValue::new(*float_value),
                    )));
                })
                .detach();

                slider.into()
            }
            Self::DmxChannel => {
                let field = cx.new_view(|cx| {
                    let mut field = NumberField::new(cx);
                    let channel: DmxChannel = initial_value
                        .try_into()
                        .expect("DmxChannel field expects a DmxChannel value");
                    field.set_value(channel.value() as f32, cx);
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
                    let mut field = NumberField::new(cx);
                    let universe: DmxUniverseId = initial_value
                        .try_into()
                        .expect("DmxUniverse field expects a DmxUniverse value");
                    field.set_value(universe.value() as f32, cx);
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
                    let mut field = TextField::new(cx);
                    let address: DmxAddress = initial_value
                        .try_into()
                        .expect("DmxAddress field expects a DmxAddress value");
                    field.set_value(address.to_string().into(), cx);
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
    pub dmx_output: DmxOutput,

    show: Show,

    group: FixtureGroup,
    current_fixture_index: usize,
}

impl ProcessingContext {
    pub fn new(show: Show) -> Self {
        Self {
            dmx_output: DmxOutput::new(),
            show,
            group: FixtureGroup::default(),
            current_fixture_index: 0,
        }
    }

    pub fn group(&self) -> &FixtureGroup {
        &self.group
    }

    pub fn set_group(&mut self, group: FixtureGroup) {
        self.group = group;
    }

    pub fn process_frame(&mut self, graph: &EffectGraph) -> Result<(), FlowError> {
        self.current_fixture_index = 0;
        while self.current_fixture_index < self.group.len() {
            if self
                .show
                .patch()
                .fixture(self.current_fixture_id())
                .is_none()
            {
                log::warn!(
                    "Tried to process effect graph with invalid FixtureId. Skipping fixture."
                );
                self.current_fixture_index += 1;
                continue;
            }

            graph.process(self)?;
            self.current_fixture_index += 1;
        }
        Ok(())
    }

    pub fn current_fixture(&self) -> &PatchedFixture {
        self.show
            .patch()
            .fixture(self.current_fixture_id())
            .unwrap()
    }

    pub fn current_fixture_id(&self) -> FixtureId {
        self.group.fixtures()[self.current_fixture_index]
    }
}
