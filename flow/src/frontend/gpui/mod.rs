use super::GraphEvent;
use crate::{Graph, GraphDef};
use gpui::*;

pub use editor::*;
pub use graph::*;

pub mod editor;
pub mod graph;

impl<'cx, E> super::Frontend for Context<'cx, E>
where
    E: EventEmitter<GraphEvent>,
{
    fn emit_event(&mut self, event: GraphEvent) {
        self.emit(event);
    }
}

impl<D: GraphDef + 'static> EventEmitter<GraphEvent> for Graph<D> {}
