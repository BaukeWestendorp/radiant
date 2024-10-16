use crate::error::GraphError;
use crate::graph::{Graph, NodeId, ProcessingResult};
use crate::node::{InputValue, Node, OutputValue};
use crate::view::control::Control;
use gpui::{rgb, Hsla, SharedString};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Float(f32),
    String(SharedString),
}

impl Value {
    pub fn try_cast_to(&self, target: &DataType) -> Result<Self, GraphError> {
        match (&self, target) {
            (Self::Int(_), DataType::Int) => Ok(self.clone()),
            (Self::Int(v), DataType::Float) => Ok(Self::Float(*v as f32)),

            (Self::Float(v), DataType::Int) => Ok(Self::Int(*v as i32)),
            (Self::Float(_), DataType::Float) => Ok(self.clone()),

            _ => Err(GraphError::CastFailed),
        }
    }
}

impl TryInto<i32> for Value {
    type Error = GraphError;

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Self::Int(v) => Ok(v),
            _ => Err(GraphError::CastFailed),
        }
    }
}

impl TryInto<f32> for Value {
    type Error = GraphError;

    fn try_into(self) -> Result<f32, Self::Error> {
        match self {
            Self::Float(v) => Ok(v),
            _ => Err(GraphError::CastFailed),
        }
    }
}

impl TryInto<SharedString> for Value {
    type Error = GraphError;

    fn try_into(self) -> Result<SharedString, Self::Error> {
        match self {
            Self::String(v) => Ok(v),
            _ => Err(GraphError::CastFailed),
        }
    }
}

impl TryInto<String> for Value {
    type Error = GraphError;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Self::String(v) => Ok(v.to_string()),
            _ => Err(GraphError::CastFailed),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DataType {
    Int,
    Float,
    String,
}

impl DataType {
    pub fn color(&self) -> Hsla {
        match self {
            DataType::Int => rgb(0xD137FF).into(),
            DataType::Float => rgb(0x37D1FF).into(),
            DataType::String => rgb(0xFFD137).into(),
        }
    }

    pub fn default_value(&self) -> Value {
        match self {
            DataType::Int => Value::Int(Default::default()),
            DataType::Float => Value::Float(Default::default()),
            DataType::String => Value::String(Default::default()),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProcessingContext {
    pub output: i32,
}

#[derive(Debug, Clone, strum::EnumIter)]
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
                        control: Control::Slider {
                            range: 0.0..=100.0,
                            step: Some(10.0),
                            strict: false,
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
                graph.add_input(
                    node_id,
                    "a".to_string(),
                    DataType::Int,
                    InputValue::Constant {
                        value: Value::Int(0),
                        control: Control::IntField,
                    },
                );
                graph.add_input(
                    node_id,
                    "b".to_string(),
                    DataType::Int,
                    InputValue::Constant {
                        value: Value::Int(0),
                        control: Control::IntField,
                    },
                );
                graph.add_output(
                    node_id,
                    "sum".to_string(),
                    DataType::Int,
                    OutputValue::Computed,
                );
            }
            Self::Output => {
                graph.add_input(
                    node_id,
                    "value".to_string(),
                    DataType::Int,
                    InputValue::Constant {
                        value: Value::Int(0),
                        control: Control::IntField,
                    },
                );
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
            let input_id = node.input(input_name)?;
            let connection_id = graph.connection_source(input_id);
            let value = match connection_id {
                None => {
                    let InputValue::Constant { value, .. } = graph.input(input_id).value.clone();
                    value
                }
                Some(id) => graph.get_output_value(&id, context)?.clone(),
            };
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
                let Value::Int(a) = value_for_input(node, "a")?.try_cast_to(&DataType::Int)? else {
                    return Err(GraphError::CastFailed);
                };
                let Value::Int(b) = value_for_input(node, "b")?.try_cast_to(&DataType::Int)? else {
                    return Err(GraphError::CastFailed);
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
