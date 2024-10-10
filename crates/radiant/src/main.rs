use effect_graph::{EffectGraph, EffectNodeKind, EffectProcessingContext, EffectValue};
use flow::{GraphProcessingCache, OutputValue};
use gpui::*;

mod assets;
mod effect_graph;

actions!(app, [ProcessGraph, Quit]);

fn main() {
    App::new().with_assets(assets::Assets).run(|cx| {
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(800.0), px(600.0)),
                cx,
            ))),
            ..Default::default()
        };

        let graph_model = cx.new_model(|_cx| create_graph());

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

        cx.on_action::<ProcessGraph>({
            let graph_model = graph_model.clone();
            move |_action, cx| {
                graph_model.update(cx, |graph, _cx| {
                    let mut context = EffectProcessingContext { output_value: 0 };
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
            cx.new_view(|cx| flow::gpui::editor::EditorView::new(cx, Some(graph_model)))
        })
        .unwrap();

        cx.activate(true);
    })
}

fn create_graph() -> EffectGraph {
    let mut graph = EffectGraph::new();

    let a_node_id = graph.add_node(EffectNodeKind::IntValue, point(px(50.0), px(50.0)));
    let b_node_id = graph.add_node(EffectNodeKind::FloatValue, point(px(50.0), px(150.0)));
    let add_node_id = graph.add_node(EffectNodeKind::IntAdd, point(px(300.0), px(100.0)));
    let output_node_id = graph.add_node(EffectNodeKind::Output, point(px(550.0), px(100.0)));

    graph
        .output_mut(graph.node(a_node_id).output("value").unwrap())
        .value = OutputValue::Constant(EffectValue::Int(42));

    graph
        .output_mut(graph.node(b_node_id).output("value").unwrap())
        .value = OutputValue::Constant(EffectValue::Float(69.8));

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
