use flow::{
    Graph, GraphDef,
    frontend::{Frontend, GraphEvent},
};

pub mod editor;
mod graph;
mod node;
pub mod state;

pub use editor::*;
pub use state::*;

/// A wrapper around the GPUI `Context`.
pub struct GpuiFrontend<'a, 'cx, D: GraphDef + 'static> {
    cx: &'a mut gpui::Context<'cx, EventEmittingGraph<D>>,
}

impl<'cx, D: GraphDef + 'static> Frontend for GpuiFrontend<'_, 'cx, D> {
    fn emit_event(&mut self, event: GraphEvent) {
        self.cx.emit(event);
    }
}

impl<'a, 'cx, D: GraphDef + 'static> From<&'a mut gpui::Context<'cx, EventEmittingGraph<D>>>
    for GpuiFrontend<'a, 'cx, D>
{
    fn from(cx: &'a mut gpui::Context<'cx, EventEmittingGraph<D>>) -> Self {
        Self { cx }
    }
}

/// A wrapper around a `Graph` that can emit GPUI events.
pub struct EventEmittingGraph<D: GraphDef> {
    graph: Graph<D>,
}

impl<D: GraphDef> EventEmittingGraph<D> {
    pub fn new(graph: Graph<D>) -> Self {
        Self { graph }
    }
}

impl<D: GraphDef> std::ops::Deref for EventEmittingGraph<D> {
    type Target = Graph<D>;

    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

impl<D: GraphDef> std::ops::DerefMut for EventEmittingGraph<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.graph
    }
}

impl<D> gpui::EventEmitter<GraphEvent> for EventEmittingGraph<D> where D: GraphDef + 'static {}
