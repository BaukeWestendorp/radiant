use flow::{Graph, GraphNodeKind, GraphProcessingCache, OutputValue};
use flow_gpui::{
    geo,
    graph::{VisualDataType, VisualGraphState, VisualNode},
};
use gpui::*;

pub type ExampleGraph = Graph<ExampleDataType, ExampleValue, ExampleNodeKind>;

pub enum ExampleDataType {
    Int,
}

impl VisualDataType for ExampleDataType {
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

#[derive(Clone)]
pub enum ExampleNodeKind {
    IntValue,
    IntAdd,
    Output,
}

impl GraphNodeKind for ExampleNodeKind {
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
}

impl VisualNode for ExampleNodeKind {
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

actions!(app, [ProcessGraph, Quit]);

fn main() {
    App::new().with_assets(flow_gpui::assets::Assets).run(|cx| {
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(800.0), px(600.0)),
                cx,
            ))),
            ..Default::default()
        };

        let mut visual_graph_state = VisualGraphState::default();

        let graph_model = cx.new_model(|_cx| {
            let mut graph = ExampleGraph::new();

            let a_node_id = graph.add_node(ExampleNodeKind::IntValue);
            let b_node_id = graph.add_node(ExampleNodeKind::IntValue);
            let add_node_id = graph.add_node(ExampleNodeKind::IntAdd);
            let output_node_id = graph.add_node(ExampleNodeKind::Output);

            visual_graph_state.set_node_position(a_node_id, geo::Point::new(50.0, 50.0));
            visual_graph_state.set_node_position(b_node_id, geo::Point::new(50.0, 150.0));
            visual_graph_state.set_node_position(add_node_id, geo::Point::new(300.0, 100.0));
            visual_graph_state.set_node_position(output_node_id, geo::Point::new(550.0, 100.0));

            graph
                .output_mut(graph.node(a_node_id).output("value").unwrap())
                .value = OutputValue::Constant(ExampleValue::Int(42));

            graph
                .output_mut(graph.node(b_node_id).output("value").unwrap())
                .value = OutputValue::Constant(ExampleValue::Int(69));

            graph.add_connection(
                graph.input(graph.node(add_node_id).input("a").unwrap()).id,
                graph
                    .output(graph.node(a_node_id).output("value").unwrap())
                    .id,
            );

            graph.add_connection(
                graph.input(graph.node(add_node_id).input("b").unwrap()).id,
                graph
                    .output(graph.node(b_node_id).output("value").unwrap())
                    .id,
            );

            graph.add_connection(
                graph
                    .input(graph.node(output_node_id).input("value").unwrap())
                    .id,
                graph
                    .output(graph.node(add_node_id).output("sum").unwrap())
                    .id,
            );
            graph
        });

        cx.bind_keys([
            KeyBinding::new("p", ProcessGraph, None),
            KeyBinding::new("cmd-q", Quit, None),
        ]);

        cx.set_menus(vec![Menu {
            name: "simple_graph".to_string().into(),
            items: vec![
                MenuItem::action("Quit", Quit),
                MenuItem::action("Process Graph", ProcessGraph),
            ],
        }]);

        cx.on_action::<ProcessGraph>({
            let graph_model = graph_model.clone();
            move |_action, cx| {
                graph_model.update(cx, |graph, _cx| {
                    let mut context = ExampleProcessingContext { output_value: 0 };
                    let mut cache = GraphProcessingCache::default();

                    graph.process(&mut context, &mut cache).unwrap();

                    println!("Output value: {}", context.output_value);
                })
            }
        });

        cx.on_action::<Quit>(|_action, cx| {
            cx.quit();
        });

        cx.open_window(options, |cx| {
            cx.new_view(|cx| flow_gpui::Editor::new(cx, Some(graph_model), visual_graph_state))
        })
        .unwrap();

        cx.activate(true);
    })
}
