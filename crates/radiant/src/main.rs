use gpui::*;
use graph::node::OutputValue;
use graph::node_kind::NodeKind;
use graph::view::graph::GraphView;
use graph::view::node::ControlEvent;
use graph::{Graph, ProcessingContext, Value};

mod assets;
pub mod graph;

actions!(app, [ProcessGraph, Quit]);

fn main() {
    env_logger::init();

    App::new().with_assets(assets::Assets).run(|cx| {
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(800.0), px(600.0)),
                cx,
            ))),
            ..Default::default()
        };

        cx.bind_keys([
            KeyBinding::new("p", ProcessGraph, None),
            KeyBinding::new("cmd-q", Quit, None),
        ]);

        cx.set_menus(vec![Menu {
            name: "Radiant".to_string().into(),
            items: vec![
                MenuItem::action("Quit", Quit),
                MenuItem::action("Process Graph", ProcessGraph),
            ],
        }]);

        let mut graph = cx.new_model(|_cx| create_graph());

        register_actions(&mut graph, cx);

        cx.open_window(options, |cx| GraphView::build(graph, cx))
            .unwrap();

        cx.activate(true);
    })
}

fn register_actions(graph: &mut Model<Graph>, cx: &mut AppContext) {
    cx.on_action::<ProcessGraph>({
        let graph = graph.clone();
        move |_, cx| {
            graph.update(cx, |graph, _cx| {
                let mut context = ProcessingContext::default();

                log::info!("Processing graph...");
                graph.process(&mut context).unwrap();
                log::info!("Output value: {}", context.output);
            });
        }
    });

    cx.on_action::<Quit>(|_action, cx| {
        cx.quit();
    });
}

impl EventEmitter<ControlEvent> for GraphView {}

fn create_graph() -> Graph {
    let mut graph = Graph::new();

    let a_node_id = graph.add_node(NodeKind::NewInt, point(px(50.0), px(50.0)));
    let b_node_id = graph.add_node(NodeKind::NewFloat, point(px(50.0), px(150.0)));
    let add_node_id = graph.add_node(NodeKind::IntAdd, point(px(300.0), px(100.0)));
    let output_node_id = graph.add_node(NodeKind::Output, point(px(550.0), px(100.0)));

    graph
        .output_mut(graph.node(a_node_id).output("value").unwrap())
        .value = OutputValue::Constant(Value::Int(42));

    graph
        .output_mut(graph.node(b_node_id).output("value").unwrap())
        .value = OutputValue::Constant(Value::Float(69.8));

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
}
