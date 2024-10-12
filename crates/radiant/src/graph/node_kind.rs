use super::{
    error::GraphError,
    node::{Node, OutputValue},
    DataType, Graph, NodeId, ProcessingCache, ProcessingContext, Value,
};

#[derive(Debug, Clone)]
pub enum NodeKind {
    NewInt,
    NewFloat,
    IntAdd,
    Output,
}

impl NodeKind {
    pub fn build(&self, graph: &mut Graph, node_id: NodeId)
    where
        Self: Sized,
    {
        match self {
            Self::NewInt => {
                graph.add_output(
                    node_id,
                    "value".to_string(),
                    DataType::Int,
                    OutputValue::Constant(Value::Int(0)),
                );
            }
            Self::NewFloat => {
                graph.add_output(
                    node_id,
                    "value".to_string(),
                    DataType::Float,
                    OutputValue::Constant(Value::Float(0.0)),
                );
            }
            Self::IntAdd => {
                graph.add_input(node_id, "a".to_string(), DataType::Int, Value::Int(0));
                graph.add_input(node_id, "b".to_string(), DataType::Int, Value::Int(0));
                graph.add_output(
                    node_id,
                    "sum".to_string(),
                    DataType::Int,
                    OutputValue::Computed,
                );
            }
            Self::Output => {
                graph.add_input(node_id, "value".to_string(), DataType::Int, Value::Int(0));
            }
        }
    }

    pub fn process(
        &self,
        node_id: NodeId,
        context: &mut ProcessingContext,
        graph: &Graph,
        cache: &mut ProcessingCache,
    ) -> Result<(), GraphError>
    where
        Self: Sized,
    {
        let mut value_for_input = |node: &Node, input_name: &str| -> Result<Value, GraphError> {
            let connection_id = graph.connection(node.input(input_name)?).unwrap();
            let value = graph
                .get_output_value(connection_id, context, cache)?
                .clone();
            Ok(value)
        };

        let node = graph.node(node_id);
        match &node.kind {
            Self::NewInt => {}
            Self::NewFloat => {}
            Self::Output => {
                let Value::Int(value) = value_for_input(node, "value")? else {
                    return Err(GraphError::CastFailed);
                };
                context.output = value;
            }
            Self::IntAdd => {
                let Value::Int(a) = value_for_input(node, "a")? else {
                    return Err(GraphError::CastFailed);
                };
                let Value::Int(b) = value_for_input(node, "b")? else {
                    return Err(GraphError::CastFailed);
                };

                let sum = a + b;

                cache.set_output_value(node.output("sum")?, Value::Int(sum));
            }
        }

        Ok(())
    }

    pub fn label(&self) -> &str {
        match self {
            Self::NewInt => "Int Value",
            Self::NewFloat => "Float Value",
            Self::Output => "Output",
            Self::IntAdd => "Int Add",
        }
    }
}
