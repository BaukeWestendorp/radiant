use crate::fixture::{AttributeValue, FixtureId};
use crate::FixtureGroup;
use dmx::{DmxChannel, DmxOutput};
use flow::error::GraphError;
use flow::graph::Graph;
use flow::graph_def::{Control, DataType, GraphDefinition, NodeKind, ProcessingResult, Value};
use flow::node::Node;
use flow::{InputParameterKind, NodeId, OutputParameterKind};
use flow_gpui::node::ControlEvent;
use flow_gpui::{VisualControl, VisualDataType, VisualNodeData, VisualNodeKind};
use gpui::{rgb, AnyView, ElementId, EventEmitter, Hsla, ViewContext, VisualContext};
use strum::IntoEnumIterator;
use ui::input::{NumberField, Slider, SliderEvent};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct EffectGraphDefinition;

impl GraphDefinition for EffectGraphDefinition {
    type NodeKind = EffectGraphNodeKind;
    type NodeData = EffectGraphNodeData;
    type Value = EffectGraphValue;
    type DataType = EffectGraphDataType;
    type Control = EffectGraphControl;
}

pub type EffectGraph = Graph<EffectGraphDefinition>;

#[derive(Debug, Clone, PartialEq, strum::EnumIter, serde::Serialize, serde::Deserialize)]
pub enum EffectGraphNodeKind {
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

impl NodeKind<EffectGraphDefinition> for EffectGraphNodeKind {
    type ProcessingContext = EffectGraphProcessingContext;

    fn build(&self, graph: &mut EffectGraph, node_id: NodeId) {
        match self {
            Self::NewFixtureId => {
                graph.add_output(
                    node_id,
                    "id".to_string(),
                    EffectGraphDataType::FixtureId,
                    OutputParameterKind::Constant {
                        value: EffectGraphValue::FixtureId(FixtureId::default()),
                        control: EffectGraphControl::FixtureId,
                    },
                );
            }
            Self::NewAttributeValue => {
                graph.add_output(
                    node_id,
                    "value".to_string(),
                    EffectGraphDataType::AttributeValue,
                    OutputParameterKind::Constant {
                        value: EffectGraphValue::AttributeValue(AttributeValue::default()),
                        control: EffectGraphControl::AttributeValue,
                    },
                );
            }

            Self::Add => {
                graph.add_input(
                    node_id,
                    "a".to_string(),
                    EffectGraphDataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: EffectGraphValue::Float(0.0),
                        control: EffectGraphControl::Float,
                    },
                );
                graph.add_input(
                    node_id,
                    "b".to_string(),
                    EffectGraphDataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: EffectGraphValue::Float(0.0),
                        control: EffectGraphControl::Float,
                    },
                );
                graph.add_output(
                    node_id,
                    "c".to_string(),
                    EffectGraphDataType::Float,
                    OutputParameterKind::Computed,
                );
            }
            Self::Subtract => {
                graph.add_input(
                    node_id,
                    "a".to_string(),
                    EffectGraphDataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: EffectGraphValue::Float(0.0),
                        control: EffectGraphControl::Float,
                    },
                );
                graph.add_input(
                    node_id,
                    "b".to_string(),
                    EffectGraphDataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: EffectGraphValue::Float(0.0),
                        control: EffectGraphControl::Float,
                    },
                );
                graph.add_output(
                    node_id,
                    "c".to_string(),
                    EffectGraphDataType::Float,
                    OutputParameterKind::Computed,
                );
            }
            Self::Multiply => {
                graph.add_input(
                    node_id,
                    "a".to_string(),
                    EffectGraphDataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: EffectGraphValue::Float(0.0),
                        control: EffectGraphControl::Float,
                    },
                );
                graph.add_input(
                    node_id,
                    "b".to_string(),
                    EffectGraphDataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: EffectGraphValue::Float(0.0),
                        control: EffectGraphControl::Float,
                    },
                );
                graph.add_output(
                    node_id,
                    "c".to_string(),
                    EffectGraphDataType::Float,
                    OutputParameterKind::Computed,
                );
            }
            Self::Divide => {
                graph.add_input(
                    node_id,
                    "a".to_string(),
                    EffectGraphDataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: EffectGraphValue::Float(0.0),
                        control: EffectGraphControl::Float,
                    },
                );
                graph.add_input(
                    node_id,
                    "b".to_string(),
                    EffectGraphDataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: EffectGraphValue::Float(0.0),
                        control: EffectGraphControl::Float,
                    },
                );
                graph.add_output(
                    node_id,
                    "c".to_string(),
                    EffectGraphDataType::Float,
                    OutputParameterKind::Computed,
                );
            }
            Self::Floor => {
                graph.add_input(
                    node_id,
                    "value".to_string(),
                    EffectGraphDataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: EffectGraphValue::Float(0.0),
                        control: EffectGraphControl::Float,
                    },
                );
                graph.add_output(
                    node_id,
                    "floored".to_string(),
                    EffectGraphDataType::Int,
                    OutputParameterKind::Computed,
                );
            }
            Self::Round => {
                graph.add_input(
                    node_id,
                    "value".to_string(),
                    EffectGraphDataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: EffectGraphValue::Float(0.0),
                        control: EffectGraphControl::Float,
                    },
                );
                graph.add_output(
                    node_id,
                    "rounded".to_string(),
                    EffectGraphDataType::Int,
                    OutputParameterKind::Computed,
                );
            }
            Self::Ceil => {
                graph.add_input(
                    node_id,
                    "value".to_string(),
                    EffectGraphDataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: EffectGraphValue::Float(0.0),
                        control: EffectGraphControl::Float,
                    },
                );
                graph.add_output(
                    node_id,
                    "ceiled".to_string(),
                    EffectGraphDataType::Int,
                    OutputParameterKind::Computed,
                );
            }

            Self::GetFixture => {
                graph.add_output(
                    node_id,
                    "index".to_string(),
                    EffectGraphDataType::Int,
                    OutputParameterKind::Computed,
                );
            }

            Self::SetChannelValue => {
                graph.add_input(
                    node_id,
                    "channel".to_string(),
                    EffectGraphDataType::DmxChannel,
                    InputParameterKind::EdgeOrConstant {
                        value: EffectGraphValue::DmxChannel(DmxChannel::default()),
                        control: EffectGraphControl::DmxChannel,
                    },
                );
                graph.add_input(
                    node_id,
                    "value".to_string(),
                    EffectGraphDataType::AttributeValue,
                    InputParameterKind::EdgeOrConstant {
                        value: EffectGraphValue::AttributeValue(AttributeValue::default()),
                        control: EffectGraphControl::AttributeValue,
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
    ) -> Result<ProcessingResult<EffectGraphDefinition>, GraphError> {
        let node = graph.node(node_id);
        let mut processing_result = ProcessingResult::new();

        let mut value_for_input = |node: &Node<EffectGraphDefinition>,
                                   input_name: &str|
         -> Result<EffectGraphValue, GraphError> {
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
                let EffectGraphValue::Float(a) = value_for_input(node, "a")? else {
                    return Err(GraphError::CastFailed);
                };

                let EffectGraphValue::Float(b) = value_for_input(node, "b")? else {
                    return Err(GraphError::CastFailed);
                };

                processing_result.set_value(node.output("c").id, EffectGraphValue::Float(a + b));
            }
            Self::Subtract => {
                let EffectGraphValue::Float(a) = value_for_input(node, "a")? else {
                    return Err(GraphError::CastFailed);
                };

                let EffectGraphValue::Float(b) = value_for_input(node, "b")? else {
                    return Err(GraphError::CastFailed);
                };

                processing_result.set_value(node.output("c").id, EffectGraphValue::Float(a - b));
            }
            Self::Multiply => {
                let EffectGraphValue::Float(a) = value_for_input(node, "a")? else {
                    return Err(GraphError::CastFailed);
                };

                let EffectGraphValue::Float(b) = value_for_input(node, "b")? else {
                    return Err(GraphError::CastFailed);
                };

                processing_result.set_value(node.output("c").id, EffectGraphValue::Float(a * b));
            }
            Self::Divide => {
                let EffectGraphValue::Float(a) = value_for_input(node, "a")? else {
                    return Err(GraphError::CastFailed);
                };

                let EffectGraphValue::Float(b) = value_for_input(node, "b")? else {
                    return Err(GraphError::CastFailed);
                };

                processing_result.set_value(node.output("c").id, EffectGraphValue::Float(a / b));
            }
            Self::Floor => {
                let EffectGraphValue::Float(a) = value_for_input(node, "value")? else {
                    return Err(GraphError::CastFailed);
                };

                processing_result.set_value(
                    node.output("floored").id,
                    EffectGraphValue::Int(a.floor() as i32),
                );
            }
            Self::Round => {
                let EffectGraphValue::Float(a) = value_for_input(node, "value")? else {
                    return Err(GraphError::CastFailed);
                };

                processing_result.set_value(
                    node.output("rounded").id,
                    EffectGraphValue::Int(a.floor() as i32),
                );
            }
            Self::Ceil => {
                let EffectGraphValue::Float(a) = value_for_input(node, "value")? else {
                    return Err(GraphError::CastFailed);
                };

                processing_result.set_value(
                    node.output("ceiled").id,
                    EffectGraphValue::Int(a.floor() as i32),
                );
            }

            Self::GetFixture => {
                processing_result.set_value(
                    node.output("index").id,
                    EffectGraphValue::Int(context.current_fixture_index as i32),
                );
            }

            Self::SetChannelValue => {
                let EffectGraphValue::DmxChannel(channel) = value_for_input(node, "channel")?
                else {
                    return Err(GraphError::CastFailed);
                };

                let EffectGraphValue::AttributeValue(value) = value_for_input(node, "value")?
                else {
                    return Err(GraphError::CastFailed);
                };

                context
                    .dmx_output
                    .set_channel_value(0, channel, value.byte())
            }
        }

        Ok(processing_result)
    }
}

impl VisualNodeKind for EffectGraphNodeKind {
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

impl flow_gpui::NodeCategory for NodeCategory {
    fn all() -> impl Iterator<Item = Self> {
        Self::iter()
    }
}

impl ToString for NodeCategory {
    fn to_string(&self) -> String {
        match self {
            NodeCategory::NewValue => "New Value",
            NodeCategory::Math => "Math",
            NodeCategory::Context => "Context",
            NodeCategory::Output => "Output",
        }
        .to_string()
    }
}

#[derive(Clone, Default)]
pub struct EffectGraphProcessingContext {
    pub dmx_output: DmxOutput,
    current_fixture_index: usize,
    group: FixtureGroup,
}

impl EffectGraphProcessingContext {
    pub fn set_group(&mut self, group: FixtureGroup) {
        self.group = group;
    }

    pub fn process_frame(&mut self, graph: &mut EffectGraph) -> Result<(), GraphError> {
        self.current_fixture_index = 0;
        while self.current_fixture_index < self.group.len() {
            graph.process(self)?;
            self.current_fixture_index += 1;
        }
        Ok(())
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct EffectGraphNodeData {
    pub position: geo::Point,
}

impl VisualNodeData for EffectGraphNodeData {
    fn position(&self) -> &geo::Point {
        &self.position
    }

    fn set_position(&mut self, position: geo::Point) {
        self.position = position;
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum EffectGraphValue {
    Int(i32),
    Float(f32),
    FixtureId(FixtureId),
    AttributeValue(AttributeValue),
    DmxChannel(DmxChannel),
}

impl Value<EffectGraphDefinition> for EffectGraphValue {
    fn try_cast_to(&self, target: &EffectGraphDataType) -> Result<Self, GraphError> {
        use EffectGraphDataType as DT;
        match (self, target) {
            (Self::Int(_), DT::Int) => Ok(self.clone()),
            (Self::Int(v), DT::Float) => Ok(Self::Float(*v as f32)),
            (Self::Int(v), DT::FixtureId) => Ok(Self::FixtureId(FixtureId(*v as u32))),
            (Self::Int(v), DT::DmxChannel) => Ok(Self::DmxChannel(
                DmxChannel::new(*v as u16).map_err(|_| GraphError::CastFailed)?,
            )),

            (Self::Float(_), DT::Float) => Ok(self.clone()),
            (Self::Float(v), DT::Int) => Ok(Self::Int(*v as i32)),

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

impl TryFrom<EffectGraphValue> for FixtureId {
    type Error = GraphError;

    fn try_from(value: EffectGraphValue) -> Result<Self, Self::Error> {
        match value {
            EffectGraphValue::FixtureId(value) => Ok(value),
            _ => Err(GraphError::CastFailed),
        }
    }
}

impl TryFrom<EffectGraphValue> for AttributeValue {
    type Error = GraphError;

    fn try_from(value: EffectGraphValue) -> Result<Self, Self::Error> {
        match value {
            EffectGraphValue::AttributeValue(value) => Ok(value),
            _ => Err(GraphError::CastFailed),
        }
    }
}

impl TryFrom<EffectGraphValue> for DmxChannel {
    type Error = GraphError;

    fn try_from(value: EffectGraphValue) -> Result<Self, Self::Error> {
        match value {
            EffectGraphValue::DmxChannel(value) => Ok(value),
            _ => Err(GraphError::CastFailed),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum EffectGraphDataType {
    Int,
    Float,

    FixtureId,
    AttributeValue,
    DmxChannel,
}

impl DataType<EffectGraphDefinition> for EffectGraphDataType {
    fn default_value(&self) -> EffectGraphValue {
        match self {
            Self::Int => EffectGraphValue::Int(i32::default()),
            Self::Float => EffectGraphValue::Float(f32::default()),
            Self::FixtureId => EffectGraphValue::FixtureId(FixtureId::default()),
            Self::AttributeValue => EffectGraphValue::AttributeValue(AttributeValue::default()),
            Self::DmxChannel => EffectGraphValue::DmxChannel(DmxChannel::default()),
        }
    }
}

impl VisualDataType for EffectGraphDataType {
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
pub enum EffectGraphControl {
    Int,
    Float,
    FixtureId,
    AttributeValue,
    DmxChannel,
}

impl Control<EffectGraphDefinition> for EffectGraphControl {}

impl VisualControl<EffectGraphDefinition> for EffectGraphControl {
    fn view<View: EventEmitter<ControlEvent<EffectGraphDefinition>>>(
        &self,
        id: impl Into<ElementId>,
        initial_value: EffectGraphValue,
        cx: &mut ViewContext<View>,
    ) -> AnyView {
        use ui::input::NumberFieldEvent;

        match self {
            Self::Int => {
                let field = cx.new_view(|cx| {
                    let field = NumberField::new(cx);
                    field.set_validate(Some(Box::new(|v| v.parse::<i32>().is_ok())), cx);
                    field
                });

                cx.subscribe(&field, |_this, _field, event: &NumberFieldEvent, cx| {
                    let NumberFieldEvent::Change(float_value) = event;
                    let value = EffectGraphValue::Int(*float_value as i32);
                    cx.emit(ControlEvent::Change(value));
                })
                .detach();

                field.into()
            }
            Self::Float => {
                let field = cx.new_view(|cx| {
                    let field = NumberField::new(cx);
                    field.set_validate(Some(Box::new(|v| v.parse::<f32>().is_ok())), cx);
                    field
                });

                cx.subscribe(&field, |_this, _field, event: &NumberFieldEvent, cx| {
                    let NumberFieldEvent::Change(float_value) = event;
                    let value = EffectGraphValue::Float(*float_value);
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
                    let value = EffectGraphValue::FixtureId(FixtureId(*float_value as u32));
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
                    cx.emit(ControlEvent::Change(EffectGraphValue::AttributeValue(
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
                    let value = EffectGraphValue::DmxChannel(
                        DmxChannel::new(*float_value as u16).unwrap_or_default(),
                    );
                    cx.emit(ControlEvent::Change(value));
                })
                .detach();

                field.into()
            }
        }
    }
}
