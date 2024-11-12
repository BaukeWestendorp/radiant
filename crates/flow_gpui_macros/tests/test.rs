use std::fmt::Display;

use flow::{Control, DataType, Graph, GraphDefinition, GraphError, InputParameterKind, Value};
use flow_gpui::{ControlEvent, NodeCategory, VisualControl, VisualDataType, VisualNodeData};
use gpui::{rgb, AnyView, ElementId, EmptyView, EventEmitter, Hsla, ViewContext, VisualContext};

#[derive(Clone)]
pub struct TestGraphDefinition;

impl GraphDefinition for TestGraphDefinition {
    type NodeKind = TestGraphNodeKind;
    type NodeData = TestGraphNodeData;
    type Value = TestGraphValue;
    type DataType = TestGraphDataType;
    type Control = TestGraphControl;
}

pub type TestGraph = Graph<TestGraphDefinition>;

#[derive(Clone, serde::Serialize, serde::Deserialize, flow_gpui_macros::NodeKind)]
#[node_kind(
    graph_definition = "TestGraphDefinition",
    processing_context = "TestGraphProcessingContext",
    category = "Category"
)]
pub enum TestGraphNodeKind {
    #[input(
        label = "a",
        data_type = "Float",
        default_value = "0.0",
        control = "Float"
    )]
    #[input(
        label = "b",
        data_type = "Float",
        default_value = "0.0",
        control = "Float"
    )]
    #[computed_output(label = "sum", data_type = "Float")]
    #[meta(name = "Add", category = "Math", processor = "add_processor")]
    Add,
}

fn add_processor(
    input: AddProcessorInput,
    context: &mut TestGraphProcessingContext,
) -> Result<AddProcessorOutput, GraphError> {
    let AddProcessorInput { a, b } = input;

    let a: f32 = a.try_into()?;
    let b: f32 = b.try_into()?;

    Ok(AddProcessorOutput {
        sum: TestGraphValue::Float(a + b),
    })
}

#[derive(Clone, Copy, PartialEq)]
pub enum Category {
    Math,
}

impl NodeCategory for Category {
    fn all() -> impl Iterator<Item = Self> {
        vec![Self::Math].into_iter()
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Category::Math => "Math",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

pub struct TestGraphProcessingContext {
    pub output_float: f32,
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TestGraphNodeData {
    pub position: geo::Point,
}

impl VisualNodeData for TestGraphNodeData {
    fn position(&self) -> &geo::Point {
        &self.position
    }

    fn set_position(&mut self, position: geo::Point) {
        self.position = position;
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TestGraphValue {
    Float(f32),
}

impl Value<TestGraphDefinition> for TestGraphValue {
    fn try_cast_to(&self, target: &TestGraphDataType) -> Result<Self, GraphError> {
        use TestGraphDataType as DT;

        match (self, target) {
            (Self::Float(_), DT::Float) => Ok(self.clone()),
        }
    }
}

impl TryFrom<TestGraphValue> for f32 {
    type Error = GraphError;

    fn try_from(value: TestGraphValue) -> Result<Self, Self::Error> {
        match value {
            TestGraphValue::Float(value) => Ok(value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TestGraphDataType {
    Float,
}

impl DataType<TestGraphDefinition> for TestGraphDataType {
    fn default_value(&self) -> TestGraphValue {
        match self {
            Self::Float => TestGraphValue::Float(f32::default()),
        }
    }
}

impl VisualDataType for TestGraphDataType {
    fn color(&self) -> Hsla {
        match self {
            Self::Float => rgb(0xFF3C59).into(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TestGraphControl {
    Float,
}

impl Control<TestGraphDefinition> for TestGraphControl {}

impl VisualControl<TestGraphDefinition> for TestGraphControl {
    fn view<View: EventEmitter<ControlEvent<TestGraphDefinition>>>(
        &self,
        _id: impl Into<ElementId>,
        _initial_value: TestGraphValue,
        cx: &mut ViewContext<View>,
    ) -> AnyView {
        match self {
            Self::Float => cx.new_view(|_cx| EmptyView).into(),
        }
    }
}

#[test]
fn test() {
    let mut graph = TestGraph::new();
    let node_id = graph.add_node(TestGraphNodeKind::Add, TestGraphNodeData::default());
    match &mut graph.input_mut(graph.node(node_id).input("a").id).kind {
        InputParameterKind::EdgeOrConstant { value, .. } => {
            *value = TestGraphValue::Float(42.0);
        }
    };

    match &mut graph.input_mut(graph.node(node_id).input("b").id).kind {
        InputParameterKind::EdgeOrConstant { value, .. } => {
            *value = TestGraphValue::Float(2.0);
        }
    };

    let mut context = TestGraphProcessingContext { output_float: 0.0 };
    graph.process(&mut context).unwrap();

    assert_eq!(context.output_float, 44.0);
}
