use super::{GraphEvent, graph::GraphView, node::NodeMeasurements};
use crate::{GraphDef, Node};
use gpui::*;
use prelude::FluentBuilder;
use ui::{Pannable, PannableEvent, TextField, theme::ActiveTheme, z_stack};

const KEY_CONTEXT: &str = "GraphEditor";

actions!(graph_editor, [OpenNewNodeMenu, CloseNewNodeMenu]);

pub fn init(app: &mut App) {
    app.bind_keys([
        KeyBinding::new("space", OpenNewNodeMenu, Some(KEY_CONTEXT)),
        KeyBinding::new("escape", CloseNewNodeMenu, Some(KEY_CONTEXT)),
    ]);
}

pub struct GraphEditorView<D: GraphDef> {
    graph_view: Entity<Pannable>,
    new_node_menu_view: Option<Entity<NewNodeMenuView<D>>>,

    graph: Entity<crate::Graph<D>>,
    visual_graph_offset: Point<Pixels>,

    focus_handle: FocusHandle,
}

impl<D: GraphDef + 'static> GraphEditorView<D> {
    pub fn build(
        graph: Entity<crate::Graph<D>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let graph_view = GraphView::build(graph.clone(), window, cx);

            window
                .subscribe(&graph, cx, {
                    let graph_view = graph_view.clone();
                    move |graph, event: &GraphEvent, window, cx| {
                        graph_view.update(cx, |graph_view, cx| match event {
                            GraphEvent::NodeAdded(node_id) => {
                                graph_view.add_node(*node_id, window, cx)
                            }
                            GraphEvent::NodeRemoved(node_id) => graph_view.remove_node(node_id, cx),
                            GraphEvent::EdgeAdded { .. } => {}
                            GraphEvent::EdgeRemoved { .. } => {}
                        });
                        cx.notify(graph.entity_id());
                    }
                })
                .detach();

            let graph_offset = *graph.read(cx).offset();
            let pannable = cx.new(|_cx| Pannable::new("graph", graph_offset, graph_view.clone()));

            cx.subscribe(&pannable, |editor: &mut Self, _pannable, event: &PannableEvent, cx| {
                match event {
                    PannableEvent::OffsetChanged(position) => {
                        editor.visual_graph_offset = *position;
                    }
                    PannableEvent::OffsetCommitted(position) => {
                        editor.graph().update(cx, |graph, cx| {
                            graph.set_offset(*position);
                            cx.notify();
                        });
                    }
                }
            })
            .detach();

            Self {
                graph_view: pannable,
                new_node_menu_view: None,
                graph,
                visual_graph_offset: graph_offset,
                focus_handle: cx.focus_handle(),
            }
        })
    }

    pub fn graph(&self) -> Entity<crate::Graph<D>> {
        self.graph.clone()
    }

    pub fn close_new_node_menu(&mut self, cx: &mut Context<Self>) {
        self.new_node_menu_view = None;
        cx.notify();
    }
}

impl<D: GraphDef + 'static> GraphEditorView<D> {
    fn handle_open_new_node_menu(
        &mut self,
        _: &OpenNewNodeMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // TODO: Account for editor bounds origin.
        let position = window.mouse_position();
        let editor_view = cx.entity().clone();
        self.new_node_menu_view =
            Some(cx.new(|cx| NewNodeMenuView::new(position, editor_view, window, cx)));
        cx.notify();
    }

    fn handle_close_new_node_menu(
        &mut self,
        _: &CloseNewNodeMenu,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.close_new_node_menu(cx);
    }
}

impl<D: GraphDef + 'static> Render for GraphEditorView<D> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let NodeMeasurements { snap_size, .. } = NodeMeasurements::new(window);

        let grid = ui::scrollable_line_grid(
            &self.visual_graph_offset,
            snap_size,
            cx.theme().line_grid_color,
        )
        .size_full();

        let focused = self.focus_handle.is_focused(window);

        z_stack([
            grid.into_any_element(),
            self.graph_view.clone().into_any_element(),
            self.new_node_menu_view
                .clone()
                .map(|e| e.into_any_element())
                .unwrap_or_else(|| cx.new(|_cx| EmptyView).into_any_element()),
        ])
        .track_focus(&self.focus_handle)
        .key_context(KEY_CONTEXT)
        .relative()
        .size_full()
        .overflow_hidden()
        .when(focused, |e| {
            e.on_action(cx.listener(Self::handle_open_new_node_menu))
                .on_action(cx.listener(Self::handle_close_new_node_menu))
        })
    }
}

impl<D: GraphDef + 'static> Focusable for GraphEditorView<D> {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

pub struct NewNodeMenuView<D: GraphDef> {
    position: Point<Pixels>,
    editor_view: Entity<GraphEditorView<D>>,
    search_field: Entity<TextField>,
}

impl<D: GraphDef + 'static> NewNodeMenuView<D> {
    pub fn new(
        position: Point<Pixels>,
        editor_view: Entity<GraphEditorView<D>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let search_field = cx.new(|cx| {
            let field = TextField::new("search_field", window, cx);
            field.set_placeholder("Search...".into(), cx);
            field
        });

        Self { position, editor_view, search_field }
    }

    pub fn close(&self, cx: &mut App) {
        self.editor_view.update(cx, |editor, cx| editor.close_new_node_menu(cx));
    }
}

impl<D: GraphDef + 'static> Render for NewNodeMenuView<D> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let search_pattern = self.search_field.read(cx).value(cx).to_ascii_lowercase();

        let graph = self.editor_view.read(cx).graph().clone();

        let templates = graph
            .read(cx)
            .templates()
            .filter(|template| {
                template.label().to_ascii_lowercase().replace(" ", "").contains(&search_pattern)
            })
            .cloned()
            .collect::<Vec<_>>();

        let header = div()
            .p_2()
            .border_b_1()
            .border_color(cx.theme().border)
            .child(self.search_field.clone());

        let list = uniform_list(
            cx.entity().clone(),
            "templates",
            templates.len(),
            move |_menu, range, _window, cx| {
                let mut children = Vec::new();

                for ix in range {
                    let template = &templates[ix];
                    let label = template.label().to_owned();

                    let child = div().child(
                        div()
                            .p_1()
                            .hover(|e| {
                                e.bg(cx.theme().element_background_hover)
                                    .border_1()
                                    .border_color(cx.theme().border_muted)
                            })
                            .rounded(cx.theme().radius)
                            .cursor_pointer()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener({
                                    let template = template.clone();
                                    move |menu, _, _window, cx| {
                                        let position = menu.position;
                                        menu.editor_view.read(cx).graph().update(
                                            cx,
                                            |graph, cx| {
                                                let node = Node::new(&template);
                                                graph.add_node(node, position, cx);
                                            },
                                        );
                                        menu.close(cx);
                                    }
                                }),
                            )
                            .child(label),
                    );

                    children.push(child);
                }

                children
            },
        )
        .p_2()
        .size_full();

        div()
            .absolute()
            .left(self.position.x)
            .top(self.position.y)
            .bg(cx.theme().element_background)
            .border_1()
            .border_color(cx.theme().border)
            .rounded(cx.theme().radius)
            .w_80()
            .h_64()
            .child(header)
            .child(list)
    }
}
