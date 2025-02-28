use std::collections::HashMap;

use flow::{
    graph::{Graph, GraphState},
    template::{NodeTemplate, TemplateId},
};

pub fn get_templates() -> HashMap<TemplateId, NodeTemplate> {
    let templates_json = include_str!("./graph_templates.json");
    serde_json::from_str(templates_json).expect("should parse node templates")
}

pub fn get_graph_data() -> GraphState {
    let data_json = include_str!("./graph_data.json");
    serde_json::from_str(data_json).expect("should parse graph data")
}

pub fn get_graph() -> Graph {
    let templates = get_templates();
    let data = get_graph_data();
    let mut graph = Graph::new(templates, data);
    graph.register_processor("new_number", Box::new(processor::new_number));
    graph.register_processor("output", Box::new(processor::output));
    graph
}

mod processor {
    use flow::graph::NodeContext;

    pub fn new_number(_cx: &mut NodeContext) {
        todo!("Implement processing `new_number` node");
    }

    pub fn output(cx: &mut NodeContext) {
        let value = cx.input_value("value");
        cx.output_value = value;
    }
}
