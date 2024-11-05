use crate::fixture::{AttributeValue, FixtureId};
use dmx::{DmxChannel, DmxOutput};
use flow::error::GraphError;
use flow::graph::Graph;
use flow::graph_def::{Control, DataType, GraphDefinition, NodeKind, ProcessingResult, Value};
use flow::node::Node;
use flow::{InputParameterKind, NodeId, OutputParameterKind};
use flow_gpui::node::ControlEvent;
use flow_gpui::{VisualControl, VisualDataType, VisualNodeData};
use gpui::{
    rgb, AnyView, ElementId, EventEmitter, Hsla, Pixels, Point, ViewContext, VisualContext,
};
use strum::IntoEnumIterator;
use ui::input::{NumberField, Slider, SliderEvent};

#[derive(Clone)]
pub struct EffectGraphDefinition;

impl GraphDefinition for EffectGraphDefinition {
    type NodeKind = EffectGraphNodeKind;
    type NodeData = EffectGraphNodeData;
    type Value = EffectGraphValue;
    type DataType = EffectGraphDataType;
    type Control = EffectGraphControl;
}

pub type EffectGraph = Graph<EffectGraphDefinition>;

#[derive(Debug, Clone, PartialEq, strum::EnumIter)]
pub enum EffectGraphNodeKind {
    FixtureId,
    AttributeValue,
    SetFixtureAttribute,
    SetFixtureChannelValue,
}

impl NodeKind<EffectGraphDefinition> for EffectGraphNodeKind {
    type ProcessingContext = EffectGraphProcessingContext;

    fn build(&self, graph: &mut EffectGraph, node_id: NodeId) {
        match self {
            Self::FixtureId => {
                graph.add_output(
                    node_id,
                    "id".to_string(),
                    EffectGraphDataType::FixtureId,
                    OutputParameterKind::Computed,
                );
            }
            Self::AttributeValue => {
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
            Self::SetFixtureAttribute => {
                graph.add_input(
                    node_id,
                    "id".to_string(),
                    EffectGraphDataType::FixtureId,
                    InputParameterKind::EdgeOrConstant {
                        value: EffectGraphValue::FixtureId(FixtureId::default()),
                        control: EffectGraphControl::FixtureId,
                    },
                );
                graph.add_input(
                    node_id,
                    "ColorAdd_R".to_string(),
                    EffectGraphDataType::AttributeValue,
                    InputParameterKind::EdgeOrConstant {
                        value: EffectGraphValue::AttributeValue(AttributeValue::default()),
                        control: EffectGraphControl::AttributeValue,
                    },
                );
                graph.add_input(
                    node_id,
                    "ColorAdd_G".to_string(),
                    EffectGraphDataType::AttributeValue,
                    InputParameterKind::EdgeOrConstant {
                        value: EffectGraphValue::AttributeValue(AttributeValue::default()),
                        control: EffectGraphControl::AttributeValue,
                    },
                );
                graph.add_input(
                    node_id,
                    "ColorAdd_B".to_string(),
                    EffectGraphDataType::AttributeValue,
                    InputParameterKind::EdgeOrConstant {
                        value: EffectGraphValue::AttributeValue(AttributeValue::default()),
                        control: EffectGraphControl::AttributeValue,
                    },
                );
            }
            Self::SetFixtureChannelValue => {
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
        let processing_result = ProcessingResult::<EffectGraphDefinition>::new();

        let mut value_for_input = |node: &Node<EffectGraphDefinition>,
                                   input_name: &str|
         -> Result<EffectGraphValue, GraphError> {
            let input_id = node.input(input_name).id;
            let connection_id = graph.edge_source(input_id);
            let value = match connection_id {
                None => {
                    let InputParameterKind::EdgeOrConstant { value, .. } =
                        graph.input(input_id).kind.clone();
                    value
                }
                Some(id) => graph.get_output_value(&id, context)?.clone(),
            };
            Ok(value)
        };

        match node.kind() {
            EffectGraphNodeKind::FixtureId => {}
            EffectGraphNodeKind::AttributeValue => {}
            EffectGraphNodeKind::SetFixtureAttribute => {}
            EffectGraphNodeKind::SetFixtureChannelValue => {
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

    fn label(&self) -> &'static str {
        match self {
            Self::FixtureId => "Get Fixture Id",
            Self::AttributeValue => "Get Attribute Value",
            Self::SetFixtureAttribute => "Set Fixture Attribute",
            Self::SetFixtureChannelValue => "Set Fixture Channel Value",
        }
    }

    fn all() -> impl Iterator<Item = Self> {
        Self::iter()
    }
}

#[derive(Clone, Default)]
pub struct EffectGraphProcessingContext {
    pub dmx_output: DmxOutput,
}

#[derive(Clone, Default)]
pub struct EffectGraphNodeData {
    pub position: Point<Pixels>,
}

impl VisualNodeData for EffectGraphNodeData {
    fn position(&self) -> &Point<Pixels> {
        &self.position
    }

    fn set_position(&mut self, position: Point<Pixels>) {
        self.position = position;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EffectGraphValue {
    FixtureId(FixtureId),
    AttributeValue(AttributeValue),
    DmxChannel(DmxChannel),
}

impl Value<EffectGraphDefinition> for EffectGraphValue {
    fn try_cast_to(&self, target: &EffectGraphDataType) -> Result<Self, GraphError> {
        match (self, target) {
            (Self::FixtureId(_), EffectGraphDataType::FixtureId) => Ok(self.clone()),
            (Self::AttributeValue(_), EffectGraphDataType::AttributeValue) => Ok(self.clone()),

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

#[derive(Debug, Clone, PartialEq)]
pub enum EffectGraphDataType {
    FixtureId,
    AttributeValue,
    DmxChannel,
}

impl DataType<EffectGraphDefinition> for EffectGraphDataType {
    fn default_value(&self) -> EffectGraphValue {
        match self {
            Self::FixtureId => EffectGraphValue::FixtureId(FixtureId::default()),
            Self::AttributeValue => EffectGraphValue::AttributeValue(AttributeValue::default()),
            Self::DmxChannel => EffectGraphValue::DmxChannel(DmxChannel::default()),
        }
    }
}

impl VisualDataType for EffectGraphDataType {
    fn color(&self) -> Hsla {
        match self {
            Self::FixtureId => rgb(0x080AFF).into(),
            Self::AttributeValue => rgb(0xFFAE18).into(),
            Self::DmxChannel => rgb(0xFF0000).into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EffectGraphControl {
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
