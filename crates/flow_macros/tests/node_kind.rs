#![cfg(all(feature = "gpui", not(feature = "serde")))]

use flow::gpui::ControlEvent;
use flow::{Graph, GraphError, InputParameterKind};
use gpui::{AnyView, ElementId, EmptyView, EventEmitter, ViewContext, VisualContext};

//
// GraphDefinition
//

#[derive(Clone)]
pub struct GraphDefinition;

impl flow::GraphDefinition for GraphDefinition {
    type NodeKind = NodeKind;
    type NodeData = NodeData;
    type Value = Value;
    type DataType = DataType;

    type ProcessingContext = ProcessingContext;

    type NodeCategory = Category;
    type Control = Control;
}

pub type TestGraph = Graph<GraphDefinition>;

//
// Processing Context
//

pub struct ProcessingContext {
    pub output_float: f32,
}

//
// NodeData
//

#[derive(Clone, PartialEq, Default)]
pub struct NodeData {
    position: geo::Point,
}

impl flow::NodeData for NodeData {
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

#[derive(Debug, Clone, PartialEq, flow::Value)]
#[value(data_type = "DataType", graph_definition = "GraphDefinition")]
pub enum Value {
    #[meta(default_value = "0.0", color = 0xff0000)]
    Float(f32),
}

impl flow::Value<GraphDefinition> for Value {
    fn try_cast_to(&self, target: &DataType) -> Result<Self, GraphError>
    where
        Self: Sized,
    {
        match (&self, target) {
            (Self::Float(_), DataType::Float) => Ok(self.clone()),
        }
    }
}

//
// Control
//

#[derive(Debug, Clone)]
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
            Self::Float => cx.new_view(|_cx| EmptyView).into(),
        }
    }
}

//
// Category
//

#[derive(Clone, Copy, PartialEq, flow::gpui::NodeCategory)]
pub enum Category {
    Math,
    Output,
}

//
// NodeKind
//

#[derive(Clone, flow::NodeKind)]
#[node_kind(graph_definition = "GraphDefinition")]
pub enum NodeKind {
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
    _context: &mut ProcessingContext,
) -> Result<AddProcessorOutput, GraphError> {
    let AddProcessorInput { a, b } = input;

    let a: f32 = a.try_into()?;
    let b: f32 = b.try_into()?;

    Ok(AddProcessorOutput {
        sum: Value::Float(a + b),
    })
}

fn output_processor(
    input: OutputProcessorInput,
    context: &mut ProcessingContext,
) -> Result<OutputProcessorOutput, GraphError> {
    let OutputProcessorInput { value } = input;
    context.output_float = value.try_into()?;
    Ok(OutputProcessorOutput {})
}

#[test]
fn test() {
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

    let mut context = ProcessingContext { output_float: 0.0 };
    graph.process(&mut context).unwrap();

    assert_eq!(context.output_float, 44.0);
}
