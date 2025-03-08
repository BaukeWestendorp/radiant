use flow::{
    Graph, GraphDef, Node, NodeId,
    frontend::{Frontend, GraphEvent},
};

pub mod editor;
mod graph;
mod node;
pub mod state;

pub use editor::*;
pub use state::*;

/// A wrapper around the GPUI `Context`.
pub struct GpuiFrontend<'a, 'cx, D: GraphDef<State = GpuiGraphState> + 'static> {
    cx: &'a mut gpui::Context<'cx, GpuiGraph<D>>,
}

impl<'cx, D: GraphDef<State = GpuiGraphState> + 'static> Frontend for GpuiFrontend<'_, 'cx, D> {
    fn emit_event(&mut self, event: GraphEvent) {
        self.cx.emit(event);
    }
}

impl<'a, 'cx, D: GraphDef<State = GpuiGraphState> + 'static>
    From<&'a mut gpui::Context<'cx, GpuiGraph<D>>> for GpuiFrontend<'a, 'cx, D>
{
    fn from(cx: &'a mut gpui::Context<'cx, GpuiGraph<D>>) -> Self {
        Self { cx }
    }
}

/// A wrapper around a `Graph` that can emit GPUI events.
pub struct GpuiGraph<D: GraphDef<State = GpuiGraphState>> {
    graph: Graph<D>,
}

impl<D: GraphDef<State = GpuiGraphState>> GpuiGraph<D> {
    pub fn new(graph: Graph<D>) -> Self {
        Self { graph }
    }

    pub fn add_node(
        &mut self,
        node: Node<D>,
        position: NodePosition,
        frontend: &mut GpuiFrontend<D>,
    ) -> NodeId {
        let node_id = self.graph.add_node(node, frontend);
        self.graph.state_mut().set_node_position(node_id, position);
        node_id
    }
}

impl<D: GraphDef<State = GpuiGraphState> + 'static> gpui::EventEmitter<GraphEvent>
    for GpuiGraph<D>
{
}
