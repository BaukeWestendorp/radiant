use super::{GraphEvent, graph::GraphView, node::NodeMeasurements};
use crate::{Graph, GraphDef, Node, Template};
use gpui::*;
use prelude::FluentBuilder;
use ui::{
    Pannable, PannableEvent, TextField, TextInputEvent,
    theme::{ActiveTheme, InteractiveColor},
    z_stack,
};

const EDITOR_KEY_CONTEXT: &str = "GraphEditor";
const NEW_NODE_MENU_KEY_CONTEXT: &str = "NewNodeMenu";

actions!(graph_editor, [OpenNewNodeMenu, CloseNewNodeMenu]);
actions!(new_node_menu, [SelectNextItem, SelectPreviousItem]);

pub fn init(app: &mut App) {
    app.bind_keys([
        KeyBinding::new("space", OpenNewNodeMenu, Some(EDITOR_KEY_CONTEXT)),
        KeyBinding::new("escape", CloseNewNodeMenu, Some(EDITOR_KEY_CONTEXT)),
    ]);

    app.bind_keys([
        KeyBinding::new("up", SelectPreviousItem, Some(NEW_NODE_MENU_KEY_CONTEXT)),
        KeyBinding::new("down", SelectNextItem, Some(NEW_NODE_MENU_KEY_CONTEXT)),
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

    pub fn open_new_node_menu(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // TODO: Account for editor bounds origin.
        let position = window.mouse_position();
        let editor_view = cx.entity().clone();
        self.new_node_menu_view = Some(cx.new(|cx| {
            NewNodeMenuView::new(position, self.graph.read(cx).clone(), editor_view, window, cx)
        }));
        cx.notify();
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
        self.open_new_node_menu(window, cx);
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
        .key_context(EDITOR_KEY_CONTEXT)
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
    templates: Vec<Template<D>>,
    selected_item_ix: Option<usize>,
}

impl<D: GraphDef + 'static> NewNodeMenuView<D> {
    pub fn new(
        position: Point<Pixels>,
        graph: Graph<D>,
        editor_view: Entity<GraphEditorView<D>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let search_field = cx.new(|cx| {
            let field = TextField::new("search_field", cx.focus_handle(), window, cx);
            field.set_placeholder("Search...".into(), cx);
            field
        });

        cx.focus_view(&search_field, window);

        let templates = get_filtered_templates(&graph, search_field.read(cx).value(cx));
        let selected_item_ix = if templates.is_empty() { None } else { Some(0) };

        cx.subscribe(&search_field, move |menu, _search_field, event, cx| match event {
            TextInputEvent::Change(value) => {
                let templates = get_filtered_templates(&graph, value);
                menu.selected_item_ix = if templates.is_empty() { None } else { Some(0) };
                menu.templates = templates;
            }
            TextInputEvent::Submit => {
                if let Some(ix) = menu.selected_item_ix {
                    menu.create_node(ix, cx);
                }
            }
            _ => {}
        })
        .detach();

        Self { position, editor_view, search_field, templates, selected_item_ix }
    }

    pub fn close(&self, cx: &mut App) {
        self.editor_view.update(cx, |editor, cx| editor.close_new_node_menu(cx));
    }

    pub fn create_node(&mut self, ix: usize, cx: &mut Context<Self>) {
        let graph_offset = *self.editor_view.read(cx).graph().read(cx).offset();
        let position = self.position - graph_offset;
        self.editor_view.read(cx).graph().update(cx, |graph, cx| {
            let template = &self.templates[ix];
            let node = Node::new(template);
            graph.add_node(node, position, cx);
        });
        self.close(cx);
    }
}

fn get_filtered_templates<D: GraphDef + 'static>(
    graph: &Graph<D>,
    search_field_text: &str,
) -> Vec<Template<D>> {
    let normalize_search_text = |s: &str| s.to_ascii_lowercase().replace(" ", "");
    let search_pattern = normalize_search_text(search_field_text);

    graph
        .templates()
        .filter(|template| normalize_search_text(template.label()).contains(&search_pattern))
        .cloned()
        .collect::<Vec<_>>()
}

impl<D: GraphDef + 'static> NewNodeMenuView<D> {
    fn handle_mouse_down_out(
        &mut self,
        _event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.close(cx);
    }

    fn handle_select_next_item(
        &mut self,
        _event: &SelectNextItem,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.selected_item_ix {
            Some(ix) => {
                self.selected_item_ix = Some((ix + 1) % self.templates.len());
            }
            None => {
                self.selected_item_ix = Some(0);
            }
        }
        cx.notify();
    }
    fn handle_select_previous_item(
        &mut self,
        _event: &SelectPreviousItem,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.selected_item_ix {
            Some(ix) => {
                self.selected_item_ix =
                    Some((ix + self.templates.len() - 1) % self.templates.len());
            }
            None => {
                self.selected_item_ix = Some(self.templates.len() - 1);
            }
        }
        cx.notify();
    }
}

impl<D: GraphDef + 'static> Render for NewNodeMenuView<D> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let header = div()
            .p_2()
            .border_b_1()
            .border_color(cx.theme().border)
            .child(self.search_field.clone());

        let list = uniform_list(
            cx.entity().clone(),
            "templates",
            self.templates.len(),
            move |menu, range, _window, cx| {
                let mut children = Vec::new();

                for ix in range {
                    let template = &menu.templates[ix];
                    let label = template.label().to_owned();
                    let selected = menu.selected_item_ix == Some(ix);

                    let child = div().child(
                        div()
                            .px_1()
                            .border_1()
                            .hover(|e| {
                                let bg = if selected {
                                    cx.theme().element_background_selected
                                } else {
                                    cx.theme().element_background
                                };

                                let border_color = if selected {
                                    cx.theme().border_selected
                                } else {
                                    cx.theme().border.muted()
                                };

                                e.bg(bg.hovered()).border_color(border_color)
                            })
                            .when(selected, |e| {
                                e.bg(cx.theme().element_background_selected)
                                    .border_color(cx.theme().border_selected)
                            })
                            .rounded(cx.theme().radius)
                            .cursor_pointer()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |menu, _, _window, cx| {
                                    menu.create_node(ix, cx);
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
            .key_context(NEW_NODE_MENU_KEY_CONTEXT)
            .absolute()
            .w_80()
            .h_64()
            .left(self.position.x)
            .top(self.position.y)
            .bg(cx.theme().element_background)
            .border_1()
            .border_color(cx.theme().border)
            .rounded(cx.theme().radius)
            .block_mouse_down()
            .on_mouse_down_out(cx.listener(Self::handle_mouse_down_out))
            .on_action(cx.listener(Self::handle_select_next_item))
            .on_action(cx.listener(Self::handle_select_previous_item))
            .child(header)
            .child(list)
    }
}
