use flow::{Control, Graph, GraphDefinition, GraphError, InputParameterKind, Value};
use flow_gpui::{ControlEvent, VisualControl, VisualNodeData};
use gpui::{AnyView, ElementId, EmptyView, EventEmitter, ViewContext, VisualContext};

//
// GraphDefinition
//

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

//
// Processing Context
//

pub struct TestGraphProcessingContext {
    pub output_float: f32,
}

//
// NodeData
//

#[derive(Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct TestGraphNodeData {
    position: geo::Point,
}

impl VisualNodeData for TestGraphNodeData {
    fn position(&self) -> &geo::Point {
        &self.position
    }

    fn set_position(&mut self, position: geo::Point) {
        self.position = position
    }
}

//
// Value
//

#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, flow_gpui_macros::Value,
)]
#[value(
    data_type = "TestGraphDataType",
    graph_definition = "TestGraphDefinition"
)]
pub enum TestGraphValue {
    #[meta(default_value = "0.0")]
    Float(f32),
}

impl Value<TestGraphDefinition> for TestGraphValue {
    fn try_cast_to(
        &self,
        target: &<TestGraphDefinition as GraphDefinition>::DataType,
    ) -> Result<Self, GraphError>
    where
        Self: Sized,
    {
        type T = <TestGraphDefinition as GraphDefinition>::DataType;
        match (&self, target) {
            (Self::Float(_), T::Float) => Ok(self.clone()),
        }
    }
}

//
// Control
//

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

//
// Category
//

#[derive(Clone, Copy, PartialEq, flow_gpui_macros::NodeCategory)]
pub enum Category {
    Math,
    Output,
}

//
// NodeKind
//

#[derive(Clone, serde::Serialize, serde::Deserialize, flow_gpui_macros::NodeKind)]
#[node_kind(
    graph_definition = "TestGraphDefinition",
    processing_context = "TestGraphProcessingContext"
)]
pub enum TestGraphNodeKind {
    #[input(label = "a", data_type = "Float", control = "Float")]
    #[input(label = "b", data_type = "Float", control = "Float")]
    #[computed_output(label = "sum", data_type = "Float")]
    #[meta(name = "Add", category = "Math", processor = "add_processor")]
    Add,
    #[input(label = "value", data_type = "Float", control = "Float")]
    #[meta(name = "Output", category = "Output", processor = "output_processor")]
    Output,
}

fn add_processor(
    input: AddProcessorInput,
    _context: &mut TestGraphProcessingContext,
) -> Result<AddProcessorOutput, GraphError> {
    let AddProcessorInput { a, b } = input;

    let a: f32 = a.try_into()?;
    let b: f32 = b.try_into()?;

    Ok(AddProcessorOutput {
        sum: TestGraphValue::Float(a + b),
    })
}

fn output_processor(
    input: OutputProcessorInput,
    context: &mut TestGraphProcessingContext,
) -> Result<OutputProcessorOutput, GraphError> {
    let OutputProcessorInput { value } = input;
    context.output_float = value.try_into()?;
    Ok(OutputProcessorOutput {})
}

#[test]
fn test() {
    let mut graph = TestGraph::new();
    let add_node_id = graph.add_node(TestGraphNodeKind::Add, TestGraphNodeData::default());
    let output_node_id = graph.add_node(TestGraphNodeKind::Output, TestGraphNodeData::default());

    match &mut graph.input_mut(graph.node(add_node_id).input("a").id).kind {
        InputParameterKind::EdgeOrConstant { value, .. } => {
            *value = TestGraphValue::Float(42.0);
        }
    };

    match &mut graph.input_mut(graph.node(add_node_id).input("b").id).kind {
        InputParameterKind::EdgeOrConstant { value, .. } => {
            *value = TestGraphValue::Float(2.0);
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
