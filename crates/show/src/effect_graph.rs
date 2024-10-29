use crate::fixture::{AttributeValue, FixtureId};
use flow::error::GraphError;
use flow::graph::Graph;
use flow::graph_def::{Control, DataType, GraphDefinition, NodeKind, ProcessingResult, Value};
use flow::{InputParameterKind, NodeId, OutputParameterKind};
use flow_gpui::node::ControlEvent;
use flow_gpui::{VisualControl, VisualDataType, VisualNodeData};
use gpui::{
    rgb, AnyView, ElementId, EventEmitter, Hsla, Pixels, Point, ViewContext, VisualContext,
};
use strum::IntoEnumIterator;
use ui::input::{NumberField, Slider, SliderEvent};

#[derive(Clone)]
pub struct EffectGraphDefinition {}

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
}

impl NodeKind<EffectGraphDefinition> for EffectGraphNodeKind {
    type ProcessingContext = ();

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
                    InputParameterKind::RequiresEdge,
                );
                graph.add_input(
                    node_id,
                    "ColorAdd_R".to_string(),
                    EffectGraphDataType::AttributeValue,
                    InputParameterKind::RequiresEdge,
                );
                graph.add_input(
                    node_id,
                    "ColorAdd_G".to_string(),
                    EffectGraphDataType::AttributeValue,
                    InputParameterKind::RequiresEdge,
                );
                graph.add_input(
                    node_id,
                    "ColorAdd_B".to_string(),
                    EffectGraphDataType::AttributeValue,
                    InputParameterKind::RequiresEdge,
                );
            }
        }
    }

    fn process(
        &self,
        _node_id: NodeId,
        _context: &mut Self::ProcessingContext,
        _graph: &EffectGraph,
    ) -> Result<ProcessingResult, GraphError> {
        todo!();
    }

    fn label(&self) -> &'static str {
        match self {
            Self::FixtureId => "Get Fixture Id",
            Self::AttributeValue => "Get Attribute Value",
            Self::SetFixtureAttribute => "Set Fixture Attribute",
        }
    }

    fn all() -> impl Iterator<Item = Self> {
        Self::iter()
    }
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

#[derive(Debug, Clone, PartialEq)]
pub enum EffectGraphDataType {
    FixtureId,
    AttributeValue,
}

impl DataType<EffectGraphDefinition> for EffectGraphDataType {
    fn default_value(&self) -> EffectGraphValue {
        match self {
            Self::FixtureId => EffectGraphValue::FixtureId(FixtureId::default()),
            Self::AttributeValue => EffectGraphValue::AttributeValue(AttributeValue::default()),
        }
    }
}

impl VisualDataType for EffectGraphDataType {
    fn color(&self) -> Hsla {
        match self {
            EffectGraphDataType::FixtureId => rgb(0x080AFF).into(),
            EffectGraphDataType::AttributeValue => rgb(0xFFAE18).into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EffectGraphControl {
    FixtureId,
    AttributeValue,
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
        }
    }
}
