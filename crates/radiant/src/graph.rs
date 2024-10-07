use flow::{Graph, OutputValue};
use gpui::{rgb, Hsla};

pub type ExampleGraph = Graph<ExampleDataType, ExampleValue, ExampleNodeKind>;

#[derive(Clone)]
pub enum ExampleDataType {
    Int,
}

impl flow::DataType for ExampleDataType {
    type Value = ExampleValue;

    fn color(&self) -> Hsla {
        match self {
            ExampleDataType::Int => rgb(0xc905ff).into(),
        }
    }
}

#[derive(Clone)]
pub enum ExampleValue {
    Int(i32),
}

impl flow::Value for ExampleValue {
    type DataType = ExampleDataType;

    fn try_cast_to(
        self,
        _target_type: &Self::DataType,
    ) -> std::prelude::v1::Result<Self, flow::FlowError>
    where
        Self: Sized,
    {
        todo!()
    }
}

#[derive(Clone)]
pub enum ExampleNodeKind {
    IntValue,
    IntAdd,
    Output,
}

impl flow::NodeKind for ExampleNodeKind {
    type DataType = ExampleDataType;
    type Value = ExampleValue;

    type ProcessingContext = ExampleProcessingContext;

    fn build(&self, graph: &mut Graph<Self::DataType, Self::Value, Self>, node_id: flow::NodeId)
    where
        Self: Sized,
    {
        match self {
            ExampleNodeKind::IntValue => {
                graph.add_output(
                    node_id,
                    "value".to_string(),
                    ExampleDataType::Int,
                    OutputValue::Constant(ExampleValue::Int(0)),
                );
            }
            ExampleNodeKind::IntAdd => {
                graph.add_input(
                    node_id,
                    "a".to_string(),
                    ExampleDataType::Int,
                    ExampleValue::Int(0),
                );
                graph.add_input(
                    node_id,
                    "b".to_string(),
                    ExampleDataType::Int,
                    ExampleValue::Int(0),
                );
                graph.add_output(
                    node_id,
                    "sum".to_string(),
                    ExampleDataType::Int,
                    OutputValue::Computed,
                );
            }
            ExampleNodeKind::Output => {
                graph.add_input(
                    node_id,
                    "value".to_string(),
                    ExampleDataType::Int,
                    ExampleValue::Int(0),
                );
            }
        }
    }

    fn process(
        &self,
        node_id: flow::NodeId,
        context: &mut Self::ProcessingContext,
        graph: &Graph<Self::DataType, Self::Value, Self>,
        cache: &mut flow::GraphProcessingCache<Self::Value>,
    ) -> Result<(), flow::FlowError>
    where
        Self: Sized,
    {
        let node = graph.node(node_id);
        match &node.kind {
            ExampleNodeKind::IntValue => {}
            ExampleNodeKind::Output => {
                let value_id = graph.connection(node.input("value")?).unwrap();
                let value = graph.get_output_value(value_id, context, cache)?.clone();
                let ExampleValue::Int(value) = value;
                context.output_value = value;
            }
            ExampleNodeKind::IntAdd => {
                let a_id = graph.connection(node.input("a")?).unwrap();
                let b_id = graph.connection(node.input("b")?).unwrap();
                let a = graph.get_output_value(a_id, context, cache)?.clone();
                let b = graph.get_output_value(b_id, context, cache)?.clone();
                let ExampleValue::Int(a) = a;
                let ExampleValue::Int(b) = b;
                let sum = a + b;
                cache.set_output_value(node.output("sum")?, ExampleValue::Int(sum));
            }
        }

        Ok(())
    }

    fn label(&self) -> &str {
        match self {
            ExampleNodeKind::IntValue => "Int Value",
            ExampleNodeKind::Output => "Output",
            ExampleNodeKind::IntAdd => "Int Add",
        }
    }
}

pub struct ExampleProcessingContext {
    pub output_value: i32,
}
