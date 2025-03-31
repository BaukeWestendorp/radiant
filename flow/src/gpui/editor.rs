use super::{
    GraphEvent,
    graph::{GraphView, VisualGraphEvent},
    node::NodeMeasurements,
};
use crate::{AnySocket, Graph, GraphDef};
use gpui::*;
use new_node_menu::NewNodeMenuView;
use prelude::FluentBuilder;
use ui::{Pannable, PannableEvent, theme::ActiveTheme, z_stack};

mod new_node_menu;

const KEY_CONTEXT: &str = "GraphEditor";

actions!(graph_editor, [OpenNewNodeMenu, CloseNewNodeMenu]);

pub fn init(cx: &mut App) {
    new_node_menu::init(cx);

    cx.bind_keys([
        KeyBinding::new("space", OpenNewNodeMenu, Some(KEY_CONTEXT)),
        KeyBinding::new("escape", CloseNewNodeMenu, Some(KEY_CONTEXT)),
    ]);
}

pub struct GraphEditorView<D: GraphDef> {
    graph_view: Entity<Pannable>,
    new_node_menu_view: Option<Entity<NewNodeMenuView<D>>>,

    graph: Entity<Graph<D>>,
    visual_graph_offset: Point<Pixels>,

    focus_handle: FocusHandle,

    selection_corners: Option<(Point<Pixels>, Point<Pixels>)>,
}

impl<D: GraphDef + 'static> GraphEditorView<D> {
    pub fn build(graph: Entity<Graph<D>>, window: &mut Window, cx: &mut App) -> Entity<Self> {
        let graph_view = GraphView::build(graph.clone(), window, cx);

        let editor_view = cx.new(|cx| {
            cx.subscribe(&graph, |_editor, _graph_view, event: &GraphEvent, cx| {
                cx.emit(event.clone())
            })
            .detach();

            cx.subscribe(&graph_view, |_editor, _graph_view, event: &VisualGraphEvent, cx| {
                cx.emit(event.clone())
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
                selection_corners: None,
            }
        });

        window
            .subscribe(&editor_view, cx, {
                move |_editor, event: &GraphEvent, window, cx| match event {
                    GraphEvent::NodeAdded(node_id) => {
                        graph_view.update(cx, |graph, cx| {
                            graph.add_node(*node_id, window, cx);
                        });
                    }
                    GraphEvent::NodeRemoved(node_id) => {
                        graph_view.update(cx, |graph, cx| {
                            graph.remove_node(node_id, cx);
                        });
                    }

                    _ => {}
                }
            })
            .detach();

        window
            .subscribe(&editor_view, cx, {
                move |editor, event: &VisualGraphEvent, window, cx| match event {
                    VisualGraphEvent::EdgeSourceRequested { target } => {
                        editor.update(cx, |editor, cx| {
                            editor.open_new_node_menu(
                                Some(AnySocket::Input(target.clone())),
                                window,
                                cx,
                            );
                        });
                    }
                    VisualGraphEvent::EdgeTargetRequested { source } => {
                        editor.update(cx, |editor, cx| {
                            editor.open_new_node_menu(
                                Some(AnySocket::Output(source.clone())),
                                window,
                                cx,
                            );
                        });
                    }
                }
            })
            .detach();

        editor_view
    }

    pub fn graph(&self) -> Entity<Graph<D>> {
        self.graph.clone()
    }

    pub fn open_new_node_menu(
        &mut self,
        edge_start: Option<AnySocket>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // TODO: Account for editor bounds origin.
        let position = window.mouse_position();
        let editor_view = cx.entity().clone();
        self.new_node_menu_view = Some(cx.new(|cx| {
            NewNodeMenuView::new(
                position,
                edge_start,
                self.graph.read(cx).clone(),
                editor_view,
                window,
                cx,
            )
        }));
        cx.notify();
    }

    pub fn close_new_node_menu(&mut self, cx: &mut Context<Self>) {
        self.new_node_menu_view = None;
        cx.notify();
    }

    pub fn selection_bounds(&self) -> Option<Bounds<Pixels>> {
        let (a, b) = self.selection_corners?;
        Some(Bounds::from_corners(
            Point::new(a.x.min(b.x), a.y.min(b.y)),
            Point::new(a.x.max(b.x), a.y.max(b.y)),
        ))
    }
}

impl<D: GraphDef + 'static> GraphEditorView<D> {
    fn handle_open_new_node_menu(
        &mut self,
        _: &OpenNewNodeMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_new_node_menu(None, window, cx);
    }

    fn handle_close_new_node_menu(
        &mut self,
        _: &CloseNewNodeMenu,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.close_new_node_menu(cx);
    }

    fn handle_mouse_down_right(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selection_corners = Some((event.position, event.position));
        cx.notify();
    }

    fn handle_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(rect_selection) = &mut self.selection_corners {
            rect_selection.1 = event.position;
        }

        if let Some(bounds) = self.selection_bounds() {
            let graph = self.graph().read(cx);

            let selected_nodes = graph
                .node_ids()
                .filter(|id| bounds.intersects(&(graph.node_bounds(id) + self.visual_graph_offset)))
                .copied()
                .collect::<Vec<_>>();

            self.graph().update(cx, |graph, _cx| {
                for node_id in selected_nodes {
                    graph.select_node(node_id);
                }
            });
        }

        cx.notify();
    }

    fn handle_mouse_up_right(
        &mut self,
        _: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selection_corners = None;
        cx.notify();
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

        let rect_selection = match &self.selection_bounds() {
            Some(bounds) => div()
                .absolute()
                .left(bounds.origin.x)
                .top(bounds.origin.y)
                .w(bounds.size.width)
                .h(bounds.size.height)
                .border_1()
                .border_color(cx.theme().border_selected)
                .bg(cx.theme().element_background_selected),
            None => div(),
        };

        z_stack([
            grid.into_any_element(),
            self.graph_view.clone().into_any_element(),
            rect_selection.into_any_element(),
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
        .on_mouse_down(MouseButton::Right, cx.listener(Self::handle_mouse_down_right))
        .on_mouse_move(cx.listener(Self::handle_mouse_move))
        .on_mouse_up(MouseButton::Right, cx.listener(Self::handle_mouse_up_right))
        .on_mouse_up_out(MouseButton::Right, cx.listener(Self::handle_mouse_up_right))
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

impl<D: GraphDef + 'static> EventEmitter<GraphEvent> for GraphEditorView<D> {}

impl<D: GraphDef + 'static> EventEmitter<VisualGraphEvent> for GraphEditorView<D> {}
