#![cfg(all(not(feature = "serde"), feature = "gpui"))]

use flow::gpui::ControlEvent;
use flow::{
    Graph, GraphError, InputParameterKind, Node, NodeId, OutputParameterKind, ProcessingResult,
    Value as _,
};
use gpui::{AnyView, ElementId, EmptyView, EventEmitter, Hsla, ViewContext, VisualContext};
use std::fmt::Formatter;

#[derive(Clone)]
pub struct GraphDefinition;

impl flow::GraphDefinition for GraphDefinition {
    type NodeKind = NodeKind;
    type NodeData = NodeData;
    type Value = Value;
    type DataType = DataType;

    type ProcessingContext = TestGraphProcessingContext;

    type NodeCategory = NodeCategory;
    type Control = Control;
}

pub type TestGraph = Graph<GraphDefinition>;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Add,
    Output,
}

impl flow::NodeKind<GraphDefinition> for NodeKind {
    fn build(&self, graph: &mut TestGraph, node_id: NodeId) {
        match self {
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
                    "sum".to_string(),
                    DataType::Float,
                    OutputParameterKind::Computed,
                );
            }
            Self::Output => {
                graph.add_input(
                    node_id,
                    "value".to_string(),
                    DataType::Float,
                    InputParameterKind::EdgeOrConstant {
                        value: Value::Float(0.0),
                        control: Control::Float,
                    },
                );
            }
        }
    }

    fn process(
        &self,
        node_id: NodeId,
        context: &mut TestGraphProcessingContext,
        graph: &TestGraph,
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
            Self::Add => {
                let Value::Float(a) = value_for_input(node, "a")?;
                let Value::Float(b) = value_for_input(node, "b")?;
                processing_result.set_output_value(node.output("sum").id, Value::Float(a + b));
            }
            Self::Output => {
                let value = value_for_input(node, "value")?;
                match value {
                    Value::Float(float_value) => context.output_float = float_value,
                }
            }
        }

        Ok(processing_result)
    }

    fn name(&self) -> &str {
        match self {
            NodeKind::Add => "Add",
            NodeKind::Output => "Output",
        }
    }

    fn category(&self) -> NodeCategory {
        match self {
            NodeKind::Add => NodeCategory::Math,
            NodeKind::Output => NodeCategory::Output,
        }
    }

    fn all() -> impl Iterator<Item = Self> {
        vec![Self::Add, Self::Output, Self::Output].into_iter()
    }
}

pub struct TestGraphProcessingContext {
    pub output_float: f32,
}

#[derive(Clone, Default)]
pub struct NodeData {
    position: geo::Point,
}

impl flow::NodeData for NodeData {
    fn position(&self) -> &geo::Point {
        &self.position
    }

    fn set_position(&mut self, position: geo::Point) {
        self.position = position;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Float(f32),
}

impl flow::Value<GraphDefinition> for Value {
    fn try_cast_to(&self, target: &DataType) -> Result<Self, GraphError> {
        use DataType as DT;
        match (self, target) {
            (Self::Float(_), DT::Float) => Ok(self.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Float,
}

impl flow::DataType<GraphDefinition> for DataType {
    fn default_value(&self) -> Value {
        match self {
            DataType::Float => Value::Float(f32::default()),
        }
    }

    fn color(&self) -> Hsla {
        match self {
            DataType::Float => gpui::rgb(0xff0000).into(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum NodeCategory {
    Math,
    Output,
}

impl flow::gpui::NodeCategory for NodeCategory {
    fn all() -> impl Iterator<Item = Self> {
        vec![Self::Math, Self::Output].into_iter()
    }
}

impl std::fmt::Display for NodeCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Math => "Math",
                Self::Output => "Output",
            }
        )
    }
}

#[derive(Clone)]
pub enum Control {
    Float,
}

impl flow::gpui::Control<GraphDefinition> for Control {
    fn view<View: EventEmitter<ControlEvent<GraphDefinition>>>(
        &self,
        _id: impl Into<ElementId>,
        _initial_value: Value,
        cx: &mut ViewContext<View>,
    ) -> AnyView {
        match self {
            Control::Float => cx.new_view(|_cx| EmptyView).into(),
        }
    }
}

#[test]
fn process_graph() {
    let mut graph = TestGraph::new();
    let add_node_id = graph.add_node(NodeKind::Add, NodeData::default());
    let output_node_id = graph.add_node(NodeKind::Output, NodeData::default());

    match &mut graph.input_mut(graph.node(add_node_id).input("a").id).kind {
        InputParameterKind::EdgeOrConstant { value, .. } => {
            *value = Value::Float(42.0);
        }
    };

    match &mut graph.input_mut(graph.node(add_node_id).input("b").id).kind {
        InputParameterKind::EdgeOrConstant { value, .. } => {
            *value = Value::Float(2.0);
        }
    };

    graph.add_edge(
        graph.output(graph.node(add_node_id).output("sum").id).id(),
        graph
            .input(graph.node(output_node_id).input("value").id)
            .id(),
    );

    let mut context = TestGraphProcessingContext { output_float: 0.0 };
    graph.process(&mut context).unwrap();

    assert_eq!(context.output_float, 44.0);
}
