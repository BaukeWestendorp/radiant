use flow::{Graph, Node, OutputValue};
use gpui::{rgb, Hsla};

pub type EffectGraph = Graph<EffectDataType, EffectValue, EffectNodeKind>;

#[derive(Clone)]
pub enum EffectDataType {
    Int,
    Float,
}

impl flow::DataType for EffectDataType {
    type Value = EffectValue;

    fn color(&self) -> Hsla {
        match self {
            Self::Int => rgb(0xc905ff).into(),
            Self::Float => rgb(0xccff00).into(),
        }
    }
}

impl From<EffectValue> for EffectDataType {
    fn from(value: EffectValue) -> Self {
        match value {
            EffectValue::Int(_) => EffectDataType::Int,
            EffectValue::Float(_) => EffectDataType::Float,
        }
    }
}

#[derive(Clone)]
pub enum EffectValue {
    Int(i32),
    Float(f32),
}

impl flow::Value for EffectValue {
    type DataType = EffectDataType;

    fn try_cast_to(self, target_type: &Self::DataType) -> Result<Self, flow::FlowError>
    where
        Self: Sized,
    {
        match (&self, target_type) {
            (Self::Int(_), Self::DataType::Int) => Ok(self),
            (Self::Int(v), Self::DataType::Float) => Ok(Self::Float(*v as f32)),

            (Self::Float(v), Self::DataType::Int) => Ok(Self::Int(*v as i32)),
            (Self::Float(_), Self::DataType::Float) => Ok(self),
        }
    }
}

#[derive(Clone)]
pub enum EffectNodeKind {
    IntValue,
    FloatValue,
    IntAdd,
    Output,
}

impl flow::NodeKind for EffectNodeKind {
    type DataType = EffectDataType;
    type Value = EffectValue;
    type ProcessingContext = EffectProcessingContext;

    fn build(&self, graph: &mut Graph<Self::DataType, Self::Value, Self>, node_id: flow::NodeId)
    where
        Self: Sized,
    {
        match self {
            Self::IntValue => {
                graph.add_output(
                    node_id,
                    "value".to_string(),
                    EffectDataType::Int,
                    OutputValue::Constant(EffectValue::Int(0)),
                );
            }
            Self::FloatValue => {
                graph.add_output(
                    node_id,
                    "value".to_string(),
                    EffectDataType::Float,
                    OutputValue::Constant(EffectValue::Float(0.0)),
                );
            }
            Self::IntAdd => {
                graph.add_input(
                    node_id,
                    "a".to_string(),
                    EffectDataType::Int,
                    EffectValue::Int(0),
                );
                graph.add_input(
                    node_id,
                    "b".to_string(),
                    EffectDataType::Int,
                    EffectValue::Int(0),
                );
                graph.add_output(
                    node_id,
                    "sum".to_string(),
                    EffectDataType::Int,
                    OutputValue::Computed,
                );
            }
            Self::Output => {
                graph.add_input(
                    node_id,
                    "value".to_string(),
                    EffectDataType::Int,
                    EffectValue::Int(0),
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
        let mut value_for_input = |node: &Node<Self::DataType, Self::Value, Self>,
                                   input_name: &str|
         -> Result<EffectValue, flow::FlowError> {
            let connection_id = graph.connection(node.input(input_name)?).unwrap();
            Ok(graph
                .get_output_value(connection_id, context, cache)?
                .clone())
        };

        let node = graph.node(node_id);
        match &node.kind {
            Self::IntValue => {}
            Self::FloatValue => {}
            Self::Output => {
                let EffectValue::Int(value) = value_for_input(node, "value")? else {
                    return Err(flow::FlowError::CastFailed);
                };
                context.output_value = value;
            }
            Self::IntAdd => {
                let EffectValue::Int(a) = value_for_input(node, "a")? else {
                    return Err(flow::FlowError::CastFailed);
                };
                let EffectValue::Int(b) = value_for_input(node, "b")? else {
                    return Err(flow::FlowError::CastFailed);
                };

                let sum = a + b;

                cache.set_output_value(node.output("sum")?, EffectValue::Int(sum));
            }
        }

        Ok(())
    }

    fn label(&self) -> &str {
        match self {
            Self::IntValue => "Int Value",
            Self::FloatValue => "Float Value",
            Self::Output => "Output",
            Self::IntAdd => "Int Add",
        }
    }
}

pub struct EffectProcessingContext {
    pub output_value: i32,
}
