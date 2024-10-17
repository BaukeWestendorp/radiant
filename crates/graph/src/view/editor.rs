use crate::graph::{Graph, GraphEvent};
use crate::view::graph::GraphView;
use crate::NodeKind;
use gpui::*;
use strum::IntoEnumIterator;
use ui::input::TextField;
use ui::theme::ActiveTheme;
use ui::StyledExt;

actions!(graph_editor, [OpenNodeContextMenu, CloseNodeContextMenu]);

const CONTEXT: &str = "GraphEditor";

pub fn init(cx: &mut AppContext) {
    cx.bind_keys([
        KeyBinding::new("space", OpenNodeContextMenu, Some(CONTEXT)),
        KeyBinding::new("escape", CloseNodeContextMenu, Some(CONTEXT)),
    ]);
}

pub struct GraphEditorView {
    graph_view: View<GraphView>,
    new_node_context_menu: View<NewNodeContextMenu>,

    focus_handle: FocusHandle,
}

impl GraphEditorView {
    pub fn build(graph: Model<Graph>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let graph_view = GraphView::build(graph.clone(), cx);
            let new_node_context_menu = NewNodeContextMenu::build(graph, cx);
            let focus_handle = cx.focus_handle().clone();

            Self {
                graph_view,
                new_node_context_menu,
                focus_handle,
            }
        })
    }

    fn open_node_context_menu(&mut self, _: &OpenNodeContextMenu, cx: &mut ViewContext<Self>) {
        self.new_node_context_menu.update(cx, |menu, cx| {
            menu.show(cx);
        });
    }

    fn close_node_context_menu(&mut self, _: &CloseNodeContextMenu, cx: &mut ViewContext<Self>) {
        self.new_node_context_menu.update(cx, |menu, cx| {
            menu.hide(cx);
        });
    }

    fn render_header(&self, cx: &WindowContext) -> impl IntoElement {
        div()
            .h_flex()
            .px_2()
            .bg(cx.theme().secondary)
            .w_full()
            .h_8()
            .border_b_1()
            .border_color(cx.theme().border)
            .child("header")
    }
}

impl Render for GraphEditorView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .key_context(CONTEXT)
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::open_node_context_menu))
            .on_action(cx.listener(Self::close_node_context_menu))
            .size_full()
            .child(self.render_header(cx))
            .child(self.graph_view.clone())
            .child(self.new_node_context_menu.clone())
    }
}

impl FocusableView for GraphEditorView {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

struct NewNodeContextMenu {
    graph: Model<Graph>,
    shown: bool,
    position: Point<Pixels>,
    search_box: View<TextField>,
}

impl NewNodeContextMenu {
    pub fn build(graph: Model<Graph>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let search_box = cx.new_view(|cx| {
                let mut field = TextField::new(cx);
                field.set_placeholder("Search".into());
                field
            });

            Self {
                graph,
                shown: false,
                position: cx.mouse_position(),
                search_box,
            }
        })
    }

    pub fn show<V: 'static>(&mut self, cx: &mut ViewContext<V>) {
        self.shown = true;
        self.position = cx.mouse_position();
        self.search_box.focus_handle(cx).focus(cx);
        cx.notify();
    }

    pub fn hide<V: 'static>(&mut self, cx: &mut ViewContext<V>) {
        self.shown = false;
        cx.notify();
    }

    fn create_new_node(&self, node_kind: NodeKind, cx: &mut WindowContext) {
        let position = self.position;
        self.graph.update(cx, |_graph, cx| {
            cx.emit(GraphEvent::AddNode(node_kind, position));
        });
    }

    fn render_header(&self, cx: &AppContext) -> impl IntoElement {
        div()
            .h_flex()
            .gap_2()
            .w_full()
            .p_1()
            .border_b_1()
            .border_color(cx.theme().border)
            .child(self.search_box.clone())
    }

    fn render_node_list(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let filter = self.search_box.read(cx).value();
        let nodes = NodeKind::iter()
            .filter(|n| n.label().to_lowercase().contains(&filter.to_lowercase()))
            .collect::<Vec<_>>();
        let node_count = nodes.len();

        let render_list_item = move |ix, cx: &ViewContext<Self>| -> AnyElement {
            let node_kind: &NodeKind = &nodes[ix];
            let label = node_kind.label().to_string();

            div()
                .p_1()
                .bg(cx.theme().primary)
                .hover(|style| style.bg(cx.theme().primary_hover))
                .border_b_1()
                .border_color(cx.theme().background)
                .cursor_pointer()
                .child(label)
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener({
                        let node_kind = node_kind.clone();
                        move |this, _, cx| {
                            this.create_new_node(node_kind.clone(), cx);
                            this.hide(cx);
                        }
                    }),
                )
                .into_any_element()
        };

        uniform_list(
            cx.view().clone(),
            "new_node_context_menu",
            node_count,
            move |_this, visible_range, cx| -> Vec<AnyElement> {
                visible_range
                    .into_iter()
                    .map(|ix| render_list_item(ix, cx))
                    .collect()
            },
        )
        .w_full()
        .h_40()
    }
}

impl Render for NewNodeContextMenu {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        if !self.shown {
            return div();
        }

        div()
            .absolute()
            .w(px(300.0))
            .left(self.position.x)
            .top(self.position.y)
            .bg(cx.theme().secondary)
            .border_1()
            .border_color(cx.theme().border)
            .rounded(cx.theme().radius)
            .child(self.render_header(cx))
            .child(self.render_node_list(cx))
            .on_mouse_down_out(cx.listener(|this, _, cx| {
                this.hide(cx);
            }))
    }
}
