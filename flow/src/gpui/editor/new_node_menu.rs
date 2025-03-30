use gpui::*;
use prelude::FluentBuilder as _;
use ui::{
    TextField, TextInputEvent,
    theme::{ActiveTheme as _, InteractiveColor as _},
};

use crate::{AnySocket, DataType, Graph, GraphDef, Node, Template};

use super::GraphEditorView;

const KEY_CONTEXT: &str = "NewNodeMenu";

actions!(new_node_menu, [SelectNextItem, SelectPreviousItem]);

pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("up", SelectPreviousItem, Some(KEY_CONTEXT)),
        KeyBinding::new("down", SelectNextItem, Some(KEY_CONTEXT)),
    ]);
}

pub struct NewNodeMenuView<D: GraphDef> {
    position: Point<Pixels>,
    editor_view: Entity<GraphEditorView<D>>,
    search_field: Entity<TextField>,
    templates: Vec<Template<D>>,
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
        let search_field = cx.new(|cx| {
            let field = TextField::new("search_field", cx.focus_handle(), window, cx);
            field.set_placeholder("Search...".into(), cx);
            field
        });

        cx.focus_view(&search_field, window);

        let templates =
            get_filtered_templates(edge_start.as_ref(), search_field.read(cx).value(cx), &graph);
        let selected_item_ix = if templates.is_empty() { None } else { Some(0) };

        cx.subscribe(&search_field, {
            let required_socket = edge_start.clone();
            move |menu, _search_field, event, cx| match event {
                TextInputEvent::Change(value) => {
                    let templates = get_filtered_templates(required_socket.as_ref(), value, &graph);
                    menu.selected_item_ix = if templates.is_empty() { None } else { Some(0) };
                    menu.templates = templates;
                }
                TextInputEvent::Submit => {
                    if let Some(ix) = menu.selected_item_ix {
                        menu.create_node(ix, cx);
                    }
                }
                _ => {}
            }
        })
        .detach();

        Self { position, editor_view, search_field, templates, selected_item_ix, edge_start }
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
    edge_start: Option<&AnySocket>,
    search_field_text: &str,
    graph: &Graph<D>,
) -> Vec<Template<D>> {
    let normalize_search_text = |s: &str| s.to_ascii_lowercase().replace(" ", "");
    let search_pattern = normalize_search_text(search_field_text);

    graph
        .templates()
        .filter(|template| {
            let has_name = normalize_search_text(template.label()).contains(&search_pattern);

            let has_socket = match edge_start {
                Some(AnySocket::Input(input_socket)) => {
                    let input = graph.input(input_socket);
                    template.outputs().iter().any(|o| o.data_type().can_cast_to(&input.data_type()))
                }
                Some(AnySocket::Output(output_socket)) => {
                    let output = graph.output(output_socket);
                    template.inputs().iter().any(|i| i.data_type().can_cast_to(&output.data_type()))
                }
                None => true,
            };

            has_name && has_socket
        })
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
            .key_context(KEY_CONTEXT)
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
