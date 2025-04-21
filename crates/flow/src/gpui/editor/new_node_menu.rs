use super::GraphEditorView;
use crate::{AnySocket, DataType, Graph, GraphDef, InputSocket, Node, OutputSocket, TemplateId};
use gpui::*;
use prelude::FluentBuilder as _;
use ui::{ActiveTheme as _, InteractiveColor as _, TextField, TextInputEvent};

pub mod actions {
    use gpui::{App, KeyBinding, actions};

    pub const KEY_CONTEXT: &str = "NewNodeMenu";

    actions!(new_node_menu, [SelectNextItem, SelectPreviousItem]);

    pub fn init(cx: &mut App) {
        bind_keys(cx);
    }

    fn bind_keys(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("up", SelectPreviousItem, Some(KEY_CONTEXT)),
            KeyBinding::new("down", SelectNextItem, Some(KEY_CONTEXT)),
        ]);
    }
}

pub struct NewNodeMenuView<D: GraphDef> {
    position: Point<Pixels>,
    editor_view: Entity<GraphEditorView<D>>,
    search_field: Entity<TextField>,
    focus_handle: FocusHandle,
    items: Vec<NewNodeMenuItem<D>>,
    selected_item_ix: Option<usize>,
    edge_start: Option<AnySocket>,
}

impl<D: GraphDef + 'static> NewNodeMenuView<D> {
    pub fn new(
        position: Point<Pixels>,
        edge_start: Option<AnySocket>,
        graph: Graph<D>,
        editor_view: Entity<GraphEditorView<D>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window);

        let search_field = cx.new(|cx| {
            let field = TextField::new("search_field", focus_handle.clone(), window, cx);
            field.set_placeholder("Search...".into(), cx);
            field
        });

        let items =
            get_filtered_items(edge_start.as_ref(), search_field.read(cx).value(cx), &graph);
        let selected_item_ix = if items.is_empty() { None } else { Some(0) };

        let menu = cx.entity().clone();
        window
            .subscribe(&search_field, cx, {
                let required_socket = edge_start.clone();
                move |_text_field, event, window, cx| match event {
                    TextInputEvent::Change(value) => {
                        let items = get_filtered_items(required_socket.as_ref(), value, &graph);
                        menu.update(cx, |menu, _cx| {
                            menu.selected_item_ix = if items.is_empty() { None } else { Some(0) };
                            menu.items = items;
                        });
                    }
                    TextInputEvent::Submit => menu.update(cx, |menu, cx| {
                        if let Some(ix) = menu.selected_item_ix {
                            menu.create_node(ix, window, cx);
                        }
                    }),
                    _ => {}
                }
            })
            .detach();

        Self {
            position,
            editor_view,
            search_field,
            focus_handle,
            items,
            selected_item_ix,
            edge_start,
        }
    }

    pub fn close(&self, window: &mut Window, cx: &mut Context<Self>) {
        cx.focus_view(&self.editor_view, window);
        self.editor_view.update(cx, |editor, cx| {
            editor.close_new_node_menu(cx);
        });
    }

    pub fn create_node(&mut self, item_ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        let graph = self.editor_view.read(cx).graph();
        let graph_offset = *graph.read(cx).offset();
        let position = self.position - graph_offset;
        graph.update(cx, |graph, cx| {
            let item = &self.items[item_ix];
            let template_id = &item.template_id;
            let template = graph.template(template_id);
            let node = Node::new(template);
            let node_id = graph.add_node(node, position, cx);

            match &self.edge_start {
                Some(AnySocket::Input(target)) => {
                    let source_id = &item
                        .socket_info
                        .as_ref()
                        .expect(
                            "should get socket_info from NewNodeMenuItem as it has an edge_start",
                        )
                        .socket_id;
                    let source = OutputSocket::new(node_id, source_id.clone());
                    graph.add_edge(target.clone(), source, cx);
                }
                Some(AnySocket::Output(source)) => {
                    let target_id = &item
                        .socket_info
                        .as_ref()
                        .expect(
                            "should get socket_info from NewNodeMenuItem as it has an edge_start",
                        )
                        .socket_id;
                    let target = InputSocket::new(node_id, target_id.clone());
                    graph.add_edge(target, source.clone(), cx);
                }

                None => {}
            }
        });
        self.close(window, cx);
    }
}

fn get_filtered_items<D: GraphDef + 'static>(
    edge_start: Option<&AnySocket>,
    search_field_text: &str,
    graph: &Graph<D>,
) -> Vec<NewNodeMenuItem<D>> {
    let normalize_search_text = |s: &str| s.to_ascii_lowercase().replace(" ", "");
    let search_pattern = normalize_search_text(search_field_text);

    let mut items = Vec::new();

    for template in graph.templates() {
        if !normalize_search_text(template.label()).contains(&search_pattern) {
            continue;
        }

        match &edge_start {
            Some(AnySocket::Input(input_socket)) => {
                let input = graph.input(input_socket);
                for output in template.outputs() {
                    if output.data_type().can_cast_to(&input.data_type()) {
                        items.push(NewNodeMenuItem {
                            template_id: template.id().clone(),
                            node_label: template.label().to_string().into(),
                            socket_info: Some(SocketInfo {
                                label: output.label().to_string().into(),
                                data_type: output.data_type().clone(),
                                socket_id: output.id().to_string(),
                            }),
                        });
                    }
                }
            }
            Some(AnySocket::Output(output_socket)) => {
                let output = graph.output(output_socket);
                for input in template.inputs() {
                    if input.data_type().can_cast_to(output.data_type()) {
                        items.push(NewNodeMenuItem {
                            template_id: template.id().clone(),
                            node_label: template.label().to_string().into(),
                            socket_info: Some(SocketInfo {
                                label: input.label().to_string().into(),
                                data_type: input.data_type().clone(),
                                socket_id: input.id().to_string(),
                            }),
                        });
                    }
                }
            }
            None => {
                items.push(NewNodeMenuItem {
                    template_id: template.id().clone(),
                    node_label: template.label().to_string().into(),
                    socket_info: None,
                });
            }
        }
    }

    items
}

impl<D: GraphDef + 'static> NewNodeMenuView<D> {
    fn handle_mouse_down_out(
        &mut self,
        _event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.close(window, cx);
    }

    fn handle_select_next_item(
        &mut self,
        _event: &actions::SelectNextItem,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.selected_item_ix {
            Some(ix) => {
                self.selected_item_ix = Some((ix + 1) % self.items.len());
            }
            None => {
                self.selected_item_ix = Some(0);
            }
        }
        cx.notify();
    }
    fn handle_select_previous_item(
        &mut self,
        _event: &actions::SelectPreviousItem,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.selected_item_ix {
            Some(ix) => {
                self.selected_item_ix = Some((ix + self.items.len() - 1) % self.items.len());
            }
            None => {
                self.selected_item_ix = Some(self.items.len() - 1);
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
            self.items.len(),
            move |menu, range, _window, cx| {
                let mut children = Vec::new();
                for ix in range {
                    let item = menu.items[ix].clone();
                    let selected = menu.selected_item_ix == Some(ix);

                    let child = div().child(
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
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
                                cx.listener(move |menu, _, window, cx| {
                                    menu.create_node(ix, window, cx);
                                }),
                            )
                            .child(div().flex().gap_2().child(item.node_label).when_some(
                                item.socket_info.clone(),
                                |e, socket_info| {
                                    e.child(
                                        div()
                                            .text_color(cx.theme().text_primary.muted())
                                            .child(socket_info.label),
                                    )
                                },
                            ))
                            .when_some(item.socket_info, |e, socket_info| {
                                e.child(
                                    div()
                                        .size_3()
                                        .bg(socket_info.data_type.color())
                                        .border_1()
                                        .border_color(black().opacity(0.5)),
                                )
                            }),
                    );

                    children.push(child);
                }

                children
            },
        )
        .p_2()
        .size_full();

        div()
            .key_context(actions::KEY_CONTEXT)
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

impl<D: GraphDef + 'static> Focusable for NewNodeMenuView<D> {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[derive(Clone)]
struct NewNodeMenuItem<D: GraphDef> {
    template_id: TemplateId,
    node_label: SharedString,
    socket_info: Option<SocketInfo<D>>,
}

#[derive(Clone)]
struct SocketInfo<D: GraphDef> {
    label: SharedString,
    data_type: D::DataType,
    socket_id: String,
}
