use crate::graph::view::editor::EditorView;
use gpui::*;
use graph::node::OutputValue;
use graph::NodeKind;
use graph::{Graph, ProcessingContext, Value};
use ui::theme::Theme;

mod assets;
pub mod graph;

actions!(app, [ProcessGraph, Quit]);

fn main() {
    env_logger::init();

    App::new().with_assets(assets::Assets).run(|cx| {
        cx.set_global(Theme::default());
        ui::init(cx);
        graph::view::editor::init(cx);

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

        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(800.0), px(600.0)),
                cx,
            ))),
            ..Default::default()
        };

        cx.open_window(options, |cx| EditorView::build(graph, cx))
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

                graph.process(&mut context).unwrap();
                log::info!("Output value: {}", context.output);
            });
        }
    });

    cx.on_action::<Quit>(|_action, cx| {
        cx.quit();
    });
}

fn create_graph() -> Graph {
    let mut graph = Graph::new();

    let a_node_id = graph.add_node(NodeKind::NewInt, point(px(50.0), px(50.0)));
    let b_node_id = graph.add_node(NodeKind::NewFloat, point(px(50.0), px(150.0)));
    let new_string_node_id = graph.add_node(NodeKind::NewString, point(px(50.0), px(250.0)));
    let add_node_id = graph.add_node(NodeKind::IntAdd, point(px(300.0), px(100.0)));
    let output_node_id = graph.add_node(NodeKind::Output, point(px(550.0), px(100.0)));

    if let OutputValue::Constant { value, .. } = &mut graph
        .output_mut(graph.node(a_node_id).output("value").unwrap())
        .value
    {
        *value = Value::Int(42);
    }

    if let OutputValue::Constant { value, .. } = &mut graph
        .output_mut(graph.node(b_node_id).output("value").unwrap())
        .value
    {
        *value = Value::Float(0.33);
    }

    if let OutputValue::Constant { value, .. } = &mut graph
        .output_mut(graph.node(new_string_node_id).output("value").unwrap())
        .value
    {
        *value = Value::String("Hello, world!".into());
    }

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
