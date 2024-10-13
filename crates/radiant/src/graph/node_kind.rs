use std::collections::HashMap;

use super::{
    error::GraphError,
    node::{Node, OutputValue},
    view::control::Control,
    DataType, Graph, NodeId, OutputId, ProcessingContext, Value,
};

#[derive(Debug, Clone)]
pub enum NodeKind {
    NewInt,
    NewFloat,
    NewString,
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
                    OutputValue::Constant {
                        value: Value::Int(0),
                        control: Control::IntField,
                    },
                );
            }
            Self::NewFloat => {
                graph.add_output(
                    node_id,
                    "value".to_string(),
                    DataType::Float,
                    OutputValue::Constant {
                        value: Value::Float(0.0),
                        control: Control::Range {
                            range: 0.0..=100.0,
                            step: 10.0,
                            strict: true,
                        },
                    },
                );
            }
            Self::NewString => {
                graph.add_output(
                    node_id,
                    "value".to_string(),
                    DataType::String,
                    OutputValue::Constant {
                        value: Value::String("".to_string().into()),
                        control: Control::TextField,
                    },
                );
            }
            Self::IntAdd => {
                graph.add_input(node_id, "a".to_string(), DataType::Int);
                graph.add_input(node_id, "b".to_string(), DataType::Int);
                graph.add_output(
                    node_id,
                    "sum".to_string(),
                    DataType::Int,
                    OutputValue::Computed,
                );
            }
            Self::Output => {
                graph.add_input(node_id, "value".to_string(), DataType::Int);
            }
        }
    }

    pub fn process(
        &self,
        node_id: NodeId,
        context: &mut ProcessingContext,
        graph: &Graph,
    ) -> Result<ProcessingResult, GraphError>
    where
        Self: Sized,
    {
        let mut value_for_input = |node: &Node, input_name: &str| -> Result<Value, GraphError> {
            let connection_id = graph.connection(node.input(input_name)?).unwrap();
            let value = graph.get_output_value(connection_id, context)?.clone();
            Ok(value)
        };

        let mut processing_result = HashMap::new();

        let node = graph.node(node_id);
        match &node.kind {
            Self::NewInt => {}
            Self::NewFloat => {}
            Self::NewString => {}
            Self::Output => {
                let Value::Int(value) = value_for_input(node, "value")? else {
                    return Err(GraphError::CastFailed);
                };
                context.output = value;
            }
            Self::IntAdd => {
                let Value::Int(a) = value_for_input(node, "a")?.try_cast_to(DataType::Int)? else {
                    panic!("Invalid Cast");
                };
                let Value::Int(b) = value_for_input(node, "b")?.try_cast_to(DataType::Int)? else {
                    panic!("Invalid Cast");
                };

                let sum = a + b;

                processing_result.insert(node.output("sum")?, Value::Int(sum));
            }
        }

        Ok(processing_result)
    }

    pub fn label(&self) -> &str {
        match self {
            Self::NewInt => "Int Value",
            Self::NewFloat => "Float Value",
            Self::NewString => "String Value",
            Self::Output => "Output",
            Self::IntAdd => "Int Add",
        }
    }
}

pub type ProcessingResult = HashMap<OutputId, Value>;
