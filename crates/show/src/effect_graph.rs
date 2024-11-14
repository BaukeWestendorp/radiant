use crate::fixture::{AttributeValue, FixtureId};
use crate::patch::PatchedFixture;
use crate::{FixtureGroup, Show};

use dmx::{DmxChannel, DmxOutput};
use flow::gpui::{ControlEvent, VisualControl, VisualDataType, VisualNodeData, VisualNodeKind};
use flow::{
    Graph, GraphError, InputParameterKind, Node, NodeId, OutputParameterKind, ProcessingResult,
    Value as _,
};
use gpui::{rgb, AnyView, ElementId, EventEmitter, Hsla, ViewContext, VisualContext};
use std::fmt::Display;
use strum::IntoEnumIterator;
use ui::input::{NumberField, Slider, SliderEvent};

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

#[derive(Debug, Clone, PartialEq, strum::EnumIter, serde::Serialize, serde::Deserialize)]
pub enum NodeKind {
    // New Values
    NewFixtureId,
    NewAttributeValue,

    // Math
    Add,
    Subtract,
    Multiply,
    Divide,
    Floor,
    Round,
    Ceil,

    // Context
    GetFixture,

    // Output
    SetChannelValue,
}

impl flow::NodeKind<GraphDefinition> for NodeKind {
    type ProcessingContext = ProcessingContext;

    fn build(&self, graph: &mut EffectGraph, node_id: NodeId) {
        match self {
            Self::NewFixtureId => {
                graph.add_output(
                    node_id,
                    "id".to_string(),
                    DataType::FixtureId,
                    OutputParameterKind::Constant {
                        value: Value::FixtureId(FixtureId::default()),
                        control: Control::FixtureId,
                    },
                );
            }
            Self::NewAttributeValue => {
                graph.add_output(
                    node_id,
                    "value".to_string(),
                    DataType::AttributeValue,
                    OutputParameterKind::Constant {
                        value: Value::AttributeValue(AttributeValue::default()),
                        control: Control::AttributeValue,
                    },
                );
            }

            Self::Add => {
                graph.add_input(
                    node_id,
                    "a".to_string(),
                    DataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: Value::Float(0.0),
                        control: Control::Float,
                    },
                );
                graph.add_input(
                    node_id,
                    "b".to_string(),
                    DataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: Value::Float(0.0),
                        control: Control::Float,
                    },
                );
                graph.add_output(
                    node_id,
                    "c".to_string(),
                    DataType::Float,
                    OutputParameterKind::Computed,
                );
            }
            Self::Subtract => {
                graph.add_input(
                    node_id,
                    "a".to_string(),
                    DataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: Value::Float(0.0),
                        control: Control::Float,
                    },
                );
                graph.add_input(
                    node_id,
                    "b".to_string(),
                    DataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: Value::Float(0.0),
                        control: Control::Float,
                    },
                );
                graph.add_output(
                    node_id,
                    "c".to_string(),
                    DataType::Float,
                    OutputParameterKind::Computed,
                );
            }
            Self::Multiply => {
                graph.add_input(
                    node_id,
                    "a".to_string(),
                    DataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: Value::Float(0.0),
                        control: Control::Float,
                    },
                );
                graph.add_input(
                    node_id,
                    "b".to_string(),
                    DataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: Value::Float(0.0),
                        control: Control::Float,
                    },
                );
                graph.add_output(
                    node_id,
                    "c".to_string(),
                    DataType::Float,
                    OutputParameterKind::Computed,
                );
            }
            Self::Divide => {
                graph.add_input(
                    node_id,
                    "a".to_string(),
                    DataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: Value::Float(0.0),
                        control: Control::Float,
                    },
                );
                graph.add_input(
                    node_id,
                    "b".to_string(),
                    DataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: Value::Float(0.0),
                        control: Control::Float,
                    },
                );
                graph.add_output(
                    node_id,
                    "c".to_string(),
                    DataType::Float,
                    OutputParameterKind::Computed,
                );
            }
            Self::Floor => {
                graph.add_input(
                    node_id,
                    "value".to_string(),
                    DataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: Value::Float(0.0),
                        control: Control::Float,
                    },
                );
                graph.add_output(
                    node_id,
                    "floored".to_string(),
                    DataType::Int,
                    OutputParameterKind::Computed,
                );
            }
            Self::Round => {
                graph.add_input(
                    node_id,
                    "value".to_string(),
                    DataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: Value::Float(0.0),
                        control: Control::Float,
                    },
                );
                graph.add_output(
                    node_id,
                    "rounded".to_string(),
                    DataType::Int,
                    OutputParameterKind::Computed,
                );
            }
            Self::Ceil => {
                graph.add_input(
                    node_id,
                    "value".to_string(),
                    DataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: Value::Float(0.0),
                        control: Control::Float,
                    },
                );
                graph.add_output(
                    node_id,
                    "ceiled".to_string(),
                    DataType::Int,
                    OutputParameterKind::Computed,
                );
            }

            Self::GetFixture => {
                graph.add_output(
                    node_id,
                    "index".to_string(),
                    DataType::Int,
                    OutputParameterKind::Computed,
                );

                graph.add_output(
                    node_id,
                    "id".to_string(),
                    DataType::FixtureId,
                    OutputParameterKind::Computed,
                );
            }

            Self::SetChannelValue => {
                graph.add_input(
                    node_id,
                    "channel".to_string(),
                    DataType::DmxChannel,
                    InputParameterKind::EdgeOrConstant {
                        value: Value::DmxChannel(DmxChannel::default()),
                        control: Control::DmxChannel,
                    },
                );
                graph.add_input(
                    node_id,
                    "value".to_string(),
                    DataType::AttributeValue,
                    InputParameterKind::EdgeOrConstant {
                        value: Value::AttributeValue(AttributeValue::default()),
                        control: Control::AttributeValue,
                    },
                );
            }
        }
    }

    fn process(
        &self,
        node_id: NodeId,
        context: &mut Self::ProcessingContext,
        graph: &EffectGraph,
    ) -> Result<ProcessingResult<GraphDefinition>, GraphError> {
        let node = graph.node(node_id);
        let mut processing_result = ProcessingResult::new();

        let mut value_for_input =
            |node: &Node<GraphDefinition>, input_name: &str| -> Result<Value, GraphError> {
                let input = graph.input(node.input(input_name).id);
                let connection_id = graph.edge_source(input.id());
                let value = match connection_id {
                    None => {
                        let InputParameterKind::EdgeOrConstant { value, .. } =
                            graph.input(input.id()).kind.clone();
                        value
                    }
                    Some(id) => graph.get_output_value(&id, context)?.clone(),
                };
                let value = value.try_cast_to(&input.data_type())?;
                Ok(value)
            };

        match node.kind() {
            Self::NewFixtureId => {}
            Self::NewAttributeValue => {}

            Self::Add => {
                let Value::Float(a) = value_for_input(node, "a")? else {
                    return Err(GraphError::CastFailed);
                };

                let Value::Float(b) = value_for_input(node, "b")? else {
                    return Err(GraphError::CastFailed);
                };

                processing_result.set_value(node.output("c").id, Value::Float(a + b));
            }
            Self::Subtract => {
                let Value::Float(a) = value_for_input(node, "a")? else {
                    return Err(GraphError::CastFailed);
                };

                let Value::Float(b) = value_for_input(node, "b")? else {
                    return Err(GraphError::CastFailed);
                };

                processing_result.set_value(node.output("c").id, Value::Float(a - b));
            }
            Self::Multiply => {
                let Value::Float(a) = value_for_input(node, "a")? else {
                    return Err(GraphError::CastFailed);
                };

                let Value::Float(b) = value_for_input(node, "b")? else {
                    return Err(GraphError::CastFailed);
                };

                processing_result.set_value(node.output("c").id, Value::Float(a * b));
            }
            Self::Divide => {
                let Value::Float(a) = value_for_input(node, "a")? else {
                    return Err(GraphError::CastFailed);
                };

                let Value::Float(b) = value_for_input(node, "b")? else {
                    return Err(GraphError::CastFailed);
                };

                processing_result.set_value(node.output("c").id, Value::Float(a / b));
            }
            Self::Floor => {
                let Value::Float(a) = value_for_input(node, "value")? else {
                    return Err(GraphError::CastFailed);
                };

                processing_result
                    .set_value(node.output("floored").id, Value::Int(a.floor() as i32));
            }
            Self::Round => {
                let Value::Float(a) = value_for_input(node, "value")? else {
                    return Err(GraphError::CastFailed);
                };

                processing_result
                    .set_value(node.output("rounded").id, Value::Int(a.floor() as i32));
            }
            Self::Ceil => {
                let Value::Float(a) = value_for_input(node, "value")? else {
                    return Err(GraphError::CastFailed);
                };

                processing_result.set_value(node.output("ceiled").id, Value::Int(a.floor() as i32));
            }

            Self::GetFixture => {
                processing_result.set_value(
                    node.output("index").id,
                    Value::Int(context.current_fixture_index as i32),
                );

                processing_result.set_value(
                    node.output("id").id,
                    Value::FixtureId(context.current_fixture_id()),
                );
            }

            Self::SetChannelValue => {
                let Value::DmxChannel(channel) = value_for_input(node, "channel")? else {
                    return Err(GraphError::CastFailed);
                };

                let Value::AttributeValue(value) = value_for_input(node, "value")? else {
                    return Err(GraphError::CastFailed);
                };

                context
                    .dmx_output
                    .set_channel_value(0, channel, value.byte());
            }
        }

        Ok(processing_result)
    }
}

impl VisualNodeKind for NodeKind {
    type Category = NodeCategory;

    fn label(&self) -> &str {
        match self {
            Self::NewFixtureId => "New Fixture Id",
            Self::NewAttributeValue => "New Attribute ",

            Self::Add => "Add",
            Self::Subtract => "Subtract",
            Self::Multiply => "Multiply",
            Self::Divide => "Divide",
            Self::Floor => "Floor",
            Self::Round => "Round",
            Self::Ceil => "Ceil",

            Self::GetFixture => "Get Fixture",

            Self::SetChannelValue => "Set Channel Value",
        }
    }

    fn category(&self) -> Self::Category {
        match self {
            Self::NewFixtureId | Self::NewAttributeValue => NodeCategory::NewValue,
            Self::Add
            | Self::Subtract
            | Self::Multiply
            | Self::Divide
            | Self::Floor
            | Self::Round
            | Self::Ceil => NodeCategory::Math,
            Self::GetFixture => NodeCategory::Context,
            Self::SetChannelValue => NodeCategory::Output,
        }
    }

    fn all() -> impl Iterator<Item = Self> {
        Self::iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, strum::EnumIter)]
pub enum NodeCategory {
    NewValue,
    Math,
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
            NodeCategory::Context => "Context",
            NodeCategory::Output => "Output",
        }
        .to_string();
        write!(f, "{}", str)
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

    pub fn set_group(&mut self, group: FixtureGroup) {
        self.group = group;
    }

    pub fn process_frame(&mut self, graph: &EffectGraph) -> Result<(), GraphError> {
        self.current_fixture_index = 0;
        while self.current_fixture_index < self.group.len() {
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Value {
    Int(i32),
    Float(f32),
    FixtureId(FixtureId),
    AttributeValue(AttributeValue),
    DmxChannel(DmxChannel),
}

impl flow::Value<GraphDefinition> for Value {
    fn try_cast_to(&self, target: &DataType) -> Result<Self, GraphError> {
        use DataType as DT;
        match (self, target) {
            (Self::Int(_), DT::Int) => Ok(self.clone()),
            (Self::Int(v), DT::Float) => Ok(Self::Float(*v as f32)),
            (Self::Int(v), DT::FixtureId) => Ok(Self::FixtureId(FixtureId(*v as u32))),
            (Self::Int(v), DT::DmxChannel) => Ok(Self::DmxChannel(
                DmxChannel::new(*v as u16).map_err(|_| GraphError::CastFailed)?,
            )),
            (Self::Int(v), DT::AttributeValue) => {
                Ok(Self::AttributeValue(AttributeValue::new(*v as f32)))
            }

            (Self::Float(_), DT::Float) => Ok(self.clone()),
            (Self::Float(v), DT::Int) => Ok(Self::Int(*v as i32)),
            (Self::Float(v), DT::AttributeValue) => {
                Ok(Self::AttributeValue(AttributeValue::new(*v)))
            }

            (Self::FixtureId(_), DT::FixtureId) => Ok(self.clone()),
            (Self::FixtureId(v), DT::Int) => Ok(Self::Int(v.0 as i32)),
            (Self::FixtureId(v), DT::Float) => Ok(Self::Float(v.0 as f32)),
            (Self::FixtureId(v), DT::DmxChannel) => Ok(Self::DmxChannel(
                DmxChannel::new(v.id() as u16).map_err(|_| GraphError::CastFailed)?,
            )),

            (Self::AttributeValue(_), DT::AttributeValue) => Ok(self.clone()),
            (Self::AttributeValue(v), DT::Int) => Ok(Self::Int(v.byte() as i32)),
            (Self::AttributeValue(v), DT::Float) => Ok(Self::Float(v.byte() as f32)),

            (Self::DmxChannel(_), DT::DmxChannel) => Ok(self.clone()),
            (Self::DmxChannel(v), DT::Int) => Ok(Self::Int(v.value() as i32)),
            (Self::DmxChannel(v), DT::Float) => Ok(Self::Float(v.value() as f32)),

            _ => Err(GraphError::CastFailed),
        }
    }
}

impl TryFrom<Value> for i32 {
    type Error = GraphError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(value) => Ok(value),
            _ => Err(GraphError::CastFailed),
        }
    }
}

impl TryFrom<Value> for f32 {
    type Error = GraphError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(value) => Ok(value),
            _ => Err(GraphError::CastFailed),
        }
    }
}

impl TryFrom<Value> for FixtureId {
    type Error = GraphError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::FixtureId(value) => Ok(value),
            _ => Err(GraphError::CastFailed),
        }
    }
}

impl TryFrom<Value> for AttributeValue {
    type Error = GraphError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::AttributeValue(value) => Ok(value),
            _ => Err(GraphError::CastFailed),
        }
    }
}

impl TryFrom<Value> for DmxChannel {
    type Error = GraphError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::DmxChannel(value) => Ok(value),
            _ => Err(GraphError::CastFailed),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DataType {
    Int,
    Float,

    FixtureId,
    AttributeValue,
    DmxChannel,
}

impl flow::DataType<GraphDefinition> for DataType {
    fn default_value(&self) -> Value {
        match self {
            Self::Int => Value::Int(i32::default()),
            Self::Float => Value::Float(f32::default()),
            Self::FixtureId => Value::FixtureId(FixtureId::default()),
            Self::AttributeValue => Value::AttributeValue(AttributeValue::default()),
            Self::DmxChannel => Value::DmxChannel(DmxChannel::default()),
        }
    }
}

impl VisualDataType for DataType {
    fn color(&self) -> Hsla {
        match self {
            Self::Int => rgb(0xC741FF).into(),
            Self::Float => rgb(0xFF3C59).into(),

            Self::FixtureId => rgb(0x080AFF).into(),
            Self::AttributeValue => rgb(0xFFAE18).into(),
            Self::DmxChannel => rgb(0xFF0000).into(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Control {
    Int,
    Float,
    FixtureId,
    AttributeValue,
    DmxChannel,
}

impl flow::Control<GraphDefinition> for Control {}

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
        }
    }
}
