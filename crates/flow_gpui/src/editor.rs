use crate::graph::{GraphEvent, GraphView};
use crate::{NodeCategory, VisualControl, VisualDataType, VisualNodeData, VisualNodeKind};

use flow::{Graph, GraphDefinition};
use gpui::*;
use ui::input::TextField;
use ui::theme::ActiveTheme;
use ui::{bounds_updater, StyledExt};

actions!(graph_editor, [CloseNodeContextMenu]);

const CONTEXT: &str = "GraphEditor";

pub(crate) fn init(cx: &mut AppContext) {
    cx.bind_keys([KeyBinding::new(
        "escape",
        CloseNodeContextMenu,
        Some(CONTEXT),
    )]);
}

pub struct GraphEditorView<Def: GraphDefinition>
where
    Def::NodeKind: VisualNodeKind,
{
    graph_view: View<GraphView<Def>>,
    new_node_context_menu: View<NewNodeContextMenu<Def>>,
    graph_offset: Point<Pixels>,
    prev_mouse_pos: Option<Point<Pixels>>,

    focus_handle: FocusHandle,
    bounds: Bounds<Pixels>,
}

impl<Def: GraphDefinition + 'static> GraphEditorView<Def>
where
    Def::NodeData: VisualNodeData,
    Def::NodeKind: VisualNodeKind,
    Def::DataType: VisualDataType,
    Def::Control: VisualControl<Def>,
{
    pub fn build(graph: Model<Graph<Def>>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let graph_view = GraphView::build(graph.clone(), cx);
            let new_node_context_menu = NewNodeContextMenu::build(graph, cx);
            let focus_handle = cx.focus_handle().clone();

            Self {
                graph_view,
                new_node_context_menu,
                graph_offset: Point::default(),
                prev_mouse_pos: None,
                focus_handle,
                bounds: Bounds::default(),
            }
        })
    }

    fn open_node_context_menu(&mut self, cx: &mut ViewContext<Self>) {
        self.new_node_context_menu.update(cx, |menu, cx| {
            menu.show(cx);
            let position = cx.mouse_position() - self.bounds.origin;
            menu.set_position(position, cx);
        });
    }

    fn close_node_context_menu(&mut self, _: &CloseNodeContextMenu, cx: &mut ViewContext<Self>) {
        self.new_node_context_menu.update(cx, |menu, cx| {
            menu.hide(cx);
        });
    }

    fn handle_drag_move(&mut self, _: &DragMoveEvent<()>, cx: &mut ViewContext<Self>) {
        let diff = self
            .prev_mouse_pos
            .map_or(Point::default(), |prev| cx.mouse_position() - prev);

        self.graph_offset += diff;
        self.prev_mouse_pos = Some(cx.mouse_position());
    }

    fn handle_mouse_up(&mut self, _: &MouseUpEvent, _cx: &mut ViewContext<Self>) {
        self.prev_mouse_pos = None;
    }
}

impl<Def: GraphDefinition + 'static> Render for GraphEditorView<Def>
where
    Def::NodeData: VisualNodeData,
    Def::NodeKind: VisualNodeKind,
    Def::DataType: VisualDataType,
    Def::Control: VisualControl<Def>,
{
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let graph_viewer = div()
            .id("editor-graph")
            .size_full()
            .absolute()
            .overflow_hidden()
            .child(
                div()
                    .size_full()
                    .left(self.graph_offset.x)
                    .top(self.graph_offset.y)
                    .child(self.graph_view.clone()),
            )
            .on_drag((), |_, cx| cx.new_view(|_cx| EmptyView))
            .on_drag_move(cx.listener(Self::handle_drag_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::handle_mouse_up));

        div()
            .key_context(CONTEXT)
            .track_focus(&self.focus_handle)
            .size_full()
            .child(graph_viewer)
            .child(self.new_node_context_menu.clone())
            .child(bounds_updater(
                cx.view().clone(),
                |this: &mut Self, bounds, _cx| this.bounds = bounds,
            ))
            .on_action(cx.listener(Self::close_node_context_menu))
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(|this, _, cx| this.open_node_context_menu(cx)),
            )
    }
}

impl<Def: GraphDefinition + 'static> FocusableView for GraphEditorView<Def>
where
    Def::DataType: VisualDataType,
    Def::NodeKind: VisualNodeKind,
    Def::NodeData: VisualNodeData,
    Def::Control: VisualControl<Def>,
{
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

struct NewNodeContextMenu<Def: GraphDefinition>
where
    Def::NodeKind: VisualNodeKind,
{
    graph: Model<Graph<Def>>,
    shown: bool,
    position: Point<Pixels>,
    search_box: View<TextField>,
    selected_category: Option<<Def::NodeKind as VisualNodeKind>::Category>,
}

impl<Def: GraphDefinition + 'static> NewNodeContextMenu<Def>
where
    Def::NodeKind: VisualNodeKind,
    Def::NodeData: VisualNodeData,
{
    pub fn build(graph: Model<Graph<Def>>, cx: &mut WindowContext) -> View<Self> {
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
                selected_category: None,
            }
        })
    }

    pub fn show<View: 'static>(&mut self, cx: &mut ViewContext<View>) {
        self.shown = true;
        self.deselect_category(cx);
        self.search_box.update(cx, |search_box, cx| {
            search_box.focus(cx);
            search_box.clear(cx);
        });
        cx.stop_propagation();
        cx.notify();
    }

    pub fn hide<View: 'static>(&mut self, cx: &mut ViewContext<View>) {
        self.shown = false;
        cx.notify();
    }

    pub fn set_position<View: 'static>(
        &mut self,
        position: Point<Pixels>,
        cx: &mut ViewContext<View>,
    ) {
        self.position = position;
        cx.notify();
    }

    fn create_new_node(
        &self,
        node_kind: Def::NodeKind,
        mut data: Def::NodeData,
        cx: &mut WindowContext,
    ) {
        data.set_position(self.position.into());
        self.graph.update(cx, |_graph, cx| {
            cx.emit(GraphEvent::AddNode {
                kind: node_kind,
                data,
            });
        });
    }

    fn select_category(
        &mut self,
        category: <Def::NodeKind as VisualNodeKind>::Category,
        cx: &mut WindowContext,
    ) {
        self.search_box.update(cx, |search_box, _cx| {
            search_box.set_placeholder(format!("Search in '{}'", category.to_string()).into());
        });

        self.selected_category = Some(category);
    }

    fn deselect_category(&mut self, cx: &mut WindowContext) {
        self.search_box.update(cx, |search_box, _cx| {
            search_box.set_placeholder("Search".into());
        });

        self.selected_category = None;
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
        let nodes = Def::NodeKind::all()
            .filter(|n| {
                n.label().to_lowercase().contains(&filter.to_lowercase())
                    && match self.selected_category {
                        Some(category) => n.category() == category,
                        None => true,
                    }
            })
            .collect::<Vec<_>>();
        let categories = <Def::NodeKind as VisualNodeKind>::Category::all().collect::<Vec<_>>();

        let show_categories = filter.is_empty() && self.selected_category.is_none();

        let render_list_item = move |item: AnyElement, cx: &ViewContext<Self>| -> AnyElement {
            div()
                .p_1()
                .bg(cx.theme().primary)
                .hover(|style| style.bg(cx.theme().primary_hover))
                .border_b_1()
                .border_color(cx.theme().background)
                .cursor_pointer()
                .child(item)
                .into_any_element()
        };

        let item_count = if show_categories {
            categories.len()
        } else {
            nodes.len()
        };

        uniform_list(
            cx.view().clone(),
            "new_node_context_menu",
            item_count,
            move |_this, visible_range, cx| -> Vec<AnyElement> {
                visible_range
                    .into_iter()
                    .map(|ix| {
                        if show_categories {
                            let category = &categories[ix];
                            let label = format!("> {}", category.to_string());

                            let item = div().size_full().child(label).on_mouse_down(
                                MouseButton::Left,
                                cx.listener({
                                    let category = *category;
                                    move |this, _, cx| {
                                        cx.prevent_default();

                                        this.select_category(category, cx);
                                    }
                                }),
                            );
                            render_list_item(item.into_any_element(), cx)
                        } else {
                            let node_kind = &nodes[ix];
                            let label = node_kind.label().to_string();

                            let item = div().size_full().child(label).on_mouse_down(
                                MouseButton::Left,
                                cx.listener({
                                    let node_kind = node_kind.clone();
                                    move |this, _, cx| {
                                        cx.prevent_default();

                                        let data = <Def::NodeData as Default>::default();
                                        this.create_new_node(node_kind.clone(), data, cx);
                                        this.hide(cx);
                                    }
                                }),
                            );

                            render_list_item(item.into_any_element(), cx)
                        }
                    })
                    .collect()
            },
        )
        .w_full()
        .h_40()
    }
}

impl<Def: GraphDefinition + 'static> Render for NewNodeContextMenu<Def>
where
    Def::NodeKind: VisualNodeKind,
    Def::NodeData: VisualNodeData,
{
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
