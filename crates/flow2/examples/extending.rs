use std::any::{Any, TypeId};

use flow2::{
    def::{Node, ValueType},
    graph::{NodeGraph, NodeId},
};

#[derive(Debug)]
pub struct AddNode;

impl Node for AddNode {
    fn id(&self) -> &'static str {
        "add"
    }

    fn process(&self, inputs: Vec<Box<dyn ValueType>>) -> Vec<Box<dyn ValueType>> {
        if inputs.len() >= 2 {
            let a = inputs[0].as_any().downcast_ref::<f64>();
            let b = inputs[1].as_any().downcast_ref::<f64>();
            if let (Some(a), Some(b)) = (a, b) {
                return vec![Box::new(a + b)];
            }
        }
        vec![Box::new(0.0f64)]
    }
}

fn main() {
    let mut graph = NodeGraph::new();
    graph.add_node(AddNode);

    let inputs: Vec<Box<dyn ValueType>> = vec![Box::new(3.0f64), Box::new(4.5f64)];
    let result = graph.nodes[&NodeId(0)].process(inputs);

    dbg!(&result[0].type_id() == &TypeId::of::<f>());
    if let Some(result_val) = result[0].as_any().downcast_ref::<f64>() {
        println!("Result: {}", result_val); // Should print 7.5
    }
}
